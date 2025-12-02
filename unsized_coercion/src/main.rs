


fn str_slice_fn(s: &str) {}

fn array_slice<T>(s: &[T]) {}

trait Some_Traits {
    fn method(&self);
}


impl <T> Some_Traits for [T] {
    fn method(&self) {
        // 1.) ant &[T]
        // 2.) Vec<T>
        // 3.) [T; N]
    }
}

fn main() {
    let some_string =String::from("STring");
    str_slice_fn(&some_string);
    

    //unsized coercion

    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1,2,3,4,5];

    array_slice(slice);
    array_slice(&vec);    //deref coercion
    array_slice(&array);  //unsized coercion
    
        // calling the method fn

        slice.method();
        vec.method();   // deref coercion
        array.method(); // unsized coercion
}
