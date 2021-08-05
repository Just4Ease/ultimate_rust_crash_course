const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 1;

fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);


    println!("Firing {} of my {} missiles...\n", ready, missiles);

    missiles -= ready;
    println!("{} missiles left", missiles);
}
