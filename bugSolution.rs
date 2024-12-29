fn main() {
    let mut v = vec![1, 2, 3];
    let first_element = v.get_mut(0);
    if let Some(elem) = first_element {
        *elem = 10; // Safe modification
    } 
    println!("The first element is: {}", v[0]);
}