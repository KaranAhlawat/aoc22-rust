#[derive(Debug)]
struct ElfRange {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct Pair(ElfRange, ElfRange);

impl Pair {
    fn subsumes(&self) -> bool {
        let first_sub = self.0.start <= self.1.start && self.0.end >= self.1.end;
        let second_sub = self.1.start <= self.0.start && self.1.end >= self.0.end;

        return first_sub || second_sub;
    }

    fn overlaps(&self) -> bool {
        if self.subsumes() {
            return true;
        }

        if self.0.start <= self.1.start && self.0.end >= self.1.start {
            return true;
        }
        if self.1.start <= self.0.start && self.1.end >= self.0.start {
            return true;
        }

        return false;
    }
}

fn str_to_range(piece: &str) -> ElfRange {
    let (start, end) = piece.split_at(piece.find('-').unwrap());

    return ElfRange {
        start: start.parse::<u32>().unwrap(),
        end: end[1..end.len()].parse::<u32>().unwrap(),
    };
}

fn line_to_elf_pair(line: &str) -> Pair {
    let (first, second) = line.split_at(line.find(',').unwrap());

    return Pair {
        0: str_to_range(first),
        1: str_to_range(&second[1..second.len()]),
    };
}

pub fn part_1(file: &str) -> u32 {
    file.split('\n')
        .map(line_to_elf_pair)
        .filter(|pair| pair.subsumes())
        .count() as u32
}

pub fn part_2(file: &str) -> u32 {
    file.split('\n')
        .map(line_to_elf_pair)
        .filter(|pair| pair.overlaps())
        .count() as u32
}
