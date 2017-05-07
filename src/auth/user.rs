//Any models used for logging in go here

//Registration
#[derive(FromForm)]
pub struct Register {
    user: String,
    pass: String,
    email: String,
}

impl Register {
    pub fn get_username(&self) -> String {
        return self.user.clone();
    }

    pub fn get_password(&self) -> String {
        return self.pass.clone();
    }

    pub fn get_email(&self) -> String {
        return self.email.clone();
    }
}

//Log in
#[derive(FromForm)]
pub struct Login {
    user: String,
    pass: String,
}

impl Login {
    pub fn get_username(&self) -> String {
        return self.user.clone();
    }

    pub fn get_password(&self) -> String {
        return self.pass.clone();
    }
}