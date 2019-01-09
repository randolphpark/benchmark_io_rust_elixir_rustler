extern crate rust_read;

fn main() {
	let target = [1, 93, 88, 98, 146, 95, 31, 149, 60, 2]; // no match
    println!("{:?} - {:?}", target, rust_read::run(target));

    println!("Hello, world!");
}
