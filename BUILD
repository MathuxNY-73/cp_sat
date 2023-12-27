load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_cc//cc:defs.bzl", "cc_library")

cc_library(
    name = "cp_sat_wrapper",
    srcs = [
        "src/cp_sat_wrapper.cc"
    ],
    deps = [
        "@com_google_ortools//:libortools",
    ],
)

rust_prost_library(
    name = "cp_model_rust_proto",
    tags = ["manual"],
    proto = "@com_google_ortools_protos//ortools/sat:cp_model_proto",
)

rust_prost_library(
    name = "sat_parameters_rust_proto",
    tags = ["manual"],
    proto = "@com_google_ortools_protos//ortools/sat:sat_parameters_proto",
)


rust_library(
    name = "cp_model_bindings",
    srcs = [
        "src/builder.rs",
        "src/ffi.rs",
        "src/lib.rs",
    ],
    edition = "2021",
    deps = all_crate_deps(normal = True) + [
        ":cp_model_rust_proto",
        ":sat_parameters_rust_proto",
        ":cp_sat_wrapper",
    ],
)
