workspace(name = "cp_sat")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

HERMETIC_CC_TOOLCHAIN_VERSION = "v2.1.3"

http_archive(
    name = "rules_cc",
    urls = ["https://github.com/bazelbuild/rules_cc/releases/download/0.0.9/rules_cc-0.0.9.tar.gz"],
    sha256 = "2037875b9a4456dce4a79d112a8ae885bbc4aad968e6587dca6e64f3a0900cdf",
    strip_prefix = "rules_cc-0.0.9",
)
http_archive(
    name = "hermetic_cc_toolchain",
    sha256 = "a5caccbf6d86d4f60afd45b541a05ca4cc3f5f523aec7d3f7711e584600fb075",
    urls = [
        "https://mirror.bazel.build/github.com/uber/hermetic_cc_toolchain/releases/download/{0}/hermetic_cc_toolchain-{0}.tar.gz".format(HERMETIC_CC_TOOLCHAIN_VERSION),
        "https://github.com/uber/hermetic_cc_toolchain/releases/download/{0}/hermetic_cc_toolchain-{0}.tar.gz".format(HERMETIC_CC_TOOLCHAIN_VERSION),
    ],
)
http_archive(
    name = "com_google_protobuf",
    sha256 = "9bd87b8280ef720d3240514f884e56a712f2218f0d693b48050c836028940a42",
    url = "https://github.com/protocolbuffers/protobuf/archive/refs/tags/v25.1.tar.gz",
    strip_prefix = "protobuf-25.1",
)
http_archive(
    name = "rules_rust",
    sha256 = "75177226380b771be36d7efc538da842c433f14cd6c36d7660976efb53defe86",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.34.1/rules_rust-v0.34.1.tar.gz"],
)
http_archive(
    name = "com_google_ortools_protos",
    sha256 = "85e10e7acf0a9d9a3b891b9b108f76e252849418c6230daea94ac429af8a4ea4",
    url = "https://github.com/google/or-tools/archive/refs/tags/v9.8.tar.gz",
    strip_prefix = "or-tools-9.8",
)
http_archive(
    name = "com_google_ortools",
    sha256 = "253efad127c55b78967e3e3a3b4a573f9da0a2562c4f33f14fbf462ca58448f7",
    url = "https://github.com/google/or-tools/releases/download/v9.8/or-tools_arm64_macOS-14.1_cpp_v9.8.3296.tar.gz",
    strip_prefix = "or-tools_arm64_macOS-14.1_cpp_v9.8.3296",
    build_file = "//third_party/com_google_ortools:com_google_ortools.BUILD.bazel",
)

load("@hermetic_cc_toolchain//toolchain:defs.bzl", zig_toolchains = "toolchains")

# Plain zig_toolchains() will pick reasonable defaults. See
# toolchain/defs.bzl:toolchains on how to change the Zig SDK version and
# download URL.
zig_toolchains()

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
protobuf_deps()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")
rules_proto_dependencies()
rules_proto_toolchains()

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

register_toolchains(
    "//toolchains:prost_toolchain",
    # "@zig_sdk//toolchain:linux_amd64_gnu.2.28",
    # "@zig_sdk//toolchain:linux_arm64_gnu.2.28",
    # "@zig_sdk//toolchain:darwin_amd64",
    # "@zig_sdk//toolchain:darwin_arm64",
    # "@zig_sdk//toolchain:windows_amd64",
    # "@zig_sdk//toolchain:windows_arm64",
)
