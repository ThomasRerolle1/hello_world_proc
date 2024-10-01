use hello_macro_proc::hello_macro;

#[hello_macro]
fn my_function() {
    println!("My function is running!");
}

fn main() {
    my_function();

    hello_world();
}
