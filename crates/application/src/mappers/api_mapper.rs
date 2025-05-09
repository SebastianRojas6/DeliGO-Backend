pub trait ApiMapper<Entity, Presenter, Payload, DTO> {
    fn to_api(entity: Entity) -> Presenter;
    fn to_dto(payload: Payload) -> DTO;

}