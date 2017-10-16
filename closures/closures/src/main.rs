use 

fn main() {
	fn generate_workout(intensity: i32, random_number: i32){

		let expensive_result = simulated_expensive_calculation(intensity);

		let expensive_closure = |num| {
			println!("calculating slowly...");
			thread::sleep(Duration::from_secs(2));
			num
		};

		if intensity < 25 {
			println!("Today do {} pushups!", expensive_result);
			println!("Then do {} situps!", expensive_result);
		}
		else {
		    if random_number == 3 {
		    	println!("Rest today! Stay Hydrated!");
		    }
		    else {
		    	println!("Today run for {} minutes", expensive_result);
		    }
		}

	}
}
