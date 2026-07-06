use core::marker::PhantomData;
use core::fmt::{Display, Formatter, Result};

use shadow_traits::{Named, ShadowTrait};

pub struct DisplayImpl1;
impl ShadowTrait for DisplayImpl1 {
    type Target = i32;
}
impl Display for Named<DisplayImpl1> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("DisplayImpl1")
    }
}

pub struct DisplayImplProxy<T: Display>(PhantomData<T>);
impl<T: Display> ShadowTrait for DisplayImplProxy<T> {
    type Target = T;
}
impl<T: Display> Display for Named<DisplayImplProxy<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("Display Pre ")?;
        self.0.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
