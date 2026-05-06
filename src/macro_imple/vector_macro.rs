#[macro_export]
macro_rules! my_vec {
    // macro is made with one or more rules
    // Each rule looks like a match statement arm: (pattern) => { code to generate };

    // The $ tells Rust, "This is a macro variable." We are naming it element

    //:expr: This is called a designator. It tells the compiler what kind of Rust code to expect.
    // In this case, we are saying that the macro will take an expression as an argument. An expression can be a literal, a variable, a function call, etc.
    ($( $element:expr ),*) => {{
        let mut temp_vec = Vec::new();
        //use $( ... )* inside the generated code block, it unrolls and duplicates that code for every item it caught
       $(
                temp_vec.push($element);
            )*
        temp_vec
    }};
}
