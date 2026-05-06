mod macro_imple {
    pub mod vector_macro;
}


fn main() {
    let my_vector = crate::my_vec![1, 2, 3, 4, 5];
    println!("{:?}", my_vector);
    println!("Hello, world!");
}
