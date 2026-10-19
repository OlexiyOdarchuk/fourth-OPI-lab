fn main() {
    cxx_build::CFG.include_prefix = "";

    cxx_build::bridge("src/fare_calculation.rs")
        .file("cpp/fare_calculation.cpp")
        .std("c++17")
        .flag_if_supported("-Wpedantic")
        .compile("fare_calculation");

    println!("cargo:rerun-if-changed=src/fare_calculation.rs");
    println!("cargo:rerun-if-changed=cpp");
}
