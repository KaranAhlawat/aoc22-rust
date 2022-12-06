#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
    common: Vec<char>,
}

struct Group<'a>(&'a Rucksack, &'a Rucksack, &'a Rucksack);

impl<'a> Group<'a> {
    fn get_elf_badge(&self) -> char {
        let first: Vec<&char> = self
            .0
            .first_compartment
            .iter()
            .chain(self.0.second_compartment.iter())
            .collect();
        let second: Vec<&char> = self
            .1
            .first_compartment
            .iter()
            .chain(self.1.second_compartment.iter())
            .collect();
        let third: Vec<&char> = self
            .2
            .first_compartment
            .iter()
            .chain(self.2.second_compartment.iter())
            .collect();

        for ch in first {
            if second.contains(&ch) && third.contains(&ch) {
                return *ch;
            }
        }

        '0'
    }
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
    let sacks = file.split('\n').map(line_to_rucksack).collect::<Vec<_>>();

    let mut groups: Vec<Group> = Vec::new();
    let mut idx = 0;
    let total_len = sacks.len();
    while idx < total_len {
        groups.push(Group(&sacks[idx], &sacks[idx + 1], &sacks[idx + 2]));
        idx += 3;
    }

    groups
        .iter_mut()
        .map(|group| group.get_elf_badge())
        .map(|ch| char_to_priority(&ch))
        .sum()
}
