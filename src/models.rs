use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub pseudo: String,
  pub password: String,
  pub activated: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub pseudo: String,
  pub password: String,
  pub activated: bool,
}

impl User {
  pub fn get_user(id: i32, connection: &PgConnection) -> Vec<User> {
    all_users
      .find(id)
      .load::<User>(connection)
      .expect("Error during user loading")
  }

  pub fn get_users(connection: &PgConnection) -> Vec<User> {
    all_users
      .order(users::id.desc())
      .load::<User>(connection)
      .expect("Error during users loading")
  }

  pub fn update(id: i32, connection: &PgConnection, user: NewUser) -> bool {
    use super::schema::users::dsl::{
      activated as a, email as e, first_name as f_n, last_name as l_n, password as pw, pseudo as p,
    };
    let NewUser {
      first_name,
      last_name,
      email,
      pseudo,
      password,
      activated,
    } = user;

    diesel::update(all_users.find(id))
      .set((
        f_n.eq(first_name),
        l_n.eq(last_name),
        e.eq(email),
        a.eq(activated),
        p.eq(pseudo),
        pw.eq(password),
      ))
      .get_result::<User>(connection)
      .is_ok()
  }

  pub fn insert(user: NewUser, connection: &PgConnection) -> bool {
    diesel::insert_into(users::table)
      .values(&user)
      .execute(connection)
      .is_ok()
  }

  pub fn remove(id: i32, user: User, connection: &PgConnection) -> bool {
    if User::get_user(id, connection).is_empty() {
      return false;
    }
    diesel::delete(all_users.find(id))
      .execute(connection)
      .is_ok()
  }
}
