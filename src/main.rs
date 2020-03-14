use chrono;
use time::Duration;

mod pio;
mod strct;



fn main() {
	// Environment variables
	let user_name = env!("USER");

	// init 
	let utc_today = chrono::Local::now();
	let utc_yesterday = chrono::Local::now() + Duration::days(-1);

	// config
	let folder = format!("/home/{}/Data/", user_name);
	let today = utc_today.format("%b-%e").to_string();
	let yesterday = utc_yesterday.format("%b-%e").to_string();

	// Set file names
	let data_file_tracked = format!("{}{}-tracked.csv", folder, yesterday).to_string();
	let data_file_total = format!("{}{}-total.csv", folder, yesterday).to_string();
	let data_file_new = format!("{}{}-total.csv", folder, today).to_string();
	let data_file_new_tracked = format!("{}{}-tracked.csv", folder, today).to_string();

	println!("New    : {}, \nOld    : {}, \nTracked: {}\n\n", data_file_new, data_file_total, data_file_tracked);

	// get data
	let data_zh = pio::read_data(&data_file_tracked);
	let data_total = pio::read_data(&data_file_total);
	let data_total_new = pio::read_data(&data_file_new);

	// output for new 'tracked' file
	let mut tracked_list: Vec<strct::City> = vec![];

	for city in data_total_new 
	{
		// check if city is known
		let mut known_amount = 0;
		for c in &data_total {
			if c.0 == city.0 {
				known_amount = c.1;
			}
		}

		// check if city is being tracked
		let mut tracked = false;
		for c in &data_zh {
			if c.0 == city.0 {
				tracked = true;
				break;
			}
		}

		// Create City object
		let mut city_obj = strct::City {
			name: city.0,
			amount: city.1,
			changed: false,
			new: false
		};

		// new untracked city: decide if it needs to be tracked
		if known_amount == 0 {
			println!("- New city found, '{}' with {} confirmed cases.", city_obj.name, city_obj.amount);
			println!("  Do you want to add this city to tracked cities? (y/N)");

			let answer = pio::readline();
			if answer == "y" {
				city_obj.changed = true;
				city_obj.new = true;
				tracked_list.push(city_obj);
				println!("  > Added to tracker\n");
			}
		}

		// tracked city
		else if tracked {
			// data changed: notify
			if known_amount != city.1 {
				city_obj.changed = true;
				tracked_list.push(city_obj);
			}
			else {
				tracked_list.push(city_obj);
			}
		}
	}

	// Print changes
	for city in &tracked_list {
		if city.changed {
			match city.new {
				false => println!("- Confirmed cases in '{}' changed to {}.", city.name, city.amount),
				true =>  println!("- Confirmed cases in '{}' changed to {}. (NEW)", city.name, city.amount)
			}
		}
	}

	pio::write_new_tracked_file(&data_file_new_tracked, tracked_list);
}

