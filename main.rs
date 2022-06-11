use std::io::stdin;


fn count_sub_array(origin_array: Vec<char>,sub_array: Vec<char>) -> i32{
	let n = origin_array.len();
	let m = sub_array.len();
	if n < m{
		return 0;
	}
	// println!("{:?}", origin_array);
	// println!("{:?}", sub_array);
	let mut count:i32 = 0;
	let mut i = 0; let mut j = 0;
	while i < n && j < m{
	if origin_array[i] == sub_array[j]{
	    i += 1;
	    j += 1;
	    if j == m{
			count+=1;
			i = i - j + 1;
	    	j = 0;
	    }
	}
	else{
	    i = i - j + 1;
	    j = 0;
	}
	}      
	return count;
}

fn find_keyword(_input_string: &String,_text_find: &String) -> i32{
	let lower_case_text = _text_find.to_lowercase();
	let lower_case_input = _input_string.to_lowercase();
	let lower_case_text_char_vec: Vec<char> = lower_case_text.chars().collect();
	let lower_case_input_char_vec: Vec<char> = lower_case_input.chars().collect();
	let result = count_sub_array(lower_case_text_char_vec, lower_case_input_char_vec);
	return result;
}

fn main() {
	let mut text_find: String = String::from("https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt");
    let mut input_string = String::new();
    println!("input anything");
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    input_string.pop();
    let result = find_keyword(&input_string, &text_find);
    println!("{input_string} found {result} times in text");

}
