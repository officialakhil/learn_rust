// Define a generic function to increment each element in a collection by n
fn increment_by_n<T>(collection: Vec<T>, n: T) -> Vec<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    collection.iter().map(|&x| x + n).collect()
}

// Define a generic function to filter elements based on a closure's condition
fn filter_by_condition<T, F>(collection: Vec<T>, condition: F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    collection.into_iter().filter(|x| condition(x)).collect()
}

// Define a generic function to transform elements to strings based on a closure
fn transform_to_string<T, F>(collection: Vec<T>, transform: F) -> Vec<String>
where
    F: Fn(T) -> String,
{
    collection.into_iter().map(|x| transform(x)).collect()
}

fn decrement_by_n_and_filter<F>(collection: Vec<i32>, n: i32, filter: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    filter_by_condition(increment_by_n(collection, -1 * n), filter)
}

fn main() {
    // Example usage of data transformation functions
    let numbers = vec![1, 2, 3, 4, 5];

    let incremented = increment_by_n(numbers.clone(), 10);
    println!("Incremented by 10: {:?}", incremented);

    let filtered = filter_by_condition(numbers.clone(), |&x| x % 2 == 0);
    println!("Filtered (even numbers only): {:?}", filtered);

    let to_string = transform_to_string(numbers.clone(), |x| x.to_string());
    println!("Transformed to strings: {:?}", to_string);

    // Example usage of decrement_and_filter function
    let decremented_by_5_odd = decrement_by_n_and_filter(numbers.clone(), 5, |&x| x % 2 != 0);
    println!(
        "Decrement and filter (odd numbers only): {:?}",
        decremented_by_5_odd
    );
}
