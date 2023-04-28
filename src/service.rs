use mongodb::bson::oid::ObjectId;
use tonic::{Request, Response, Status};
use user::{
    user_service_server::UserService, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, Empty, GetAllUsersResponse, UpdateUserRequest, UpdateUserResponse,
};

use crate::mongo_connection::{self, DBMongo};

use self::user::{UserRequest, UserResponse};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Debug, Default)]
pub struct User {}

#[tonic::async_trait]
impl UserService for User {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let new_user = mongo_connection::User {
            id: None,
            name: req.name,
            location: req.location,
            title: req.title,
        };
        let db = DBMongo::create_user(new_user).await;

        match db {
            Ok(resp) => {
                let user = CreateUserResponse {
                    data: resp.inserted_id.to_string(),
                };
                Ok(Response::new(user))
            }
            Err(error) => Err(Status::aborted(format!("{}", error))),
        }
    }

    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let db = DBMongo::get_user(req.id).await;

        match db {
            Ok(resp) => {
                let user = UserResponse {
                    id: resp.id.unwrap().to_string(),
                    name: resp.name,
                    location: resp.location,
                    title: resp.title,
                };
                Ok(Response::new(user))
            }
            Err(error) => Err(Status::aborted(format!("{}", error))),
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let req = request.into_inner();
        let new_user = mongo_connection::User {
            id: Some(ObjectId::parse_str(req.id.clone()).unwrap()),
            name: req.name,
            location: req.location,
            title: req.title,
        };
        let db = DBMongo::update_user(req.id.clone(), new_user).await;

        match db {
            Ok(_) => {
                let user = UpdateUserResponse {
                    data: String::from("User details updated successfully!"),
                };
                Ok(Response::new(user))
            }
            Err(error) => Err(Status::aborted(format!("{}", error))),
        }
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let req = request.into_inner();
        let db = DBMongo::delete_user(req.id).await;

        match db {
            Ok(_) => {
                let user = DeleteUserResponse {
                    data: String::from("User details deleted successfully!"),
                };
                Ok(Response::new(user))
            }
            Err(error) => Err(Status::aborted(format!("{}", error))),
        }
    }

    async fn get_all_users(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<GetAllUsersResponse>, Status> {
        let db = DBMongo::get_all_users().await;

        match db {
            Ok(resp) => {
                let mut user_list: Vec<UserResponse> = Vec::new();
                for data in resp {
                    let mapped_user = UserResponse {
                        id: data.id.unwrap().to_string(),
                        name: data.name,
                        location: data.location,
                        title: data.title,
                    };
                    user_list.push(mapped_user);
                }

                let user = GetAllUsersResponse { users: user_list };
                Ok(Response::new(user))
            }
            Err(error) => Err(Status::aborted(format!("{}", error))),
        }
    }
}
