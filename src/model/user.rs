use crate::db::{
    establish_connection,
    schema::users,
};
use anyhow::{Context, Error};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub pwhash: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset, Eq, PartialEq)]
pub struct User {
    pub email: String,
    pub name: String,
    pub pwhash: String,
}

impl User {
    pub fn create(new_user: NewUser) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut connection)
            .with_context(|| format!("Could not create new user: {new_user:?}"))
    }

    pub fn update(user: User) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        diesel::update(users::table.find(&user.email))
            .set(&user)
            .get_result(&mut connection)
            .with_context(|| format!("Could not update user{:?}", user.email))
    }

    pub fn delete(email: &str) -> Result<usize, Error> {
        let mut connection = establish_connection()?;
        diesel::delete(users::table.find(&email))
            .execute(&mut connection)
            .with_context(|| format!("Could not delete user: {email}"))
    }

    pub fn get(email: &str) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        users::table
            .filter(users::email.eq(email))
            .first(&mut connection)
            .with_context(|| format!("Could not receive user: {email}"))
    }

    pub fn get_all() -> Result<Vec<Self>, Error> {
        let mut connection = establish_connection()?;
        users::table
            .load::<User>(&mut connection)
            .context("Could not receive all users")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialization() {
        let user = User {
            email: "otto@gmail.com".to_string(),
            name: "otto".to_string(),
            pwhash: "1234".to_string(),
        };
        let json = dbg!(serde_json::to_string(&user).unwrap());
        let test_user: User = serde_json::from_str(&json).unwrap();
        assert_eq!(test_user, user);
    }
}
