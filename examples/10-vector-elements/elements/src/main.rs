fn add_number_to_start_and_end(v: &mut Vec<i32>, number: i32) {
    v.insert(0, number);
    v.push(number);
}

fn append_vector(v: &mut Vec<i32>, other: &mut Vec<i32>) {
    v.append(other);
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
    add_number_to_start_and_end(&mut v, 9);
    println!("{:?}", v); // Output: [9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

    let mut more_numbers = vec![7, 8];
    append_vector(&mut v, &mut more_numbers);
    println!("{:?}", v); // Output: [9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 7, 8] 
    println!("{:?}", other_numbers); // Output: []
}
