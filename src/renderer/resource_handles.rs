
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

pub struct UniformBufferHandle<U>(pub u32, pub std::marker::PhantomData<U>);

impl<U> UniformBufferHandle<U> {

    pub(crate) fn new(id: u32) -> Self {
        return Self(
            id,
            std::marker::PhantomData
        );
    }

}
impl<U> Copy for UniformBufferHandle<U> {} 
impl<U> Clone for UniformBufferHandle<U> {
    fn clone(&self) -> Self { *self }
} 

#[derive(Copy, Clone)]
pub struct ShaderHandle(pub u32);

#[derive(Copy, Clone)]
pub struct ShaderLayoutHandle(pub u32);

#[derive(Copy, Clone)]
pub struct ViewportHandle(pub u32);

#[derive(Copy, Clone)]
pub struct ScissorHandle(pub u32);

#[derive(Copy, Clone)]
pub struct PipelineHandle(pub u32);