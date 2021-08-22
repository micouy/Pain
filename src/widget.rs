use crate::buffer::{Guard, GuardedBuffer};

pub trait Widget {
    fn display(&self, buffer: &mut GuardedBuffer<'_>);
}
