use bytemuck::TransparentWrapper;
use type_tricks::Wrap;

mod share;

use share::named_display_impls::*;

#[test]
fn test_named_display() {
    let wrap1 = Wrap::<NamedDisplay1>::new(123);
    assert_eq!(wrap1.to_string(), "NamedDisplay1");

    let wrap2 = Wrap::<NamedDisplayProxy<i32>>::new(456);
    assert_eq!(wrap2.to_string(), "Display Pre 456 Post");
}

// test unsafe convert: &T -> &Wrap<T>
#[test]
fn test_named_display_ref() {
    let x = 789;
    let ref1: &Wrap<NamedDisplay1> = Wrap::wrap_ref(&x);
    assert_eq!(ref1.to_string(), "NamedDisplay1");

    let ref2: &Wrap<NamedDisplayProxy<i32>> = Wrap::wrap_ref(&x);
    assert_eq!(ref2.to_string(), "Display Pre 789 Post");
}
