fn main() {
    let config = slint_build::CompilerConfiguration::new().with_style("material".into());
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
    // slint_build::compile("ui/app.slint").unwrap();

    slint_build::print_rustc_flags().unwrap();
}
