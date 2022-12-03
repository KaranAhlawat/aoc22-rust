pub fn part_1(file: &String) -> u32 {
    return file
        .split("\n\n")
        .map(|load| {
            load.split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
}

pub fn part_2(file: &String) -> u32 {
    let mut sums = file
        .split("\n\n")
        .map(|load| {
            load.split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    sums.sort_by(|a, b| b.cmp(a));
    return sums.iter().take(3).sum();
}