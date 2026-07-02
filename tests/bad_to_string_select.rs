use bytemuck::TransparentWrapper;
use type_tricks::Wrap;

mod share;

use share::named_to_string::*;
use share::named_to_string_impls::*;

#[test]
fn test_named_to_string() {
    let wrap1 = Wrap::<ToStringSelector<NamedToString1>>::new(123);
    assert_eq!(wrap1.to_string(), "NamedToString1");

    let wrap2 = Wrap::<ToStringSelector<NamedToStringProxy<i32>>>::new(456);
    assert_eq!(wrap2.to_string(), "Pre 456 Post");
}

// test unsafe  &T -> &Wrap<T>
#[test]
fn test_named_to_string_ref() {
    let x = 789;
    let ref1: &Wrap<ToStringSelector<NamedToString1>> = Wrap::wrap_ref(&x);
    assert_eq!(ref1.to_string(), "NamedToString1");

    let ref2: &Wrap<ToStringSelector<NamedToStringProxy<i32>>> = Wrap::wrap_ref(&x);
    assert_eq!(ref2.to_string(), "Pre 789 Post");
}
