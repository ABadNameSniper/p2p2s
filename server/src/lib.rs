pub mod tables {
    use std::error::Error;

    use argon2::{
        password_hash, 
        Argon2
    };
    use password_hash::{SaltString, PasswordHasher};
    use sqlx::{
        Row, 
        FromRow, 
        PgPool
    };
    

    #[derive(Debug, FromRow)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub clique_ids: Vec<i32>,
        pub posessed_file_ids: Vec<i32>,
        pub salt: SaltString,
        pub hashbrown: String,
    }
    impl User {
        pub async fn get_one(pool: &PgPool, user_id: i32) -> Result<User, Box<dyn Error>> {
            let q = "SELECT id, name, clique_ids, posessed_file_ids, salt, hashbrown FROM users WHERE id = $1";
            let query = sqlx::query(q)
                .bind(user_id);
        
            let row = query.fetch_one(pool).await?;
        
            Ok(User { 
                id: row.get("id"), 
                name: row.get("name"), 
                clique_ids: match row.try_get("clique_ids") {
                    Ok(vec) => vec,
                    Err(_) => Vec::new(),
                },
                posessed_file_ids: match row.try_get("posessed_file_ids") {
                    Ok(vec) => vec,
                    Err(_) => Vec::new(),
                },
                salt: SaltString::from_b64(row.get::<&str, _>("salt")).unwrap(),//remove unwrap later
                hashbrown: row.get("hashbrown") 
            })
        }

        pub async fn add_new (pool: &PgPool, username: String, password: String) -> Result<User, Box<dyn Error>> {
            let clique_ids = Vec::new();
            let posessed_file_ids = Vec::new();
            let salt = SaltString::generate(rand::thread_rng());
            let argon2 = Argon2::default();
            let hashbrown = argon2.hash_password(password.as_bytes(), &salt).unwrap().hash.unwrap().to_string();

            let query = "INSERT INTO users (name, clique_ids, posessed_file_ids, salt, hashbrown) VALUES ($1, $2, $3, $4, $5) RETURNING id";

            let id: i32 = sqlx::query_scalar(query)
                .bind(&username)
                .bind(&clique_ids)
                .bind(&posessed_file_ids)
                .bind(salt.as_str())
                .bind(&hashbrown)
                .fetch_one(pool)
                .await?;

            Ok(User {
                id,
                name: username,
                clique_ids,
                posessed_file_ids,
                salt,
                hashbrown
            })
        
        }

        pub fn default_user() -> User {
            User { 
                id: 1, 
                name: String::from("John Doe"), 
                clique_ids: vec![], 
                posessed_file_ids: vec![], 
                salt: SaltString::from_b64("saltySalt").unwrap(), 
                hashbrown: String::from("potato-y goodness")
            }
        }


        pub fn get_ids(&self, column_name: &str) -> &Vec<i32> {
            match column_name {
                "clique_ids" => &self.clique_ids,
                "posessed_file_ids" => &self.posessed_file_ids,
                other => panic!("{other} is an invalid column name!", )
            }
        }

        pub fn set_column_values(mut self, column_name: &str, column_val: Vec<i32>) -> () {
            match column_name {
                "clique_ids" => self.clique_ids = column_val,
                "posessed_file_ids" => self.posessed_file_ids = column_val,
                other => panic!("{other} is an invalid column name!", )
            };
        }

        pub async fn push_id(&mut self, pool: &PgPool, column_name: &str, id: i32) -> Result<(), Box<dyn Error>> {
            let query: String = format!("UPDATE users SET {} = $1 WHERE id = $2", column_name);

            println!("{:?}, {:?}", self.clique_ids, self.posessed_file_ids);

            sqlx::query(query.as_str())
                .bind(match column_name {
                    "clique_ids" => {
                        self.clique_ids.push(id);
                        self.clique_ids.clone()
                    },
                    "posessed_file_ids" => {
                        self.posessed_file_ids.push(id);
                        self.posessed_file_ids.clone()
                    },
                    other => panic!("{other} is an invalid column name!")
                })
                .bind(self.id)
                .execute(pool)
                .await?;

            Ok(())
        }

        

    }

    #[derive(Debug)]
    pub struct Metadata {
        _id: i32,
        _hash: [u8; 32],
        _sender: User,
        _posessors: Vec<User>,
    }

    #[derive(Debug)]
    struct Clique {
        _id: i32,
        _users: Vec<User>,
        _metadatas: Vec<Metadata>,
    }

}
    
#[cfg(test)]
mod tests {
    use sqlx::postgres as pg;
    use std::str::FromStr;
    use crate::tables::User;

    #[tokio::test]
    async fn setup() {
        let options = pg::PgConnectOptions::from_str(
            "postgres://postgres:7c25im2XgEJkz6fH@localhost:5432"
        ).unwrap();
    
        let pool = pg::PgPool::connect_with(options).await.unwrap();

        let user = User::default_user();

        let bad_password = String::from("1234");

        let mut user = User::add_new(&pool, user.name, bad_password.clone()).await.unwrap();

        let old_hashbrown = user.hashbrown.clone();

        assert_ne!(bad_password, old_hashbrown);

        user.push_id(&pool, "clique_ids", 3).await.unwrap();
        user.push_id(&pool, "posessed_file_ids", 5).await.unwrap();
        user.push_id(&pool, "clique_ids", 0).await.unwrap();

        assert_eq!(user.clique_ids, vec![3, 0]);
        assert_eq!(user.posessed_file_ids, vec![5]);

        let user = User::get_one(&pool, user.id).await.unwrap();

        assert_eq!(user.hashbrown, old_hashbrown);

    }

}
