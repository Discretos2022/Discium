use crate::window::rawhandle::RawHandle;


pub trait BaseWindow {

    fn create() -> Self where Self: Sized;
    fn pool_events(&self) -> bool;
    fn get_raw_handle(&self) -> RawHandle;

}