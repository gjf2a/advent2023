fn main() {
    let v: Vec<u128> = (0..500_000_000).map(|n: u128| n.pow(2)).collect();
    println!("{}", v.iter().sum::<u128>());
}
