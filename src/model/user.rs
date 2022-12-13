use crate::db::{
    establish_connection,
    schema::users::{self, dsl::*},
};
use anyhow::{Context, Error};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Queryable, Debug, AsChangeset, Eq, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn create(new_user: NewUser) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(&mut connection)
            .with_context(|| format!("Could not create new user: {new_user:?}"))
    }

    pub fn update(user: User) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        diesel::update(users.find(user.id))
            .set(&user)
            .get_result(&mut connection)
            .with_context(|| format!("Could not update user with id: {:?}", user.id))
    }

    pub fn delete(i: i32) -> Result<usize, Error> {
        let mut connection = establish_connection()?;
        diesel::delete(users.find(i))
            .execute(&mut connection)
            .with_context(|| format!("Could not delete user with id: {i}"))
    }

    pub fn get(i: i32) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        users
            .filter(users::id.eq(i))
            .first(&mut connection)
            .with_context(|| format!("Could not receive user with id : {i}"))
    }

    pub fn get_all() -> Result<Vec<Self>, Error> {
        let mut connection = establish_connection()?;
        users
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
            id: 21,
            name: "otto".to_string(),
            email: "otto@gmail.com".to_string(),
        };
        let json = dbg!(serde_json::to_string(&user).unwrap());
        let test_user: User = serde_json::from_str(&json).unwrap();
        assert_eq!(test_user, user);
    }
}
