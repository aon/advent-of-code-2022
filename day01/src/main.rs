fn main() {
    let input = include_str!("../input.txt");

    let lines = input.split("\n\n");

    let mut lines: Vec<u32> = lines
        .map(|line| {
            line.split("\n")
                .flat_map(|num| num.parse::<u32>())
                .sum::<u32>()
        })
        .collect();

    lines.sort_by(|a,b| b.cmp(a));

    println!("Part 1: {}", lines[0]);
    println!("Part 2: {}", lines[0] + lines[1] + lines[2]);
}
