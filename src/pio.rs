use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;


pub fn readline() -> String {
	// get answer
	let mut answer = String::new();
	io::stdin().read_line(&mut answer)
		.expect("Failed to read line");

	// remove newline that's added by the console
	return answer[..answer.len()-1].to_string();

}

pub fn read_data(data_file: &str) -> Vec<(String, u32)> {
	// output
	let mut data: Vec<(String, u32)> = Vec::new();

	// open file
	let result = fs::read_to_string(data_file);
	let mut contents: String;
	match result {
		Ok(cont)=> { contents = cont; },
		Err(e)=> {
		   println!("file {} not found \n{:?}", data_file, e);
		   process::exit(1);
		}
	 }

	// clean up 
	contents = contents.replace(" ", "");
	contents = contents.replace('"', "");
	contents = contents.replace('\u{feff}', "");
	contents = contents.replace('*', "");

	// parse lines
	let lines = contents.split("\n");
	for line in lines { 
		// skip empty lines and comments
		if line == "" { continue; }
		if &line[0..1] == "[" { continue; }
		if line == "Category;Aantal" { continue; }

		// convert line to tuple
		let parts: Vec<&str> = line.split(";").collect();
		let city = parts[0].to_owned();
		let amount = parts[1].parse().expect(&format!("Could not parse '{}' as u32", parts[1]));
		
		// add to data output
		data.push((city, amount));
	}

	return data
}

pub fn write_new_tracked_file(data_file: &str, cities: Vec<super::strct::City>) {
	// create string
	let mut output = "".to_string();
	for city in cities {
		match city.changed {
			true => output += &format!("*\"{}\";{}\n", city.name, city.amount),
			false => output += &format!("\"{}\";{}\n", city.name, city.amount)
		}
	}

	// create new file
	let mut file = std::fs::File::create(data_file).expect("create failed");
	file.write_all(output.as_bytes()).expect("write failed");

}