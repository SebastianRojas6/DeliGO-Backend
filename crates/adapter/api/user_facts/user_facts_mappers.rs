use application::DTOs::user_dto::{CreateUserDTOs, UpdateUserDTOs};
use application::mappers::api_mapper::ApiMapper;
use domain::user::UserEntity;
use crate::api::user_facts::user_facts_payloads::{CreateUserFactPayload, UpdateUserFactPayload};
use crate::api::user_facts::user_facts_presenters::UserFactsPresenter;

pub struct UserFactsMapper;

impl ApiMapper<UserEntity, UserFactsPresenter, CreateUserFactPayload, CreateUserDTOs> for UserFactsMapper {
    fn to_api(entity: UserEntity) -> UserFactsPresenter {
        UserFactsPresenter {
            id_user: entity.id_user,
            name: entity.name,
            phone: entity.phone,
            address: entity.address,
        }
    }

    fn to_dto(payload: CreateUserFactPayload) -> CreateUserDTOs {
        CreateUserDTOs {
            name: payload.name,
            phone: payload.phone,
            address: payload.address,
        }
    }
}

pub struct UserFactsUpdateMapper;

impl ApiMapper<UserEntity, UserFactsPresenter, UpdateUserFactPayload, UpdateUserDTOs> for UserFactsUpdateMapper {
    fn to_api(entity: UserEntity) -> UserFactsPresenter {
        UserFactsPresenter {
            id_user: entity.id_user,
            name: entity.name,
            phone: entity.phone,
            address: entity.address,
        }
    }

    fn to_dto(payload: UpdateUserFactPayload) -> UpdateUserDTOs {
        UpdateUserDTOs {
            id_user: payload.id_user,
            name: payload.name,
            phone: payload.phone,
            address: payload.address,
        }
    }
}