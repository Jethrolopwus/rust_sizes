

//size in Rust
//Sized Types
// Unsized types

use std::mem::size_of;


trait Some_traits {

}

trait Shape {
    fn print(&self);
    
}

#[derive(Debug)]
struct  Circle;

#[derive(Debug)]

struct Rectangle;


impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    // Sized Types
    println!("i32 size is: {}", size_of::<i32>());
    println!("(i32, i32) size is: {}", size_of::<(i32, i32)>());
    println!("[i32: 3] size is: {}", size_of::<[i32; 3]>());

    struct Point {
        x: bool,
        y: i64,
    }

    println!("Struct size is: {}", size_of::<Point>());
    println!("i32 refernece size is: {}", size_of::<&i32>());
    println!("i32  mutable refernece size is: {}", size_of::<&mut i32>());
    println!("Machine Word size is: {}", size_of::<&()>());
    println!("Box<i32> size is: {} ", size_of::<Box<i32>>());
    println!("fn(i32) -> i32 size is: {} ", size_of::<fn(i32) -> i32>());
    

    // Unsized Types

    println!("[i32] size is: {} ", size_of::<&[i32]>());
    let a: [i32; 4];
    // println!("str size is: {} ", size_of::<str>());

    println!("The size of the trait is: {} ", size_of::<&dyn Some_traits>());



    //Pointers to sized and unsized Types 


    println!("Size of a reference to sized types: {} ", size_of::<&[i32; 3]>());

    println!("Size of a reference to unsized types: {} ", size_of::<&[i32]>());

    let num_1:&[i32; 3] = &[10,12,30];
    let num_2: &[i32] = &[10, 12,30];


    let mut sum = 0;

    for num in num_1 {
        sum += num;
    }
     for num in num_2 {
        sum += num;
     }


     // displaying the sizes of the structs and the traits object

     println!("The size of the &Circle is: {} ", size_of::<&Circle>());
     println!("The size of the &Rectangle is: {} ", size_of::<&Rectangle>());
     println!("The size of the &dyn Shape is: {} ", size_of::<&dyn Shape>());



}
