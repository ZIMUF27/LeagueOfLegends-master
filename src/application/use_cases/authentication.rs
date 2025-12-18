use anyhow::Result;
use cookie::time::error;
use std::sync::Arc;
pub struct AuthenticationUseCase <T> 
where T:BrawlerRepository + Send + Sync ,

{brawler_repository: Arc<T>}

impl<T> AuthenticationUseCase<T> 
where T:BrawlerRepository + Send + Sync
{
 pub fn new(brawler_repository: Arc<T>) -> Self{
    Self { brawler_repository }
 }

 pub async fn login(&self,login_model:LoginModel) -> Result<Passport>{
    let secret_env: String = String = get_user_secret()?;
    let username: String = login_model.username.clone();
    let password: String = login_model.password; 
    //find this user in database
    let user : Result<BrawlerEntity , error> = self.brawler_repository.find_by_username(&username).await?;
    let hashed_password: String = user.password;

    if !argon2::verify(login_model.password, hashed_password?){
        return Err(anyhow::anyhow!("Invalid password"));
    }

    let claims :Claims = Claims {
        sub: user.id,
        exp: (Utc::now() + Duration::days(3)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };
    let token : String = generate_token(&secret_env, &claims);

    Ok(Passport {
        access_token: token,
    })  
 }
}