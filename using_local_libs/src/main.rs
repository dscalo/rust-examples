extern crate point;

use point::Point;

// https://readlnh.github.io/2020/09/22/Rust/local_unpublished_crate_in_Rust/

fn main() {
    let a = Point::new(2,3);
    let b = Point::new(3,4);

    let c = a + b;
    println!("a + b == {} {}", c.x, c.y);
}
