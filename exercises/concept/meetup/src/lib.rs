
// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn roman_to_arabic(roman_value: String) -> u32 {
    //unimplemented!()

   use std::collections::HashMap as Map;

   let mut mapping = Map::new();
   mapping.insert(String::from("I"), 1);
   mapping.insert(String::from("V"), 5);


   *(mapping.get(&roman_value).unwrap())
}
