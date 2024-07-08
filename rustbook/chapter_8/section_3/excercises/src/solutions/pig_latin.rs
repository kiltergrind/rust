#![allow(dead_code)]


/**
 * Convert strings to pig latin.
 * The first consonant of each word is moved to the end of the word and “ay” is added,
 * so “first” becomes “irst-fay”
 * Words that start with a vowel have “hay” added to the end instead
 * (“apple” becomes “apple-hay”).
 * Keep in mind the details about UTF-8 encoding!
 */



static VOWELS: [u8; 6] = *b"aeiouy";


fn convert_into_pig(string: &str) -> Result<String, String> {
    if string.len() < 1 {
        return Err(String::from("String is empty!"));
    }

    let first_letter = string.bytes().next().unwrap().to_ascii_lowercase();
    match first_letter {
        (b'a' ..= b'z') => {
            for ch in VOWELS {
                if ch == first_letter {
                    return Ok(format!("{}-hay", &string[..]));
                }
            }
            return Ok(format!("{}-{}ay", &string[1..], &string[0..1]));
        }
        _ => Err(String::from("ONLY LATIN First-letter!")),
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_pig_consonant() {
        let string = String::from("first");
        let result = convert_into_pig(&string);
        
        assert_eq!(result, Ok(String::from("irst-fay")));
    }

    #[test]
    fn test_pig_vowel() {
        let string1 = String::from("apple");
        let string2 = String::from("a");

        let result1 = convert_into_pig(&string1);
        let result2 = convert_into_pig(&string2);
        
        assert_eq!(result1, Ok(String::from("apple-hay")));
        assert_eq!(result2, Ok(String::from("a-hay")));
    }

    #[test]
    fn test_pig_str_length() {
        let string = String::new();
        let result = convert_into_pig(&string);
        
        assert_eq!(result, Err(String::from("String is empty!")));
    }

    #[test]
    fn test_pig_str_non_ascii_first() {
        let string1 = String::from("Ýnormal");
        let string2 = String::from("Ý");

        let result1 = convert_into_pig(&string1);
        let result2 = convert_into_pig(&string2);
        
        assert_eq!(result1, Err(String::from("ONLY LATIN First-letter!")));
        assert_eq!(result2, Err(String::from("ONLY LATIN First-letter!")));
    }
}