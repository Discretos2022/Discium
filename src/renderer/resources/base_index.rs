use crate::renderer::resources::index_type::IndexType;



pub trait BaseIndex {
    fn get_index_type() -> IndexType;
}


impl BaseIndex for u8 {
    fn get_index_type() -> IndexType {
        return IndexType::U8
    }
}

impl BaseIndex for u16 {
    fn get_index_type() -> IndexType {
        return IndexType::U16
    }
}

impl BaseIndex for u32 {
    fn get_index_type() -> IndexType {
        return IndexType::U32
    }
}

impl BaseIndex for u64 {
    fn get_index_type() -> IndexType {
        return IndexType::U64
    }
}