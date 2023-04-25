use tonic::{transport::Server, Request, Response, Status};
use user::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, Empty,
    GellAllUsersResponse, UpdateUserRequest, UpdateUserResponse,
};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Default)]
pub struct User {}

impl UserService for User {
    
}
