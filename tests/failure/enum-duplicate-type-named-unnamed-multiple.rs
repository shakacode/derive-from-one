use derive_from_one::FromOne;

#[derive(FromOne)]
pub enum T {
    A(u32),
    B { b: &'static str },
    C { foo: bool, bar: u32 },
}

fn main() {
    let _: T = T::from(1);
}
