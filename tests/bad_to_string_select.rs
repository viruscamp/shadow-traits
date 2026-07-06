use bytemuck::TransparentWrapper;
use shadow_traits::Wrap;

mod share;

use share::to_string::*;
use share::named_to_string_impls::*;

#[test]
fn test_named_to_string() {
    let wrap1 = Wrap::<ToStringSelector<ToStringImpl1>>::new(123);
    assert_eq!(wrap1.to_string(), "ToStringImpl1");

    let wrap2 = Wrap::<ToStringSelector<ToStringImplProxy<i32>>>::new(456);
    assert_eq!(wrap2.to_string(), "Pre 456 Post");
}

// test unsafe  &T -> &Wrap<T>
#[test]
fn test_named_to_string_ref() {
    let x = 789;
    let ref1: &Wrap<ToStringSelector<ToStringImpl1>> = Wrap::wrap_ref(&x);
    assert_eq!(ref1.to_string(), "ToStringImpl1");

    let ref2: &Wrap<ToStringSelector<ToStringImplProxy<i32>>> = Wrap::wrap_ref(&x);
    assert_eq!(ref2.to_string(), "Pre 789 Post");
}
