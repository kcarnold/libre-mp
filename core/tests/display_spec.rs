use libremp_core::capture::DisplaySpec;

#[test]
fn parses_display_specs() {
    assert_eq!("primary".parse(), Ok(DisplaySpec::Primary));
    assert_eq!("5".parse(), Ok(DisplaySpec::Id(5)));
    assert_eq!("1024x768".parse(), Ok(DisplaySpec::Size(1024, 768)));
    assert!("1024x".parse::<DisplaySpec>().is_err());
    assert!("left".parse::<DisplaySpec>().is_err());
}
