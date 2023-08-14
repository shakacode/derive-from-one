use derive_from_one::FromOne;

#[derive(FromOne)]
pub enum T {
    A(u32),
    B(u32),
}

fn main() {
    let _: T = T::from(1);
}
