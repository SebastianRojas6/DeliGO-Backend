pub trait ApiMapper<Entity, Presenter, Payload> {
    fn to_api(entity: Entity) -> Presenter;
    fn to_entity(payload: Payload) -> Entity;
}