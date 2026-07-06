use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use shadow_traits::{Named, ShadowTrait};

pub struct DebugImpl1;
impl ShadowTrait for DebugImpl1 {
    type Target = i32;
}
impl Debug for Named<DebugImpl1> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("DebugImpl1")
    }
}

pub struct DebugImplProxy<T: Debug>(PhantomData<T>);
impl<T: Debug> ShadowTrait for DebugImplProxy<T> {
    type Target = T;
}
impl<T: Debug> Debug for Named<DebugImplProxy<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("Debug Pre ")?;
        self.0.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
