use prost_build;

fn main() {
    prost_build::compile_protos(
        // &["src/cp_model.proto", "src/sat_parameters.proto"],
        &[
            "/Users/antoinepouillaude/Downloads/or-tools-9.8/ortools/sat/cp_model.proto",
            "/Users/antoinepouillaude/Downloads/or-tools-9.8/ortools/sat/sat_parameters.proto",
        ],
        &["/Users/antoinepouillaude/Downloads/or-tools-9.8/ortools/sat/"],
    )
    .unwrap();

    if std::env::var("DOCS_RS").is_err() {
        let ortools_prefix = std::env::var("ORTOOLS_PREFIX")
            .ok()
            .unwrap_or_else(|| "/opt/ortools".into());
        println!("Antoine");
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++17")
            .file("src/cp_sat_wrapper.cc")
            .include(&[&ortools_prefix, "/include"].concat())
            .include("/usr/local/Cellar/protobuf/25.1/include/google/protobuf/")
            .compile("cp_sat_wrapper.a");

        println!("cargo:rustc-link-lib=dylib=ortools");
        println!("cargo:rustc-link-search=native={}/lib", ortools_prefix);
    }
}
