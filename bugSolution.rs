fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    match vec.get(index) {
        Some(element) => println!("Element at index {}: {}", index, element),
        None => println!("Index {} is out of bounds", index),
    }
}