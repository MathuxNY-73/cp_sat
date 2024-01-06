workspace(name = "cp_sat")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")

# HERMETIC_CC_TOOLCHAIN_VERSION = "v2.1.3"

# # Dependencies
# ## ZLIB
# new_git_repository(
#     name = "zlib",
#     build_file = "@com_google_protobuf//:third_party/zlib.BUILD",
#     tag = "v1.2.13",
#     remote = "https://github.com/madler/zlib.git",
# )

# ## Re2
# git_repository(
#     name = "com_google_re2",
#     tag = "2023-11-01",
#     remote = "https://github.com/google/re2.git",
# )

# # Abseil-cpp
# git_repository(
#     name = "com_google_absl",
#     tag = "20211102.0",
#     remote = "https://github.com/abseil/abseil-cpp.git",
# )

# ## Solvers
# http_archive(
#     name = "glpk",
#     build_file = "//bazel:glpk.BUILD",
#     sha256 = "4a1013eebb50f728fc601bdd833b0b2870333c3b3e5a816eeba921d95bec6f15",
#     url = "http://ftp.gnu.org/gnu/glpk/glpk-5.0.tar.gz",
# )

# http_archive(
#     name = "bliss",
#     build_file = "//bazel:bliss.BUILD",
#     patches = ["//bazel:bliss-0.73.patch"],
#     sha256 = "f57bf32804140cad58b1240b804e0dbd68f7e6bf67eba8e0c0fa3a62fd7f0f84",
#     url = "https://github.com/google/or-tools/releases/download/v9.0/bliss-0.73.zip",
#     #url = "http://www.tcs.hut.fi/Software/bliss/bliss-0.73.zip",
# )

# new_git_repository(
#     name = "scip",
#     build_file = "//bazel:scip.BUILD",
#     patches = ["//third_party/bliss:scip.patch"],
#     patch_args = ["-p1"],
#     tag = "v804",
#     remote = "https://github.com/scipopt/scip.git",
# )

# # Eigen has no Bazel build.
# new_git_repository(
#     name = "eigen",
#     tag = "3.4.0",
#     remote = "https://gitlab.com/libeigen/eigen.git",
#     build_file_content =
# """
# cc_library(
#     name = 'eigen3',
#     srcs = [],
#     includes = ['.'],
#     hdrs = glob(['Eigen/**']),
#     defines = ["EIGEN_MPL2_ONLY",],
#     visibility = ['//visibility:public'],
# )
# """
# )

# git_repository(
#     name = "highs",
#     branch = "bazel",
#     remote = "https://github.com/ERGO-Code/HiGHS.git",
# )

# # git_repository(
# #     name = "rules_proto",
# #     tag = "5.3.0-21.7",
# #     remote = "https://github.com/bazelbuild/rules_proto.git",
# # )

# http_archive(
#     name = "rules_cc",
#     urls = ["https://github.com/bazelbuild/rules_cc/releases/download/0.0.9/rules_cc-0.0.9.tar.gz"],
#     sha256 = "2037875b9a4456dce4a79d112a8ae885bbc4aad968e6587dca6e64f3a0900cdf",
#     strip_prefix = "rules_cc-0.0.9",
# )
# http_archive(
#     name = "hermetic_cc_toolchain",
#     sha256 = "a5caccbf6d86d4f60afd45b541a05ca4cc3f5f523aec7d3f7711e584600fb075",
#     urls = [
#         "https://mirror.bazel.build/github.com/uber/hermetic_cc_toolchain/releases/download/{0}/hermetic_cc_toolchain-{0}.tar.gz".format(HERMETIC_CC_TOOLCHAIN_VERSION),
#         "https://github.com/uber/hermetic_cc_toolchain/releases/download/{0}/hermetic_cc_toolchain-{0}.tar.gz".format(HERMETIC_CC_TOOLCHAIN_VERSION),
#     ],
# )
# ## Protobuf
# # proto_library, cc_proto_library, and java_proto_library rules implicitly
# # depend on @com_google_protobuf for protoc and proto runtimes.
# # This statement defines the @com_google_protobuf repo.
# git_repository(
#     name = "com_google_protobuf",
#     tag = "v3.19.4",
#     # patches = ["//patches:protobuf-v25.7.patch"],
#     # patch_args = ["-p1"],
#     remote = "https://github.com/protocolbuffers/protobuf.git",
# )
# # Load common dependencies.
# load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
# protobuf_deps()

http_archive(
    name = "rules_rust",
    sha256 = "ff1c4b8d154509154acbad7af94d1dda3b59163e62bcd81f8087df10a5f66468",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.36.1/rules_rust-v0.36.1.tar.gz"],
)
# git_repository(
#     name = "com_google_ortools_protos",
#     #branch = "main",
#     tag = "v9.4",
#     remote = "https://github.com/google/or-tools.git",
# )
# http_archive(
#     name = "com_google_ortools_protos",
#     sha256 = "180fbc45f6e5ce5ff153bea2df0df59b15346f2a7f8ffbd7cb4aed0fb484b8f6",
#     url = "https://github.com/google/or-tools/archive/refs/tags/v9.4.tar.gz",
#     strip_prefix = "or-tools-9.4",
# )
# http_archive(
#     name = "com_google_ortools",
#     sha256 = "253efad127c55b78967e3e3a3b4a573f9da0a2562c4f33f14fbf462ca58448f7",
#     url = "https://github.com/google/or-tools/releases/download/v9.8/or-tools_arm64_macOS-14.1_cpp_v9.8.3296.tar.gz",
#     strip_prefix = "or-tools_arm64_macOS-14.1_cpp_v9.8.3296",
#     build_file = "//third_party/com_google_ortools:com_google_ortools.BUILD.bazel",
# )

# load("@hermetic_cc_toolchain//toolchain:defs.bzl", zig_toolchains = "toolchains")

# Plain zig_toolchains() will pick reasonable defaults. See
# toolchain/defs.bzl:toolchains on how to change the Zig SDK version and
# download URL.
# zig_toolchains()

# load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
# protobuf_deps()

# load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")
# rules_proto_dependencies()
# rules_proto_toolchains()




# ORTools Dependencies

## ZLIB
new_git_repository(
    name = "zlib",
    build_file = "@com_google_protobuf//:third_party/zlib.BUILD",
    tag = "v1.2.13",
    remote = "https://github.com/madler/zlib.git",
)

## Re2
git_repository(
    name = "com_google_re2",
    tag = "2023-11-01",
    remote = "https://github.com/google/re2.git",
    build_file = "//third_party/com_google_re2:com_google_re2.BUILD.bazel",
)

git_repository(
    name = "com_google_absl",
    tag = "20230802.1",
    build_file = "//third_party/com_google_absl:com_google_absl.BUILD.bazel",
    patches = ["//third_party/com_google_absl:abseil-cpp-20230802.1.patch"],
    patch_args = ["-p1"],
    remote = "https://github.com/abseil/abseil-cpp.git",
)

## Protobuf
# proto_library, cc_proto_library, and java_proto_library rules implicitly
# depend on @com_google_protobuf for protoc and proto runtimes.
# This statement defines the @com_google_protobuf repo.
git_repository(
    name = "com_google_protobuf",
    tag = "v25.0",
    patches = ["//third_party/com_google_protobuf:protobuf-v25.0.patch"],
    patch_args = ["-p1"],
    remote = "https://github.com/protocolbuffers/protobuf.git",
    # build_file = "//third_party/com_google_protobuf:com_google_protobuf.BUILD.bazel",
)

## Solvers
http_archive(
    name = "glpk",
    build_file = "//third_party/glpk:glpk.BUILD.bazel",
    sha256 = "4a1013eebb50f728fc601bdd833b0b2870333c3b3e5a816eeba921d95bec6f15",
    url = "http://ftp.gnu.org/gnu/glpk/glpk-5.0.tar.gz",
)

http_archive(
    name = "bliss",
    build_file = "//third_party/bliss:bliss.BUILD.bazel",
    patches = ["//third_party/bliss:bliss-0.73.patch"],
    sha256 = "f57bf32804140cad58b1240b804e0dbd68f7e6bf67eba8e0c0fa3a62fd7f0f84",
    url = "https://github.com/google/or-tools/releases/download/v9.0/bliss-0.73.zip",
    #url = "http://www.tcs.hut.fi/Software/bliss/bliss-0.73.zip",
)

new_git_repository(
    name = "scip",
    build_file = "//third_party/scip:scip.BUILD.bazel",
    patches = ["//third_party/scip:scip.patch"],
    patch_args = ["-p1"],
    tag = "v804",
    remote = "https://github.com/scipopt/scip.git",
)

# Eigen has no Bazel build.
new_git_repository(
    name = "eigen",
    tag = "3.4.0",
    remote = "https://gitlab.com/libeigen/eigen.git",
    build_file_content =
"""
cc_library(
    name = 'eigen3',
    srcs = [],
    includes = ['.'],
    hdrs = glob(['Eigen/**']),
    defines = ["EIGEN_MPL2_ONLY",],
    visibility = ['//visibility:public'],
)
"""
)

git_repository(
    name = "highs",
    branch = "bazel",
    build_file = "//third_party/highs:highs.BUILD.bazel",
    remote = "https://github.com/ERGO-Code/HiGHS.git",
)

git_repository(
    name = "com_google_ortools",
    #branch = "main",
    tag = "v9.8",
    remote = "https://github.com/google/or-tools.git",
    build_file = "//third_party/com_google_ortools:com_google_ortools.BUILD.bazel",
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains()

load("@rules_rust//proto/prost:repositories.bzl", "rust_prost_dependencies")
rust_prost_dependencies()

load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")
rust_prost_transitive_repositories()

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate", "splicing_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo-bazel.lock.json",
    manifests = [
        "//:Cargo.toml",
    ],
    annotations = {
        "protoc-gen-prost": [crate.annotation(
            gen_binaries = ["protoc-gen-prost"],
        )],
        "protoc-gen-tonic": [crate.annotation(
            gen_binaries = ["protoc-gen-tonic"],
        )],
    },
    packages = {
        "prost-types": crate.spec(
            version = "0.12.3",
        ),
        "prost-build": crate.spec(
            version = "0.12.3",
        ),
        "protoc-gen-prost": crate.spec(
            version = "0.2.3",
        ),
        "protoc-gen-tonic": crate.spec(
            version = "0.3.0",
        ),
    },
    splicing_config = splicing_config(resolver_version = "2"),
    tags = ["manual"],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

http_archive(
    name = "rules_foreign_cc",
    sha256 = "476303bd0f1b04cc311fc258f1708a5f6ef82d3091e53fd1977fa20383425a6a",
    strip_prefix = "rules_foreign_cc-0.10.1",
    url = "https://github.com/bazelbuild/rules_foreign_cc/releases/download/0.10.1/rules_foreign_cc-0.10.1.tar.gz",
)

load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
rules_foreign_cc_dependencies()

register_toolchains(
    "//toolchains:prost_toolchain",
    # "@zig_sdk//toolchain:linux_amd64_gnu.2.28",
    # "@zig_sdk//toolchain:linux_arm64_gnu.2.28",
    # "@zig_sdk//toolchain:darwin_amd64",
    # "@zig_sdk//toolchain:darwin_arm64",
    # "@zig_sdk//toolchain:windows_amd64",
    # "@zig_sdk//toolchain:windows_arm64",
)
