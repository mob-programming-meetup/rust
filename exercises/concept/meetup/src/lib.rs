
// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]


pub fn roman_to_arabic_digit(c: char) -> u32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        _ => 0
    }
}


////////////////////////////////////////
// everytime we find I, we decrement one
////////////////////////////////////////
// if we find V or X, we increment by 5 or 10 accordingly
// afterwards, for each I we increment by 1
pub fn roman_to_arabic(roman_value: String) -> u32 {
    let mut sum = 0;
    let mut previous_digit = 0;

   for c in roman_value.chars() {
       let digit = roman_to_arabic_digit(c);
       sum += digit;
       if previous_digit < digit {
           sum -= 2 * previous_digit;
       }
   }

   sum
}

