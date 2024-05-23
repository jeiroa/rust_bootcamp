pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    // instead
    pub fn new(_username: String, _password: String) -> Credentials {
        Credentials {
            username: _username,
            password: _password,
        }
    }
}