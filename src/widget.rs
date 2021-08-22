use crate::buffer::{GuardedBuffer, Guard};

pub trait Widget {
    fn display<G>(&self, buffer: &mut GuardedBuffer<'_, G>) where G: Guard;
}
