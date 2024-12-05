fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    let element = unsafe { *vec.get_unchecked(index) };
    println!("Element at index {}: {}", index, element);
}