use core::fmt::{Debug, Formatter, Result};

use type_tricks::{NamedImplBase, Wrap, debug::NamedDebug};

mod share;
use share::named_debug_impls::NamedDebugProxy;

struct WrapI32Tag;
impl NamedImplBase for WrapI32Tag {
    type Target = i32;
}
/// I donot want WrapI32 to be a transparent wrapper, 
/// so Deref is disabled with ImplDeref=false
type WrapI32 = Wrap<WrapI32Tag, false>;

impl ToString for WrapI32 {
    fn to_string(&self) -> String {
        format!("WrapI32({})", self.0)
    }
}

impl Debug for WrapI32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NamedDebugProxy::fmt(&self.0, f)
    }
}

#[test]
fn test_wrap_as_newtype() {
    let wrap = WrapI32::new(42);
    assert_eq!(wrap.to_string(), "WrapI32(42)");
    assert_eq!(format!("{wrap:?}"), "Debug Pre 42 Post");
}
