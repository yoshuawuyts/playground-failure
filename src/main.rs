extern crate failure;
extern crate lib;

fn main() {
  match lib::hello() {
    Ok(_) => println!("sup"),
    Err(err) => println!("{}", err),
  }
}
