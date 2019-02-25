use std::fs::File;
use std::io::Read;
use rand::Rng;

use std::iter;

use unicode_width::UnicodeWidthStr;

const MAX_LINE_WIDTH: usize = 50;
const OCTOPUS: &str = "
        \\       ⌢⌢
         \\   ◜      ◝
          \\ (        ) 
           ◟( (@)(@) )
             )  ◟◞  (
            /,'))((`.\\
           (( ((  )) ))
           ))`\\ `)(´/((";



fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn main() {

    let mut message = "You are a ".to_string();

    //Read animals from file
    let adjectives =
        lines_from_file("/Users/gargisharma/rust/projects/galentines/data/adjectives.txt");
    let animals = lines_from_file("/Users/gargisharma/rust/projects/galentines/data/nouns.txt");

    //let adjectives = file_to_string("/Users/gargisharma/rust/projects/galentines/src/adjectives.txt").unwrap();
    
    let mut len_of_adjective = rand::thread_rng().gen_range(2, 6);

    //get all adjectives
    
    while len_of_adjective != 0 {
        let index = rand::thread_rng().gen_range(0, adjectives.len());
        message.push_str(&adjectives[index]);
        message.push_str(", ");
        len_of_adjective = len_of_adjective - 1;
    }

    //get the noun
    let index2 = rand::thread_rng().gen_range(0, animals.len());
    message.push_str(&animals[index2]);
    message.push_str("!");
    //Read adjectives from file

    //Generate message from file

   print_octosay(&message);
}

fn print_octosay(input: &str) {
    let mut lines = Vec::new();
    let words = input.split(" ");
    let mut current_line = String::new();

    for (i, word) in words.enumerate() {
        let word_width = word.width();

        if word_width > MAX_LINE_WIDTH {
            // special case: if one word is >40 chars, break it up with hyphens
            if !current_line.is_empty() {
                current_line.push(' ');
            }

            for c in word.chars() {
                if current_line.width() < MAX_LINE_WIDTH - 1 {
                    current_line.push(c);
                } else {
                    current_line.push('-');
                    lines.push(current_line);
                    current_line = c.to_string();
                }
            }
        } else if current_line.width() + word_width > MAX_LINE_WIDTH {
            // starts a new line
            lines.push(current_line);
            current_line = String::from(word);
        } else {
            if i != 0 {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if current_line.width() > 0 {
        lines.push(current_line);
    }

    let max_line_length = lines
        .iter()
        .map(|line| line.as_str().width())
        .max()
        .unwrap_or(0);

    let padded_lines = lines
        .into_iter()
        .map(|mut line| {
            let spaces = iter::repeat(" ")
                .take(max_line_length - line.as_str().width())
                .collect::<String>();
            line.push_str(&spaces);
            line
        })
        .collect::<Vec<_>>();

    let horizontal_border = iter::repeat("-")
        .take(max_line_length + 2)
        .collect::<String>();

    println!(" {} ", horizontal_border);
    for line in &padded_lines {
        println!("< {} >", line);
    }
    println!(" {} ", horizontal_border);
    println!("{}", OCTOPUS);
}