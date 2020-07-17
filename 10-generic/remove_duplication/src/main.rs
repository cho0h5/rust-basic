fn main() {
    let numbers = vec![54, 21, 87, 23, 76, 123, 4];

    let largest = largest(&numbers);
    println!("The largest number is {}", largest);

    let chars = vec!['a', 'g', 'x', 'b'];

    let largest = largest(&chars);
    println!("The largest char is {}", largest);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
