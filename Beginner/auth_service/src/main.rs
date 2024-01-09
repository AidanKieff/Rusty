use auth_service::{Credentials};

fn main() {
    let creds = Credentials {
        username: "letmebe123".to_owned(),
        password: "password".to_owned(),
    };
    auth_service::authenticate(creds);

}