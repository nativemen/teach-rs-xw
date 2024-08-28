fn main() {
    let mut input = [23, 82, 16, 45, 21, 94, 12, 34];

    input.sort_unstable();

    println!(
        "{} is largest and {} is smallest",
        input.last().unwrap(),
        input.first().unwrap()
    );
}
