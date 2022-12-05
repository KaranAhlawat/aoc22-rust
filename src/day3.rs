#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
    common: Vec<char>,
}

impl Rucksack {
    fn get_priority(&self) -> u32 {
        self.common.iter().map(char_to_priority).sum()
    }
}

fn char_to_priority(character: &char) -> u32 {
    match character {
        'a'..='z' => *character as u32 - 'a' as u32 + 1,
        'A'..='Z' => *character as u32 - 'A' as u32 + 1 + 26,
        _ => unreachable!("Invalid character"),
    }
}

fn line_to_rucksack(line: &str) -> Rucksack {
    let split = line.split_at(line.len() / 2);

    // Populate the common part between both the splits
    let mut cur = Rucksack {
        first_compartment: split.0.chars().collect(),
        second_compartment: split.1.chars().collect(),
        common: Vec::new(),
    };

    cur.first_compartment.iter().for_each(|current_char| {
        if cur.second_compartment.contains(current_char) && !cur.common.contains(current_char) {
            cur.common.push(*current_char)
        }
    });

    cur
}

pub fn part_1(file: &str) -> u32 {
    file.split('\n')
        .map(line_to_rucksack)
        .map(|sack| sack.get_priority())
        .sum()
}

pub fn part_2(file: &str) -> u32 {
    0
}
