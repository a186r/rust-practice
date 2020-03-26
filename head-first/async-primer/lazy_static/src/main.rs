use lazy_static::*;

lazy_static!{
    static ref A: u8 = 42;
}

fn main() {
    println!("{}", *A);
}