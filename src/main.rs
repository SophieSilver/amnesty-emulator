fn a(_n: i32) {}

trait Foo {
    fn f(&self);
}

impl<F: Fn(i32)> Foo for F {
    fn f(&self) {
        todo!()
    }
}

fn main() {
    a.f();
    // println!("{:?}", ram);
}
