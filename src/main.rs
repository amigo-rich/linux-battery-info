use linux_battery_info::run;

fn main() {
    let laptop_battery = run().unwrap();
    println!("{}", laptop_battery);
}
