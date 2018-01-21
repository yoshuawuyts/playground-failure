#[macro_use]
extern crate failure;

use failure::Error;

pub fn hello() -> Result<String, Error> {
  bail!("We encountered snakes on the plane.");
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
