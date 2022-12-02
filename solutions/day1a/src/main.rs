static INPUT_TEXT: &str = include_str!("../input.txt");

fn main() {
    let solution = INPUT_TEXT
                        .trim_end()
                        .split("\n\n") // Split by empty lines to get an iterator with an element for each elf's input
                        .map(|s| // Get a sum out of each elf's input
                            s.split('\n') // Get an iterator
                            .map(|e| 
                                e.parse::<u32>().unwrap()
                            ).sum::<u32>()
                        )
                        .max()
                        .unwrap();
    println!("Day 1A Solution: {solution}")
}
