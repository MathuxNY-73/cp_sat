use prost_build;

fn main() {
    println!("cargo:rerun-if-changed=src/cp-sat-wrapper.cc");
    prost_build::compile_protos(
        &["src/cp_model.proto", "src/sat_parameters.proto"],
        &["src/"]
    )
    .unwrap();

    if std::env::var("DOCS_RS").is_err() {
        let ortools_prefix = std::env::var("ORTOOLS_PREFIX")
            .ok()
            .unwrap_or_else(|| "/usr/local".into());
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++20")
            .file("src/cp-sat-wrapper.cc")
            .include(&[&ortools_prefix, "/include"].concat())
            .compile("cp_sat_wrapper.a");

        println!("cargo:rustc-link-lib=dylib=ortools");
        println!("cargo:rustc-link-lib=dylib=protobuf");
        println!("cargo:rustc-link-search=native={}/lib", ortools_prefix);
    }
}
