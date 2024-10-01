use hello_macro_proc::hello_macro;
use hello_macro_proc_attr::hello_world_attr;

#[hello_macro]
fn my_function() {
    println!("My function is running!");
}

#[hello_world_attr(name = "Excalibur")]
fn my_other_function() {
    println!("Ny other function is running too!");
}

fn main() {
    my_function();
    my_other_function();
    hello_world();
    hello_world_attr();
}
