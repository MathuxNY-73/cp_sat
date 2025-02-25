#include <iostream>

#include "ortools/sat/cp_model.h"
#include "ortools/sat/cp_model.pb.h"
#include "ortools/sat/cp_model_checker.h"
#include "ortools/sat/cp_model_solver.h"
#include "ortools/sat/sat_parameters.pb.h"
#include "ortools/util/time_limit.h"

namespace sat = ::operations_research::sat;

using ::operations_research::TimeLimit;

extern "C" unsigned char*
cp_sat_wrapper_solve(
    const unsigned char* model_buf,
    size_t model_size,
    size_t* out_size)
{
    sat::CpModelProto model;
    bool res = model.ParseFromArray(model_buf, model_size);
    assert(res);

    sat::CpSolverResponse response = sat::Solve(model);

    *out_size = response.ByteSizeLong();
    unsigned char* out_buf = (unsigned char*) malloc(*out_size);
    res = response.SerializeToArray(out_buf, *out_size);
    assert(res);

    return out_buf;
}

extern "C" unsigned char*
cp_sat_wrapper_solve_with_parameters(
    const unsigned char* model_buf,
    size_t model_size,
    const unsigned char* params_buf,
    size_t params_size,
    size_t* out_size)
{
    sat::CpModelProto model;
    bool res = model.ParseFromArray(model_buf, model_size);
    assert(res);

    sat::SatParameters params;
    res = params.ParseFromArray(params_buf, params_size);
    assert(res);

    sat::CpSolverResponse response = sat::SolveWithParameters(model, params);

    *out_size = response.ByteSizeLong();
    unsigned char* out_buf = (unsigned char*) malloc(*out_size);
    res = response.SerializeToArray(out_buf, *out_size);
    assert(res);

    return out_buf;
}

extern "C" char*
cp_sat_wrapper_cp_model_stats(const unsigned char* model_buf, size_t model_size) {
    sat::CpModelProto model;
    const bool res = model.ParseFromArray(model_buf, model_size);
    assert(res);

    const std::string stats = sat::CpModelStats(model);
    return strdup(stats.c_str());
}

extern "C" char*
cp_sat_wrapper_cp_solver_response_stats(
    const unsigned char* response_buf,
    size_t response_size,
    bool has_objective)
{
    sat::CpSolverResponse response;
    const bool res = response.ParseFromArray(response_buf, response_size);
    assert(res);

    const std::string stats = sat::CpSolverResponseStats(response, has_objective);
    return strdup(stats.c_str());
}

extern "C" char*
cp_sat_wrapper_validate_cp_model(const unsigned char* model_buf, size_t model_size) {
    sat::CpModelProto model;
    const bool res = model.ParseFromArray(model_buf, model_size);
    assert(res);

    const std::string stats = sat::ValidateCpModel(model);
    return strdup(stats.c_str());
}

extern "C" bool
cp_sat_wrapper_solution_is_feasible(
    const unsigned char* model_buf,
    size_t model_size,
    const int64_t* solution_buf,
    size_t solution_size)
{
    sat::CpModelProto model;
    const bool res = model.ParseFromArray(model_buf, model_size);
    assert(res);

    std::vector<int64_t> variable_values;
    variable_values.reserve(solution_size);
    for (size_t i = 0; i < solution_size; ++i) {
        variable_values.push_back(solution_buf[i]);
    }

    return sat::SolutionIsFeasible(model, variable_values);
}

extern "C" {
    typedef bool (*rust_callback)(void*, const unsigned char* , size_t);
}

extern "C" unsigned char*
cp_sat_wrapper_solve_with_parameters_and_observer(
        const unsigned char* model_buf,
        size_t model_size,
        const unsigned char* params_buf,
        size_t params_size,
        size_t* out_size,
        rust_callback callback,
        void* cb_data) {
    sat::CpModelProto model_proto;
    bool res = model_proto.ParseFromArray(model_buf, model_size);
    assert(res);

    sat::Model model;
    sat::SatParameters params;
    if (params_buf && params_size > 0) {
        res = params.ParseFromArray(params_buf, params_size);
        assert(res);
        model.Add(sat::NewSatParameters(params));
    }

    std::atomic<bool> stopped(false);

    model.GetOrCreate<TimeLimit>()->RegisterExternalBooleanAsLimit(&stopped);

    model.Add(sat::NewFeasibleSolutionObserver([&](const sat::CpSolverResponse& r) {
        const size_t buf_size = r.ByteSizeLong();
        unsigned char* const resp_buf = (unsigned char*) malloc(buf_size);
        res = r.SerializeToArray(resp_buf, buf_size);
        stopped = !callback(cb_data, resp_buf, buf_size);
    }));

    sat::CpSolverResponse response = sat::SolveCpModel(model_proto, &model);

    *out_size = response.ByteSizeLong();
    unsigned char* out_buf = (unsigned char*) malloc(*out_size);
    res = response.SerializeToArray(out_buf, *out_size);
    assert(res);

    return out_buf;
}
