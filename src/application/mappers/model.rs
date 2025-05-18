pub trait ModelMapper<E> {
    fn from_entity(entity: E) -> Self;
    fn to_entity(&self) -> E;
}
