use linux_battery_info::run;

fn main() {
    let battery_information = match run() {
	Ok(bi) => bi,
	Err(e) => {
	    eprintln!("{}", e);
	    panic!();
	},
    };
    println!("{}", battery_information);
}
