use crate::errors::DbError;
use crate::handlers::login::User;
use crate::StaticState;
use axum::async_trait;
use chrono::Utc;
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
    async fn add_user(&self, user: User) -> Result<User, DbError>;
    async fn add_user_auth(&self, user: User, token: String) -> Result<(), DbError>;
    async fn auth_user(&self, token: String) -> Result<User, DbError>;
}

#[async_trait]
impl DbService for PgConn {
    fn state(&self) -> StaticState {
        self.state.clone()
    }
    async fn auth_user(&self, token: String) -> Result<User, DbError> {
        let res = sqlx::query!("SELECT * FROM session WHERE token = $1", token)
            .fetch_one(&self.conn)
            .await?;
        let time = res.time;
        if time + 20 * 60 * 1000 < Utc::now().timestamp() {
            return match sqlx::query!("DELETE FROM session WHERE token = $1", token)
                .execute(&self.conn)
                .await
            {
                Ok(_) => Err(DbError::TokenStale),
                Err(e) => Err(DbError::Error(e)),
            };
        }
        let user_id = res.userid;
        let user = match sqlx::query!("SELECT * FROM usertable WHERE  userid  = $1", user_id)
            .fetch_one(&self.conn)
            .await
        {
            Ok(user) => user,
            Err(e) => return Err(DbError::Error(e)),
        };
        let name = match user.username {
            Some(name) => name,
            None => return Err(DbError::NoResult),
        };
        let pass = match user.password {
            Some(pass) => pass,
            None => return Err(DbError::NoResult),
        };
        let key = match user.key {
            Some(key) => key,
            None => return Err(DbError::NoResult),
        };
        let new_user = User {
            name,
            pass,
            key,
            user_id: user.userid,
        };
        return Ok(new_user);
    }
    async fn add_user_auth(&self, user: User, token: String) -> Result<(), DbError> {
        let now = Utc::now();
        let ts = now.timestamp();
        match sqlx::query!(
            "UPDATE session SET token = $1, time = $2  WHERE  userid  = $3",
            token,
            ts,
            user.user_id
        )
        .execute(&self.conn)
        .await
        {
            Err(e) => Err(DbError::Error(e)),
            Ok(i) => {
                let rows = i.rows_affected();
                if  rows < 1  {
                    match sqlx::query!(
                        "INSERT INTO session (token,time,userid)  VALUES($1,$2,$3)",
                        token,
                        ts,
                        user.user_id
                    )
                    .execute(&self.conn)
                    .await
                    {
                        Err(e) => Err(DbError::Error(e)),
                        Ok(_) => Ok(()),
                    }
                }else{
                    Ok(())
                }
            }
        }
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
        let new_user = User {
            name,
            pass,
            key,
            user_id: res.userid,
        };

        return Ok(new_user);
    }
    async fn add_user(&self, user: User) -> Result<User, DbError> {
        let user_id = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query!(
            "INSERT INTO usertable (username,password,key,userid ) VALUES($1,$2,$3,$4)",
            &user.name,
            &user.pass,
            &user.key,
            &user_id
        )
        .execute(&self.conn)
        .await?;
        return Ok(User {
            user_id,
            name: user.name,
            pass: user.pass,
            key: user.key,
        });
    }
}
