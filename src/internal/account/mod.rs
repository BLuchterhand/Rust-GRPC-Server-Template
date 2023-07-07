pub mod store;
use tonic::{Request, Response, Status};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use authorization::{
    LoginRequest, 
    LoginResponse,
    SignupRequest,
    SignupResponse,
    authorization_server::Authorization
};
use store::DBClient;

pub mod authorization {
    tonic::include_proto!("authorization");
}

#[derive(Debug)]
pub struct AuthorizationService {
    pub db_client: DBClient,
}

impl AuthorizationService {
    pub fn new(db_client: DBClient) -> Self {
        AuthorizationService { db_client }
    }
}

#[tonic::async_trait]
impl Authorization for AuthorizationService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);
        let LoginRequest { email, password } = request.into_inner();
        
        let result = self.db_client.login(
            email,
        ).await;
        match result {
            Ok(result) => {
                let db_password: String = result.get(1);

                let parsed_hash = match PasswordHash::new(&db_password){
                    Ok(h) => h,
                    Err(e) => return Err(Status::internal(format!("Error hashing password: {}", e))),
                };

                let valid = Argon2::default().verify_password(&password.as_bytes(), &parsed_hash);
                
                match valid {
                    Ok(_) => {
                        let reply = authorization::LoginResponse {
                            account_id: result.get(0),
                            status: true,
                        };
                        
                        return Ok(Response::new(reply))
                    },

                    Err(_) => {
                        let reply = authorization::LoginResponse {
                            account_id: 0,
                            status: false,
                        };
                
                        return Ok(Response::new(reply))
                    },
                }
        
            }
            Err(err) => {
                eprintln!("Error fetching rows: {}", err);
            }
        }

        let reply = authorization::LoginResponse {
            account_id: 0,
            status: false,
        };

        Ok(Response::new(reply))
    }

    async fn signup(
        &self,
        request: Request<SignupRequest>,
    ) -> Result<Response<SignupResponse>, Status> {
        println!("Got a request: {:?}", request);

        let SignupRequest { email, password } = request.into_inner();
        let account_exists = self.validate_email(&email).await;
        if account_exists {
            let reply = authorization::SignupResponse {
                message: format!("That email is already linked to an account.").into(),
            };
            return Ok(Response::new(reply))
        }

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = match argon2.hash_password(&password.as_bytes(), &salt) {
            Ok(h) => h.to_string(),
            Err(e) => return Err(Status::internal(format!("Error hashing password: {}", e))),
        };

        let result = self.db_client.signup(
            email,
            hashed_password,
        ).await;
        match result {
            Ok(_) => {
                let reply = authorization::SignupResponse {
                    message: format!("Your account has been created!").into(),
                };
                
                return Ok(Response::new(reply))
            }
            Err(err) => {
                let reply = authorization::SignupResponse {
                    message: format!("Error fetching rows: {}", err).into(),
                };
                
                return Ok(Response::new(reply))
            }
        }
    }
}

impl AuthorizationService {
    async fn validate_email(
        &self,
        email: &String,
    ) -> bool {
        let result = self.db_client.get_account_by_email(
            email,
        ).await;
        match result {
            Ok(result) => {
                let exists: bool = result.get(0);
                return exists
        
            }
            Err(err) => {
                eprintln!("Error fetching rows: {}", err);
            }
        }

        return false
    }
}
