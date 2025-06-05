use application::DTOs::user::user_in_dto::{UserCreateInDTO, UserUpdateInDTO};
use application::DTOs::user::user_out_dto::UserOutDTO;
use application::mappers::api_mapper::{ApiInMapper, ApiOutMapper};
use crate::api::user_facts::user_facts_payloads::{CreateUserFactPayload, UpdateUserFactPayload};
use crate::api::user_facts::user_facts_presenters::UserFactsPresenter;

pub struct UserFactsMapper;

impl ApiInMapper<CreateUserFactPayload, UserCreateInDTO> for UserFactsMapper {
    fn to_api(entity: CreateUserFactPayload) -> UserCreateInDTO {
        UserCreateInDTO {
            name: entity.name,
            phone: entity.phone,
            address: entity.address,
        }
    }
}

impl ApiInMapper<UpdateUserFactPayload, UserUpdateInDTO> for UserFactsMapper {
    fn to_api(entity: UpdateUserFactPayload) -> UserUpdateInDTO {
        UserUpdateInDTO {
            name: entity.name,
            phone: entity.phone,
            address: entity.address,
        }
    }
}

impl ApiOutMapper<UserOutDTO, UserFactsPresenter> for UserFactsMapper {
    fn to_presenter(presenter: UserOutDTO) -> UserFactsPresenter {
        UserFactsPresenter {
            id_user: presenter.id_user,
            name: presenter.name,
            phone: presenter.phone,
            address: presenter.address,
        }
    }
}

