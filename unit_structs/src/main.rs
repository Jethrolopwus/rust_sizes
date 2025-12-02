

// unit structs

struct Admin;
struct User;

trait Authenticate   {
    fn authenticat(&self, username: &str, password: &str) -> bool;
}

impl Authenticate for Admin {
    fn authenticat(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "adminpass"
    }
}

impl Authenticate for User {
    fn authenticat(&self, username: &str, password: &str) -> bool {
        username == "user" && password == "userpass"
    }
}

fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
    role.authenticat(username, password)
}


// Phantom data

use std::{marker::PhantomData, mem::size_of, rc::Rc};

struct ABC{
    ensuring_no_send_sync: PhantomData<Rc<()>>,
}



fn main() {
  let admin = Admin;
  let user = User;
  let admin_login = login(admin, "admin", "adminpass");
  let user_login = login(user, "user", "userpass");

  if admin_login {
    println!("Admin Login successfully");
  } else {
    println!("User Login Successfully");
  }


  // using phantom data to inspect size of the struct

  println!("{}", size_of::<ABC>());
}
