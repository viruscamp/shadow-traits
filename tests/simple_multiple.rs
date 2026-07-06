use bytemuck::TransparentWrapper;
use shadow_traits::ShadowTrait;
use shadow_traits::Wrap;

mod share;

use share::named_display_impls::*;
use share::named_debug_impls::*;

use shadow_traits::display::{DisplayProvider};
use shadow_traits::debug::{DebugProvider};

pub struct SimpleMultipleTag;

impl ShadowTrait for SimpleMultipleTag {
    type Target = i32;
}

impl DisplayProvider for SimpleMultipleTag {
    type Impl = DisplayImpl1;
}

impl DebugProvider for SimpleMultipleTag {
    type Impl = DebugImpl1;
}

#[test]
fn test_simple_multiple() {
    let num = 42;

    // note: to_string() calls Display, format!("{:?}") calls Debug
    
    let a1 = Wrap::<SimpleMultipleTag>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "DisplayImpl1");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");
}
