load("@rules_rust//proto/prost:defs.bzl", "rust_prost_toolchain")
load("@rules_rust//rust:defs.bzl", "rust_library_group")
load(":cc_toolchain_config.bzl", "cc_toolchain_config")

package(default_visibility = ["//visibility:public"])

rust_library_group(
    name = "prost_runtime",
    deps = [
        "@crate_index//:prost",
    ],
)

rust_library_group(
    name = "tonic_runtime",
    deps = [
        ":prost_runtime",
        "@crate_index//:tonic",
    ],
)

rust_prost_toolchain(
    name = "prost_toolchain_impl",
    prost_plugin = "@crate_index//:protoc-gen-prost__protoc-gen-prost",
    prost_runtime = ":prost_runtime",
    prost_types = "@crate_index//:prost-types",
    proto_compiler = "@com_google_protobuf//:protoc",
    tonic_plugin = "@crate_index//:protoc-gen-tonic__protoc-gen-tonic",
    tonic_runtime = ":tonic_runtime",
)

toolchain(
    name = "prost_toolchain",
    toolchain = "prost_toolchain_impl",
    toolchain_type = "@rules_rust//proto/prost:toolchain_type",
)

filegroup(name = "empty")

cc_toolchain_config(name = "arm64_toolchain_config")

cc_toolchain_suite(
    name = "clang_suite",
    toolchains = {
        "clang": ":arm64_toolchain",
    }
)

cc_toolchain(
    name = "arm64_clang_toolchain",
    toolchain_identifier = "arm64-toolchain",
    toolchain_config = ":arm64_toolchain_config",
    all_files = ":empty",
    compiler_files = ":empty",
    dwp_files = ":empty",
    linker_files = ":empty",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 0,
)