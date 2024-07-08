use crate::errors::DbError;
use crate::handlers::login::AuthedUser;
use crate::handlers::login::User;
use crate::StaticState;
use axum::async_trait;
use dotenv::*;
use sqlx::*;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
pub type DynUserRepo = Arc<dyn DbService + Send + Sync>;
#[derive(Clone)]
pub struct PgConn {
    conn: Pool<Postgres>,
    state: StaticState,
}

pub async fn new_postgres_conn(staticstate: StaticState) -> Result<PgConn, anyhow::Error> {
    if let Ok(mode) = std::env::var("mode") {
        match mode.as_str() {
            "docker" => dotenv::from_filename("run/secrets/db_conn").ok(),
            _ => dotenv().ok(),
        };
    } else {
        dotenv().ok();
    }

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_url)
        .await?;
    Ok(PgConn {
        conn: pool,
        state: staticstate,
    })
}

#[async_trait]
pub trait DbService {
    fn state(&self) -> StaticState;
    async fn get_user(&self, user: User) -> Result<User, DbError>;
    async fn add_user(&self, user: User) -> Result<(), DbError>;
    async fn add_user_auth(&self, user: AuthedUser) -> Result<(), DbError>;
    async fn auth_user(&self, token: String) -> Result<User, DbError>;
}

#[async_trait]
impl DbService for PgConn {
    fn state(&self) -> StaticState {
        self.state.clone()
    }
    async fn auth_user(&self, token: String) -> Result<User, DbError> {
        let res = sqlx::query!("SELECT * FROM usertable WHERE auth = $1", token)
            .fetch_one(&self.conn)
            .await?;
        let name = match res.username {
            Some(name) => name,
            None => return Err(DbError::NoResult),
        };
        let pass = match res.password {
            Some(pass) => pass,
            None => return Err(DbError::NoResult),
        };
        let key = match res.key {
            Some(key) => key,
            None => return Err(DbError::NoResult),
        };
        let new_user = User { name, pass, key };
        return Ok(new_user);
    }
    async fn add_user_auth(&self, user: AuthedUser) -> Result<(), DbError> {
        let _ = sqlx::query!(
            "UPDATE usertable SET auth = $1  WHERE key = $2",
            user.token,
            user.key
        )
        .fetch_one(&self.conn)
        .await?;
        return Ok(());
    }
    async fn get_user(&self, user: User) -> Result<User, DbError> {
        let res = sqlx::query!("SELECT * FROM usertable WHERE username = $1", user.name)
            .fetch_one(&self.conn)
            .await?;
        let name = match res.username {
            Some(name) => name,
            None => return Err(DbError::NoResult),
        };
        let pass = match res.password {
            Some(pass) => pass,
            None => return Err(DbError::NoResult),
        };
        let key = match res.key {
            Some(key) => key,
            None => return Err(DbError::NoResult),
        };
        let new_user = User { name, pass, key };

        return Ok(new_user);
    }
    async fn add_user(&self, user: User) -> Result<(), DbError> {
        let _ = sqlx::query!(
            "INSERT INTO usertable (username,password,key) VALUES($1,$2,$3)",
            user.name,
            user.pass,
            user.key
        )
        .fetch_one(&self.conn)
        .await?;
        return Ok(());
    }
}
