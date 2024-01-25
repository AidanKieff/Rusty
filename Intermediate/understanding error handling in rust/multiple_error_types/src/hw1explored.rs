// Make the code compile by addressing the TODO.

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() {
    let pretend_user_input = "1";
    let parsed = input_parser(pretend_user_input);
    match parsed {
        Ok(i) => println!("output is {:?}", i),
        Err(e) => println!("Error: {e}")
    };
}

fn input_parser(input: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let x: i64 = input.parse()?;
    let y = PositiveNonzeroInteger::new(x)?;
    Ok(y)
}
// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
