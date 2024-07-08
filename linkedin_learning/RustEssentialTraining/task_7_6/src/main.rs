//! Perform a string trimming
//!
//! Double-side string whitespace trimming.
//! Only '\s' whitespace character is considered for now
//! 

fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */

/*
/// SOLUTION USING `while` loop
/// 
fn trim_spaces(input: &str) -> &str {
    if input.is_empty() {
        return input;
    }
    
    let mut first_sym_pos: usize = 0;
    let mut last_sym_pos: usize = input.len();
    let mut first_met = false;
    let mut last_met = false;
    let walker = input.as_bytes();

    while first_sym_pos < last_sym_pos {
        if walker[first_sym_pos] != b' ' {
            first_met = true;
        }
        
        if !first_met { first_sym_pos += 1; }
        if !last_met  { last_sym_pos -= 1;  }

        if walker[last_sym_pos] != b' ' {
            last_met = true;
        }
        
        // When whitespace boundary reached from both sides
        if first_met && last_met {
            break;
        }
    }

    &input[first_sym_pos..last_sym_pos + 1]
}
*/


/*
/// SOLUTION USING MAPPINGS AND `for` loop
///
fn trim_spaces(input: &str) -> &str {
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    let bytes = input.as_bytes();
    let walker = bytes.iter().enumerate().zip(bytes.iter().rev().enumerate());

    for (i, j) in walker {
        if start.is_some() && end.is_some() {
            break;
        }
        if start.is_none() && i.1 != &b' ' {
            start = Some(i.0);
        }
        if end.is_none() && j.1 != &b' ' {
            end = Some(input.len() - j.0);
        }
    }

    &input[start.unwrap_or(0)..end.unwrap_or(0)]
}
*/


/*
/// SOLUTION INVOLVING BITWISE LOGIC
/// 
fn trim_spaces(input: &str) -> &str {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut check: u8 = 0;

    let bytes = input.as_bytes();
    let walker = bytes.iter().enumerate().zip(bytes.iter().rev().enumerate());

    for (i, j) in walker {
        if check ^ 0x3 == 0 {
            break;
        }
        if check & 0x1 == 0 && i.1 != &b' ' {
            start = i.0;
            check |= 0x1;
        }
        if check & 0x2 == 0 && j.1 != &b' ' {
            end = input.len() - j.0;
            check |= 0x2;
        }
    }
 
    &input[start..end]
}
*/



/// SOLUTION BY THE MEANS OF ITERATORS
/// 
fn trim_spaces(input: &str) -> &str {
    let bytes = input.as_bytes();
    let mut start: usize = bytes.iter()
        .position(|&c| c != b' ')
        .unwrap_or(0);
    let mut end: usize = input.len() - bytes.iter().rev()
        .position(|&c| c != b' ')
        .unwrap_or(input.len());

    &input[start..end]
}