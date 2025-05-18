pub trait ApplicationMapper<Model, DTO> {
    fn to_dto(model: Model) -> DTO;
}
