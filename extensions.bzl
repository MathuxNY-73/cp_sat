load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _ortools_impl(mctx):
  http_archive(
    name = "ortools-lib",
    url = [
      "https://github.com/google/or-tools/releases/download/v9.12/or-tools_arm64_macOS-15.3.1_cpp_v9.12.4544.tar.gz"
    ],
    build_file = "@//ext/ortools.BUILD",
  )

ortools = module_extension(implementation = _ortools_impl)
