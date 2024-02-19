pub struct AuthService;

impl AuthService {
    pub async fn hash_password(password: &str) -> String {
        bcrypt::hash(password, 12).unwrap()
    }

    pub async fn verify_password(password: &str, hash: &str) -> bool {
        bcrypt::verify(password, hash).unwrap()
    }

    pub async fn generate_token(email: &str) -> String {
        NotImplemented!()
    }

    pub async fn login(email: &str, password: &str)
                       -> Result<String, String>
    {
        NotImplemented!()
    }
}