//! A trait that defines methods for mapping between domain entities and Data Transfer Objects (DTOs).
//!
//! # Type Parameters
//! - `Entity`: The type representing the domain entity in the application.
//! - `InDto`: The type representing the input Data Transfer Object used to create or update the entity.
//! - `OutDto`: The type representing the output Data Transfer Object used to present entity data.
//!
//! # Required Methods
//!
//! - `fn to_entity(dto: InDto) -> Entity`
//!   - Maps an input DTO (`InDto`) to a corresponding domain entity (`Entity`).
//!   - # Parameters
//!     - `dto`: The input DTO instance.
//!   - # Returns
//!     - The corresponding domain entity instance.
//!
//! - `fn to_dto(entity: Entity) -> OutDto`
//!   - Maps a domain entity (`Entity`) to its corresponding output DTO (`OutDto`).
//!   - # Parameters
//!     - `entity`: The domain entity instance.
//!   - # Returns
//!     - The corresponding output DTO instance.
//!
//! - `fn to_dtos(entities: Vec<Entity>) -> Vec<OutDto>`
//!   - Maps a collection of domain entities (`Vec<Entity>`) to a collection of output DTOs (`Vec<OutDto>`).
//!   - # Parameters
//!     - `entities`: A vector of domain entity instances.
//!   - # Returns
//!     - A vector of corresponding output DTO instances.
pub trait DTOMapper<Entity, InDto, OutDto> {
    fn to_entity(dto: InDto) -> Entity;
    fn to_dto(entity: Entity) -> OutDto;
    fn to_dtos(entities: Vec<Entity>) -> Vec<OutDto> ;
}