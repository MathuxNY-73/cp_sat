load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(
    default_visibility = ["//:__subpackages__"],
)

# This will run the build script from the root of the workspace, and
# collect the outputs.
cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    edition = "2021",
    deps = all_crate_deps(
        build = True,
    ) + [
    ],
    build_script_env = {
      "ORTOOLS_PREFIX": "/Users/antoinepouillaude/Downloads/or-tools-9.11"
    }
)

rust_library(
  name = "cp_sat_wrapper_lib",
  srcs = glob(
    include = ["src/**/*.rs"],
    exclude = ["src/main.rs"]),
  deps = all_crate_deps(
        normal = True,
    ) + [
      ":build_script",
    ],
  proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)

rust_binary(
  name = "cp_sat_wrapper",
  srcs = ["src/main.rs"],
  deps = all_crate_deps(
        normal = True,
    ) + [
      ":cp_sat_wrapper_lib",
    ],
  proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)
