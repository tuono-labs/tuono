#[test]
fn handler() {
    macrotest::expand("tests/handler/*.rs");
}

#[test]
fn api() {
    macrotest::expand("tests/api/*.rs");
}
