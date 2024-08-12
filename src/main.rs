use error::Error;
use prisma::{user, PrismaClient, Role};

mod error;
#[allow(warnings)]
mod prisma;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to database");

    let new_user = client
        .user()
        .create(
            "Wander".into(), 
            "a@a.com".into(), 
            "secret".into(), 
            vec![user::role::set(Role::Admin)]
        )
        .exec()
        .await; 
    
    println!("{:#?}", new_user);
    Ok(())
}
