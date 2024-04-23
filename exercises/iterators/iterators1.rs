// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // Step 1: Initialize the iterator

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));  // Step 2: Check the first value
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));  // Step 3: Check the second value
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));  // Step 4: Check the third value
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));  // Step 5: Check the fourth value
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));  // Step 6: Check the fifth value
}
