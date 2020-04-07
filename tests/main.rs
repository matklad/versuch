use versuch::try_fn;

#[try_fn]
fn returns_option() -> Option<u32> {
    92
}

#[try_fn]
fn returns_result() -> Result<u32, String> {
    if false {
        return Err(":(".to_string());
    }
    92
}

#[try_fn]
fn returns_result_alias() -> std::io::Result<()> {
    std::fs::read_to_string("nope")?;
}

#[test]
fn smoke() {
    assert_eq!(returns_option(), Some(92));
    assert_eq!(returns_result(), Ok(92));
    assert!(returns_result_alias().is_err());
}
