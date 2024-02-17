use crate::proto;
use libc::size_t;
use prost::Message;
use std::ffi::{c_char, c_uchar, c_void, CStr};
use std::marker::PhantomData;

extern "C" {
    fn cp_sat_wrapper_solve(
        model_buf: *const c_uchar,
        model_size: size_t,
        out_size: *mut size_t,
    ) -> *mut c_uchar;
    fn cp_sat_wrapper_solve_with_parameters(
        model_buf: *const c_uchar,
        model_size: size_t,
        params_buf: *const c_uchar,
        params_size: size_t,
        out_size: *mut size_t,
    ) -> *mut c_uchar;
    fn cp_sat_wrapper_cp_model_stats(
        model_buf: *const c_uchar,
        model_size: size_t) -> *mut c_char;
    fn cp_sat_wrapper_cp_solver_response_stats(
        response_buf: *const c_uchar,
        response_size: size_t,
        has_objective: bool,
    ) -> *mut c_char;
    fn cp_sat_wrapper_validate_cp_model(
        model_buf: *const c_uchar, model_size: size_t) -> *mut c_char;
    fn cp_sat_wrapper_solution_is_feasible(
        model_buf: *const c_uchar,
        model_size: size_t,
        solution_buf: *const i64,
        solution_size: size_t,
    ) -> bool;
}

extern "C-unwind" {
    fn cp_sat_wrapper_solve_with_parameters_and_observer(
        model_buf: *const c_uchar,
        model_size: size_t,
        params_buf: *const c_uchar,
        params_size: size_t,
        out_size: *mut size_t,
        callback: unsafe extern "C-unwind" fn(*mut c_void, *const c_uchar, size_t) -> bool,
        cb_data: *mut c_void) -> *mut c_uchar;
}

fn decode_resp<T: prost::Message + Default>(buffer: *const u8, buf_size: usize) -> Result<T, String> {
    let out_slice = unsafe { std::slice::from_raw_parts(buffer, buf_size) };
    let response = T::decode(out_slice)
        .map_err(|e| format!("Encountered error when decoding proto message: {}", e))?;
    unsafe { libc::free(buffer as _); }
    Ok(response)
}

struct CCallback<'closure> {
    pub function: unsafe extern "C-unwind" fn(*mut c_void, *const c_uchar, size_t) -> bool,
    pub user_data: *mut c_void,
    _lifetime: PhantomData<&'closure mut c_void>,
}

impl<'closure> CCallback<'closure> {
    pub fn new<F>(closure: &'closure mut F) -> Self where F: FnMut(*const c_uchar, size_t) -> bool {
        let function: unsafe extern "C-unwind" fn(*mut c_void, *const c_uchar, size_t) -> bool = Self::call_closure::<F>;

        debug_assert_eq!(std::mem::size_of::<&'closure mut F>(), std::mem::size_of::<*const c_void>());
        debug_assert_eq!(std::mem::size_of_val(&function), std::mem::size_of::<*const c_void>());

        Self {
            function,
            user_data: closure as *mut F as *mut c_void,
            _lifetime: PhantomData,
        }
    }

    unsafe extern "C-unwind" fn call_closure<F>(
        user_data: *mut c_void, buf: *const c_uchar, buf_size: size_t)
            -> bool where F : FnMut(*const c_uchar, size_t) -> bool {
        let cb: &mut F = user_data.cast::<F>().as_mut().unwrap();
        (*cb)(buf, buf_size)
    }
}

/// Solves the given [CpModelProto][crate::proto::CpModelProto] and
/// returns an instance of
/// [CpSolverResponse][crate::proto::CpSolverResponse].
pub fn solve(model: &proto::CpModelProto) -> proto::CpSolverResponse {
    let mut buf = Vec::default();
    model.encode(&mut buf).unwrap();
    let mut out_size = 0;
    let res = unsafe { cp_sat_wrapper_solve(buf.as_ptr(), buf.len(), &mut out_size) };
    let out_slice = unsafe { std::slice::from_raw_parts(res, out_size) };
    let response = proto::CpSolverResponse::decode(out_slice).unwrap();
    unsafe { libc::free(res as _) };
    response
}

/// Solves the given [CpModelProto][crate::proto::CpModelProto] registering the given `observer`
/// to the solver that will be called for every feasible solution.
/// Returns an instance of
/// [CpSolverResponse][crate::proto::CpSolverResponse].
pub fn solve_with_parameters_and_observer<F>(
    model: &proto::CpModelProto,
    mut observer: F,
    params: Option<&proto::SatParameters>) -> proto::CpSolverResponse
where F: FnMut(proto::CpSolverResponse) -> bool {
    let mut buf = Vec::default();
    model.encode(&mut buf).unwrap();
    let params_buf = params.map(|p| {
        let mut params_buf = Vec::default();
        p.encode(&mut params_buf).unwrap();
        params_buf
    });

    let callback = &mut
        |buf, buf_size| -> bool {
            match decode_resp(buf, buf_size) {
                Ok(res) => observer(res),
                Err(e) => {
                    log::error!("Error {}", e);
                    false
                }
            }
        };
    let c = CCallback::new(callback);

    let mut out_size = 0;
    let params_size = params_buf.as_ref().map(|p| p.len()).unwrap_or(0);
    let params_p = params_buf.as_ref().map(|p| p.as_ptr()).unwrap_or(std::ptr::null());
    let res = unsafe {
        cp_sat_wrapper_solve_with_parameters_and_observer(
        buf.as_ptr(), buf.len(),
        params_p, params_size,
        &mut out_size, c.function, c.user_data) };
    decode_resp(res, out_size).unwrap()
}

/// Solves the given [CpModelProto][crate::proto::CpModelProto] with
/// the given parameters.
pub fn solve_with_parameters(
    model: &proto::CpModelProto,
    params: &proto::SatParameters,
) -> proto::CpSolverResponse {
    let mut model_buf = Vec::default();
    model.encode(&mut model_buf).unwrap();
    let mut params_buf = Vec::default();
    params.encode(&mut params_buf).unwrap();

    let mut out_size = 0;
    let res = unsafe {
        cp_sat_wrapper_solve_with_parameters(
            model_buf.as_ptr(),
            model_buf.len(),
            params_buf.as_ptr(),
            params_buf.len(),
            &mut out_size,
        )
    };
    decode_resp(res, out_size).unwrap()
}

/// Returns a string with some statistics on the given
/// [CpModelProto][crate::proto::CpModelProto].
pub fn cp_model_stats(model: &proto::CpModelProto) -> String {
    let mut model_buf = Vec::default();
    model.encode(&mut model_buf).unwrap();
    let char_ptr = unsafe { cp_sat_wrapper_cp_model_stats(model_buf.as_ptr(), model_buf.len()) };
    let res = unsafe { CStr::from_ptr(char_ptr) }
        .to_str()
        .unwrap()
        .to_owned();
    unsafe { libc::free(char_ptr as _) };
    res
}

/// Returns a string with some statistics on the solver response.
///
/// If the second argument is false, we will just display NA for the
/// objective value instead of zero. It is not really needed but it
/// makes things a bit clearer to see that there is no objective.
pub fn cp_solver_response_stats(response: &proto::CpSolverResponse, has_objective: bool) -> String {
    let mut response_buf = Vec::default();
    response.encode(&mut response_buf).unwrap();
    let char_ptr = unsafe {
        cp_sat_wrapper_cp_solver_response_stats(
            response_buf.as_ptr(),
            response_buf.len(),
            has_objective,
        )
    };
    let res = unsafe { CStr::from_ptr(char_ptr) }
        .to_str()
        .unwrap()
        .to_owned();
    unsafe { libc::free(char_ptr as _) };
    res
}

/// Verifies that the given model satisfies all the properties
/// described in the proto comments. Returns an empty string if it is
/// the case, otherwise fails at the first error and returns a
/// human-readable description of the issue.
pub fn validate_cp_model(model: &proto::CpModelProto) -> String {
    let mut model_buf = Vec::default();
    model.encode(&mut model_buf).unwrap();
    let char_ptr = unsafe { cp_sat_wrapper_validate_cp_model(model_buf.as_ptr(), model_buf.len()) };
    let res = unsafe { CStr::from_ptr(char_ptr) }
        .to_str()
        .unwrap()
        .to_owned();
    unsafe { libc::free(char_ptr as _) };
    res
}

/// Verifies that the given variable assignment is a feasible solution
/// of the given model. The values vector should be in one to one
/// correspondence with the model.variables() list of variables.
///
/// # Example
///
/// ```
/// # use cp_sat::builder::CpModelBuilder;
/// # use cp_sat::proto::CpSolverStatus;
/// # use cp_sat::ffi::solution_is_feasible;
/// let mut model = CpModelBuilder::default();
/// let x = model.new_bool_var();
/// let y = model.new_bool_var();
/// model.add_and([x, y]);
/// assert!(solution_is_feasible(model.proto(), &[1, 1]));
/// assert!(!solution_is_feasible(model.proto(), &[1, 0]));
/// assert!(!solution_is_feasible(model.proto(), &[0, 1]));
/// assert!(!solution_is_feasible(model.proto(), &[0, 0]));
/// ```
pub fn solution_is_feasible(model: &proto::CpModelProto, solution: &[i64]) -> bool {
    let mut model_buf = Vec::default();
    model.encode(&mut model_buf).unwrap();
    unsafe {
        cp_sat_wrapper_solution_is_feasible(
            model_buf.as_ptr(),
            model_buf.len(),
            solution.as_ptr(),
            solution.len(),
        )
    }
}
