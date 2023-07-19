use sha256::digest;


pub mod tables {
    use std::fmt::format;

    use sha256::digest;


    use tokio_postgres::Row;

    use crate::auth::make_salt;
    
    // I see redundant code... it looks like I could make these with a macro more easily?

    #[derive(Debug)]
    pub struct User {//All options because they won't be there if I don't query for them
        id: Option<i32>,
        name: Option<String>,
        clique_id: Option<i32>, //change to Vec later?
        posessed_files: Option<Vec<Metadata>>,
        salt: Option<String>,
        hashbrown: Option<String>,
    }

    #[derive(Debug)]
    struct Metadata {
        hash: Option<[u8; 32]>,
        sender: Option<User>,
        posessors: Option<Vec<User>>,
    }

    #[derive(Debug)]
    struct Clique {
        id: Option<i32>,
        users: Option<Vec<User>>,
        metadatas: Option<Vec<Metadata>>,
    }

    impl From<Row> for User {
        fn from(row: Row) -> Self {
            Self {//but what if i'm not getting all the columns?
                id: row.try_get("id").unwrap_or(None),
                name: row.try_get("name").unwrap_or(None),
                clique_id: row.try_get("clique_id").unwrap_or(None),
                posessed_files: Some(vec![]),
                salt: row.try_get("salt").unwrap_or(None),
                hashbrown: row.try_get("hashbrown").unwrap_or(None),
            }
        }
    }

    impl User {
        fn new(id: i32, name: String, password: String) -> User {
            let salt = make_salt(4);
            let salted_password = format!("{}{}", password, salt);
            let hashbrown = digest(salted_password);
            User {
                id: Some(id),
                name: Some(name),
                clique_id: None,
                posessed_files: None,
                salt: Some(salt),
                hashbrown: Some(hashbrown),

            }
        }
    }

}

mod auth {
    //Yes, I know I could find a crate for it.
    //I felt like doing it myself.
    
    //NOTE: change to Argon2 lol

    use rand::*;

    pub fn make_salt(half_length: usize) -> String {
        //I have decided to do this with hex codes for a readable, UTF8 output.
        //Sure, you could store the salt as an integer, though
        let mut salt = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..half_length {
            let random_chars = format!("{:X}", rng.gen::<u8>());
            salt.push_str(&random_chars);
        }

        salt
    }
}
    
#[cfg(test)]
mod tests {
    use crate::auth::make_salt;
    use sha256::digest;


    fn default_password() -> String {
        String::from("superSecretPassword1")
    }

    #[test]
    fn salted_password() {
        let original_password = default_password();
        let mut password = original_password.clone();
        let half_length = 2;
        password.push_str(&make_salt(half_length));

        println!("Original: {}, New: {}", &original_password, password);

        assert_eq!(original_password.len() + half_length * 2, password.len());
    }

    #[test]
    fn secure_password() {
        let original_password = default_password();
        let mut salted_password = original_password.clone();
        salted_password.push_str(&make_salt(2));
        let hashbrown = digest(salted_password.clone());
        
        println!("{}, {}, {}", &original_password, &salted_password, &hashbrown);
        assert_ne!(&original_password, &salted_password);
        assert_ne!(&salted_password, &hashbrown);
    }
}
