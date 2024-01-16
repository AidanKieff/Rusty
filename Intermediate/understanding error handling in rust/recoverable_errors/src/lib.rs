//HW 1-3


/*
//------------------------start of hw1------------------------------
// Rewrite the find_pos function.
// The return type must be Result<usize, Error>.

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyTextOrPattern, // either text or pattern (or both) is empty string
    TextLenSmall,       // text.len() < pattern.len()
    PatternNotFound,    // pattern not a substring of text
}

// below function finds the starting index of `pattern` in `text`
// if `pattern` is not found, it returns text.len()
// pub fn find_pos(text: &str, pattern: &str) -> usize {
//     let pattern_len = pattern.len();
//     for start in 0..text.len() - pattern_len + 1 {
//         if &text[start..start + pattern_len] == pattern {
//             return start;
//         }
//     }
//     text.len()
// }


pub fn find_pos(text: &str, pattern: &str) -> Result<usize, Error> {
    let pattern_len = pattern.len();
    if let true = (pattern_len == 0 || text.len() == 0) {
        return Err(Error::EmptyTextOrPattern)
    }

    if let true = text.len() < pattern_len {
        return Err(Error::TextLenSmall)
    }

    for start in 0..text.len() - pattern_len + 1 {
        if &text[start..start + pattern_len] == pattern {
            return Ok(start);
        }
    }
    Err(Error::PatternNotFound)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strings() {
        assert!(matches!(
            find_pos("", "pattern"),
            Err(Error::EmptyTextOrPattern)
        ));
        assert!(matches!(
            find_pos("text", ""),
            Err(Error::EmptyTextOrPattern)
        ));
        assert!(matches!(find_pos("", ""), Err(Error::EmptyTextOrPattern)));
    }

    #[test]
    fn small_text() {
        assert!(matches!(
            find_pos("hello", "hello there"),
            Err(Error::TextLenSmall)
        ));
    }

    #[test]
    fn pattern_not_present() {
        assert!(matches!(
            find_pos("hello", "bye"),
            Err(Error::PatternNotFound)
        ));
    }

    #[test]
    fn pattern_present() {
        assert!(matches!(find_pos("I luv Rust", "uv"), Ok(3)));
    }
}

*/

/*
//---------------------------------start of hw2-------------------------


pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}

*/

// ------------------------------start of hw 3 -----------------------------------
/*
// Complete the `new` associated function to check for invalid input.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value{
            i64::MIN..=-1 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            _ => Ok(PositiveNonzeroInteger(value as u64))
        }
        
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
*/

