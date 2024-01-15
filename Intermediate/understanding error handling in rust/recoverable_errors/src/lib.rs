//HW 1-3

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
