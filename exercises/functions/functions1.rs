// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn call_me(){
    let tup: (i32, bool) = (5, true);
    let a:[i32;5] = [1,2,3,4,5];
    println!("Hello from the function!");
    println!("{:?}",tup.1);
    println!("{:?}",a[2]);
}

fn main() {
    call_me();
}