use ranlux_rs;

fn main() {
    println!("{}", std::mem::size_of::<ranlux_rs::ranlx_full_word::SeedRanlxFullWorld32>());
    let mut seed = ranlux_rs::ranlx_full_word::SeedRanlxFullWorld32::default();
    println!("{:?}", seed.as_mut().len());
    println!("{:?}", seed.as_mut());
    seed.as_mut()[64] = 3;
    println!("{:?}", seed.as_mut());
    println!("{:?}", seed);
}
