use std::any::Any;

pub fn as_any_ref<T: 'static>(t: &T) -> &dyn Any {
    t
}

pub fn as_any_mut<T: 'static>(t: &mut T) -> &mut dyn Any {
    t
}
