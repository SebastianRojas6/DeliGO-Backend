use domain::user::UserEntity;
use crate::dtos::user::user_in_dto::{UserCreateInDTO, UserUpdateInDTO};
use crate::dtos::user::user_out_dto::UserOutDTO;
use crate::mappers::app_mapper::DTOMapper;

pub struct UserMapper;

impl DTOMapper<UserEntity, UserOutDTO> for UserMapper {

    fn to_dto(entity: UserEntity) -> UserOutDTO {
        UserOutDTO {
            id_user: entity.id_user,
            name: entity.name,
            phone: entity.phone,
            address: entity.address,
        }
    }

    fn to_dtos(entities: Vec<UserEntity>) -> Vec<UserOutDTO> {
        entities.into_iter().map(Self::to_dto).collect()
    }
}