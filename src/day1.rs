pub fn part_1(file: &str) -> u32 {
    return file
        .split('\n')
        .map(|load| {
            load.split('\n')
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
}

pub fn part_2(file: &str) -> u32 {
    let mut sums = file
        .split('\n')
        .map(|load| {
            load.split('\n')
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    sums.sort();
    return sums.iter().rev().take(3).sum();
}
