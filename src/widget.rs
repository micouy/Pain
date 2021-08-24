use crate::buffer::GuardedBuffer;

#[allow(unused_variables)]
pub trait Widget {
    fn display(&self, buffer: &mut GuardedBuffer<'_>) {}
}
