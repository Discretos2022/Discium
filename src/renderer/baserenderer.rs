use crate::window::rawhandle::RawHandle;


pub trait BaseRenderer {

    fn create(raw_handle: &RawHandle) -> Self where Self: Sized;

}