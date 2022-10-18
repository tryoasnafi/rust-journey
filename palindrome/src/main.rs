use std::time::Instant;

fn main() {
    let s: String = String::from("1Dennis, Nell, Edna, Leon, Nedra, Anita, Rolf, Nora, Alice, Carol, Leo, Jane, Reed, Dena, Dale, Basil, Rae, Penny, Lana, Dave, Denny, Lena, Ida, Bernadette, Ben, Ray, Lila, Nina, Jo, Ira, Mara, Sara, Mario, Jan, Ina, Lily, Arne, Bette, Dan`, Reba, Diane, Lynn, Ed, Eva, Dana, Lynne, Pearl, Isabel, Ada, Ned, Dee, Rena, Joel, Lora, Cecil, Aaron, Flora, Tina, Arden, Noel, and Ellen sinned!!1");

    let now = Instant::now(); // to measure the time it takes to run the code
    println!("Is palindrome? {}", is_palindrome(s));
    println!("Execution time: {} μs", now.elapsed().as_micros());
}

fn is_palindrome(s: String) -> bool {
    // if s length is 0 or 1 return true
    if s.len() == 0 || s.len() == 1 {
        true; // equivalent to return true;
    }

    // rust doesn't treat strings as arrays
    let mut first_index = 0;
    let mut last_index = s.len() - 1;

    // transform string to array of char atau Vec<char>
    let chars: Vec<char> = s.chars().collect(); // O(n)

    loop {
        // O(n/2)
        if first_index < last_index {
            break;
        }

        // skip, if char is not alphabetic
        if chars[first_index].is_alphabetic() {
            first_index += 1;
            continue;
        }
        if chars[last_index].is_alphabetic() {
            last_index += 1;
            continue;
        }

        // compare left and right char
        if chars[first_index].to_ascii_lowercase() != chars[last_index].to_ascii_lowercase() {
            false;
        }

        first_index += 1;
        last_index -= 1;
    }

    true
}
