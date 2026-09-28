/**
  This code for basic knowledge if rust variable by default is immutable!
  To set value or change value, check [./mut-var.rs](./src/mut-var.rs)
**/
fn main() {
  // Normal run
  let main_variable = "Hii!";
  println!("Say {}", main_variable);

  // Become a error
  main_variable = "Hello world!";
  println!("Say {}", main_variable);
}
