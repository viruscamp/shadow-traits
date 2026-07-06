use bytemuck::TransparentWrapper;
use shadow_traits::Wrap;

mod share;

use share::named_display_impls::*;

/// use Wrap and a named impl directly
#[test]
fn test_named_display() {
    let wrap1 = Wrap::<DisplayImpl1>::new(123);
    assert_eq!(wrap1.to_string(), "DisplayImpl1");

    let wrap2 = Wrap::<DisplayImplProxy<i32>>::new(456);
    assert_eq!(wrap2.to_string(), "Display Pre 456 Post");
}

// test unsafe convert: &T -> &Wrap<T>
#[test]
fn test_named_display_ref() {
    let x = 789;
    let ref1: &Wrap<DisplayImpl1> = Wrap::wrap_ref(&x);
    assert_eq!(ref1.to_string(), "DisplayImpl1");

    let ref2: &Wrap<DisplayImplProxy<i32>> = Wrap::wrap_ref(&x);
    assert_eq!(ref2.to_string(), "Display Pre 789 Post");
}
