load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_doc", "rust_doc_test", "rust_test", "rust_binary")
load("@rules_cc//cc:defs.bzl", "cc_library", "cc_binary")

cc_library(
    name = "cp_sat_wrapper",
    hdrs = ["src/cp-sat-wrapper.h"],
    srcs = ["src/cp-sat-wrapper.cc"],
    deps = [
        "@com_google_ortools//ortools/sat:cp_model",
        "@com_google_ortools//ortools/sat:cp_model_checker",
    ],
    copts = ["-std=c++17"],
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
    crate_name = "cp_sat",
    srcs = [
        "src/builder.rs",
        "src/ffi.rs",
        "src/lib.rs",
    ],
    edition = "2021",
    deps = all_crate_deps(normal = True) + [
        ":cp_model_rust_proto",
        ":cp_sat_wrapper",
        ":sat_parameters_rust_proto",
    ],
    visibility = ["//visibility:public"],
)

rust_test(
    name = "cp_model_bindings_test",
    crate = ":cp_model_bindings",
)

rust_doc(
    name = "cp_model_bindings_doc",
    crate = ":cp_model_bindings",
)

rust_doc_test(
    name = "cp_model_bindings_doc_test",
    crate = ":cp_model_bindings",
    deps = all_crate_deps(normal = True) + [
        ":cp_sat_wrapper",
    ],
)

# ## Do the same as above, but with a dynamic c library.

cc_import(
    name = "cp_sat_wrapper_so",
    hdrs = ["cp_sat_wrapper.h"],
    shared_library = ":libcp_sat_wrapper_so.so",
)

cc_binary(
    name = "libcp_sat_wrapper_so.so",
    srcs = [
        "cp_sat_wrapper.cc",
        "cp_sat_wrapper.h",
    ],
    copts = ["-std=c++17"],
    linkshared = True,
)

rust_library(
    name = "cp_model_bindings_dynamically_linked",
    crate_name = "cp_sat",
    srcs = [
        "src/ffi.rs",
        "src/lib.rs",
        "src/builder.rs",
    ],
    edition = "2021",
    target_compatible_with = select({
        # TODO: Make this work on windows
        "@platforms//os:windows": ["@platforms//:incompatible"],
        "//conditions:default": [],
    }),
    deps = all_crate_deps(normal = True) + [
        ":cp_model_rust_proto",
        ":sat_parameters_rust_proto",
        ":cp_sat_wrapper_so",
    ],
)

rust_test(
    name = "cp_model_bindings_dylib_test",
    crate = ":cp_model_bindings_dynamically_linked",
    target_compatible_with = select({
        # TODO: This test requires --incompatible_macos_set_install_name and Bazel 4.2.0+
        "@platforms//os:macos": ["@platforms//:incompatible"],
        # TODO: Make this work on windows
        "@platforms//os:windows": ["@platforms//:incompatible"],
        "//conditions:default": [],
    }),
)

rust_doc(
    name = "cp_model_bindings_dylib_doc",
    crate = ":cp_model_bindings_dynamically_linked",
)

rust_doc_test(
    name = "cp_model_bindings_dylib_doc_test",
    crate = ":cp_model_bindings_dynamically_linked",
)

cc_library(
    name = "ffi_experiment",
    srcs = ["src/ffi-experiment.cc"],
    hdrs = ["src/ffi-experiment.h"],
    copts = ["-std=c++17"],
)

cc_binary(
    name = "main",
    srcs = ["src/main.cc"],
    deps = [":ffi_experiment"],
)

rust_binary(
    name = "my-ffi",
    edition = "2021",
    srcs = ["src/my-ffi.rs"],
    deps = [":ffi_experiment"],
)

cc_import(
    name = "ffi_experiment_dylib",
    hdrs = ["src/ffi-experiment.h"],
    shared_library = ":libffi_experiment_dylib.dylib",
)

cc_binary(
    name = "libffi_experiment_dylib.dylib",
    srcs = [
        "src/ffi-experiment.cc",
        "src/ffi-experiment.h",
    ],
    copts = ["-std=c++17"],
    linkshared = True,
)
