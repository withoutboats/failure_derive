extern crate failure;
#[macro_use] extern crate failure_derive;

#[derive(Fail, Debug)]
#[fail(display = "An error has occurred.")]
struct UnitError;

#[test]
fn unit_struct() {
    let s = format!("{}", UnitError);
    assert_eq!(&s[..], "An error has occurred.");
}

#[derive(Fail, Debug)]
#[fail(display = "Error code: {}", code)]
struct RecordError {
    code: u32,
}

#[test]
fn record_struct() {
    let s = format!("{}", RecordError { code: 0 });
    assert_eq!(&s[..], "Error code: 0");
}

#[derive(Fail, Debug)]
#[fail(display = "Error code: {}", _0)]
struct TupleError(i32);

#[test]
fn tuple_struct() {
    let s = format!("{}", TupleError(2));
    assert_eq!(&s[..], "Error code: 2");
}

#[derive(Fail, Debug)]
enum EnumError {
    #[fail(display = "Error code: {}", code)]
    StructVariant {
        code: i32,
    },
    #[fail(display = "Error: {}", _0)]
    TupleVariant(&'static str),
    #[fail(display = "An error has occurred.")]
    UnitVariant,
}

#[test]
fn enum_error() {
    let s = format!("{}", EnumError::StructVariant { code: 2 });
    assert_eq!(&s[..], "Error code: 2");
    let s = format!("{}", EnumError::TupleVariant("foobar"));
    assert_eq!(&s[..], "Error: foobar");
    let s = format!("{}", EnumError::UnitVariant);
    assert_eq!(&s[..], "An error has occurred.");
}

#[derive(Fail, Debug)]
enum RecursiveError {
    #[fail(display = "Recursive: {}", _0)]
    Some(#[cause] Box<RecursiveError>),
    #[fail(display = "None")]
    None
}

#[test]
fn recursive_error() {
    let s = format!("{}", RecursiveError::None);
    assert_eq!(&s[..], "None");
    let s = format!("{}", RecursiveError::Some(Box::new(RecursiveError::None)));
    assert_eq!(&s[..], "Recursive: None");
    let s = format!("{}", RecursiveError::Some(Box::new(RecursiveError::Some(Box::new(RecursiveError::None)))));
    assert_eq!(&s[..], "Recursive: Recursive: None");
}
