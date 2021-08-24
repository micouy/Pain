use crate::buffer::GuardedBuffer;

pub trait Widget {
    fn display(&self, buffer: &mut GuardedBuffer<'_, '_>);
}
