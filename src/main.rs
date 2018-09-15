fn main() {
    let x = (500, 6.4, true);

    println!("This is true or false: {}", x.2);

    let t = [1, 3, 4, 6];

    let first = t[0];

    this_function(first);
}

fn this_function(a: i32){
    println!("This is the first array index: {}", a);
}
