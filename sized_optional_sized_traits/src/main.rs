

use std::fmt::Debug;

use negative_impl::negative_impl;


struct ABC;

#[negative_impl]

impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}


// #[negative_impl]
// impl !Sized for ABC {}

fn some_fn<T: ?Sized>(t:&T){}


// ?sized and Generic Parameter     

// Rules
// 1. must have a single unsized field
// 2. the unsized field must be the last field

struct UnsizedStruct<T: ?Sized> {
   sized_field_1: i32,
   unsized_fied: T,
}

fn print_fn<T: Debug + ?Sized >(t: &T)

{
   println!("{:?}", t)
}
fn main() {
   let x = UnsizedStruct {
      sized_field_1: 3,
      unsized_fied: [3],
   };

   let x = "my name";
   print_fn(&x);
}
