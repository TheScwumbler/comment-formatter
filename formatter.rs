/********************************************
 * Comment Formatter                        *
 * ---------------------------------------- *
 * this program converts your input to visu *
 * ally appealing comments to put at the to *
 * p of your program                        *
 *                                          *
 * it reduces/eliminates the hassle of mana *
 * ging newlines while making the border lo *
 * ok good                                  *
 *                                          *
 * remember the distinction between lines o *
 * f user input and lines that it needs to  *
 * print out. that gets confusing in the co *
 * de sometimes                             *
 *                                          *
 * hopefully it works well and hopefully yo *
 * u enjoy                                  *
 *                                          *
 *                                          *
 * Written in Rust (Oct, 2022)              *
 *                                          *
 *                                          *
 ********************************************/

// implement Write for stdout handle, allowing flush()
use std::io::Write;

// prompt the user for input
// returns whatever the user typed
fn prompt(prompt: &str) -> String {
	print!("{}", prompt);
	std::io::stdout().flush().expect("Cound not flush stdout...");

	let mut buf: String = String::from("");

	std::io::stdin().read_line(&mut buf).unwrap();

	buf
}

// prompt user input, which gets stored in the line string
// returns the length
// i believe this is every so slightly more efficient
fn prompt_dest(prompt: &str, line: &mut String) -> usize {
	print!("{}", prompt);
	std::io::stdout().flush().expect("Could not flush stdout...");

	std::io::stdin().read_line(line).unwrap()
}

fn main() {
	// create vec to store all user input
	let mut vec: Vec<String> = Vec::new();

	// ask for width (this is the max # of characters of user input, and excluding the * at the beginning and end of each line of the comment)
	let strwidth = prompt("Width of comment box (not including borders): ");

	let width: usize;

	// type check input string
	match strwidth.trim().parse::<usize>() {
		Ok(i) => width = i,
		Err(_) => panic!("Please input integer\n"),
	}

	// ask for title
	vec.push(String::from(prompt("Program Title: ").trim()));

	// prompt the user and push to vec until they hit enter twice in a row
	// pushing to vec lags behind by one so that we do not enter unnecessary newlines at the end
	
	let mut last: String = String::from("\n");
	let mut line: String = String::from("\n");

	// im not entirely sure how i got this to work
	// like i know how i did it but im not sure if i can do anything to make it more efficient without breaking it
	loop {
		
		prompt_dest("Comment Content: ", &mut line);

		// effectively do-while loop
		if last == "\n" && line == "\n" { break; }

		vec.push(String::from(line.to_owned().trim()));

		last.clear();
		last = line.to_owned();

		line.clear();
	}

	// iterate through the vector and format it while printing
	// this is a stupid thing to have to remind myself of, but there is no need to flush() here
	
	// first line
	print!("/"); 
	print!("{}\n", "*".repeat(width + 4));

	// print program title
	let title: String = vec.first().unwrap().to_owned();
	println!(" * {}{} *", title, " ".repeat(width - title.len()));

	// print title bottom border
	println!(" * {} *", "-".repeat(width));

	// print body
	// i have no plans to rewrite this even though it sucks and is super convoluted 

	let mut snum: i32 = 0;
	
	for s in vec {
		snum += 1;
		if snum == 1 {
			continue;
		}
		
		// not sure if converting to f32, rounding, and then converting back into i32 is the only way to ensure that it rounds up but its what im going with for now
		let linesperline = (s.len() as f32 / width as f32).ceil() as i32;

		// start and end pos of the slices 
		// helps with error prevention
		let mut poss = 0;
		let mut pose = width;

		// for each line that we have to print per line of user input
		for i in 0..linesperline {
			let slice: String;
			
			// if we are on the last line
			if i == linesperline - 1 {
				slice = s[poss..].to_owned();
			} else {
				slice = s[poss..pose].to_owned();
			}
			
			println!(" * {}{} *", slice, " ".repeat(width - slice.len()));

			poss += width;
			pose += width;
		}

		// extra line between paragraphs
		println!(" * {} *", " ".repeat(width));
	}

	// last line
	print!(" {}/\n", "*".repeat(width + 4));
	
}
