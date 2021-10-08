fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;

    // READY_AMOUNT = 3;

    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    let _un_used_var: i32;

    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready);
}
