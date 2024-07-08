#![allow(dead_code)]


// Given a list of integers, use a vector and return the:
// - median (when sorted, the value in the middle position)
// - mode (the value that occurs most often; a hash map will be helpful here) of the list

use std::collections::HashMap;


#[derive(Debug, PartialEq)]
struct Median(i32);

#[derive(Debug, PartialEq)]
struct Mode(i32);


fn get_median_mode_data(list: &Vec<i32>) -> Option<(Median, Mode)> {
    if list.len() < 1 {
        return None
    }


    let mut sorted_list = list.clone();
    sorted_list.sort_unstable();
    
    
    Some((get_median(&sorted_list), get_mode(&sorted_list)))
}


fn get_median(sorted_list: &Vec<i32>) -> Median {
    let arrlen = sorted_list.len();
    let midindex = arrlen / 2;

    Median(match arrlen % 2 {
        0 => sorted_list[ midindex ] + sorted_list[ midindex - 1],
        _ => sorted_list[ midindex ]
    })
}


fn get_mode(sorted_list: &Vec<i32>) -> Mode {
    let mut mode_counters = HashMap::new();
    let mut mode: i32 = sorted_list[0];
    let mut current;
    let mut largest_count: usize = 0;

    for member in sorted_list {
        current = mode_counters.entry(member)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        if *current > largest_count {
            mode = *member;
            largest_count = *current;
        }
    }
    
    Mode(mode)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mode() {
        let array = vec![1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(get_mode(&array), Mode(2));
    }

    #[test]
    fn test_get_median() {
        let array = vec![1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(get_median(&array), Median(4));
    }

    #[test]
    fn test_get_median_mode_data() {
        let array = vec![1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(get_median_mode_data(&array), Some((Median(4), Mode(2))));
    }
}