fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}

fn main() {
    let string_list = vec![String::from("alembic"), String::from("crucible")];

    let result = largest(&string_list);
    println!("The largest string is {}", result);
}
