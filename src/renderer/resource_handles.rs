
#[derive(Copy, Clone)]
pub struct VertexBufferHandle<V>(pub u32, pub std::marker::PhantomData<V>);

impl<V> VertexBufferHandle<V> {

    pub(crate) fn new(id: u32) -> Self {
        return Self(
            id,
            std::marker::PhantomData
        );
    }

}

#[derive(Copy, Clone)]
pub struct IndexBufferHandle<I>(pub u32, pub std::marker::PhantomData<I>);

impl<I> IndexBufferHandle<I> {

    pub(crate) fn new(id: u32) -> Self {
        return Self(
            id,
            std::marker::PhantomData
        );
    }

}