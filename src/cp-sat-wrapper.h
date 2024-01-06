#ifndef CP_SAT_CP_SAT_WRAPPER_H_
#define CP_SAT_CP_SAT_WRAPPER_H_

#include <stdint.h>
#include <stdlib.h>

extern "C" unsigned char* cp_sat_wrapper_solve(
    const unsigned char* model_buf,
    size_t model_size,
    size_t* out_size);

extern "C" unsigned char* cp_sat_wrapper_solve_with_parameters(
    const unsigned char* model_buf,
    size_t model_size,
    const unsigned char* params_buf,
    size_t params_size,
    size_t* out_size);

extern "C" char* cp_sat_wrapper_cp_model_stats(
    const unsigned char* model_buf,
    size_t model_size);

extern "C" char* cp_sat_wrapper_cp_solver_response_stats(
    const unsigned char* response_buf,
    size_t response_size,
    bool has_objective);

extern "C" char* cp_sat_wrapper_validate_cp_model(
    const unsigned char* model_buf,
    size_t model_size);

extern "C" bool cp_sat_wrapper_solution_is_feasible(
    const unsigned char* model_buf,
    size_t model_size,
    const int64_t* solution_buf,
    size_t solution_size);

#endif  // CP_SAT_CP_SAT_WRAPPER_H_
