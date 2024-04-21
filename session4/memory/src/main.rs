/// # Safety
///
/// This is function is unsafe because .....
unsafe fn my_fn() {}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    println!("my_vec {:#?}", my_vec.get(2));

    unsafe {
        if let Some(value) = my_vec.get(2) {
            println!("yes you can get index {}", value)
        }
    }

    let my_vec_in_case_error = Vec::<u32>::new();

    unsafe {
        // in case  get_unchecked required unsafe  operator
        let get_index = my_vec.get_unchecked(2);
    }
}
