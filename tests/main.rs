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

#[try_fn]
fn bails_none() -> Option<u32> {
    if true {
        return None;
    }
    92
}

#[try_fn]
fn bails_return_err() -> Result<u32, String> {
    if true {
        return Err("meh".into());
    }
    92
}

#[try_fn]
fn bails_errh() -> Result<u32, String> {
    if true {
        Err("meh")?;
    }
    92
}

#[test]
fn smoke() {
    assert_eq!(returns_option(), Some(92));
    assert_eq!(returns_result(), Ok(92));
    assert!(returns_result_alias().is_err());
    assert!(bails_none().is_none());
    assert!(bails_return_err().is_err());
    assert!(bails_errh().is_err());
}
