use std::collections::HashSet;

fn main() {
    let input = "R4, R3, R5, L3, L5, R2, L2, R5, L2, R5, R5, R5, R1, R3, L2, L2, L1, R5, L3, R1, \
                 L2, R1, L3, L5, L1, R3, L4, R2, R4, L3, L1, R4, L4, R3, L5, L3, R188, R4, L1, \
                 R48, L5, R4, R71, R3, L2, R188, L3, R2, L3, R3, L5, L1, R1, L2, L4, L2, R5, L3, \
                 R3, R3, R4, L3, L4, R5, L4, L4, R3, R4, L4, R1, L3, L1, L1, R4, R1, L4, R1, L1, \
                 L3, R2, L2, R2, L1, R5, R3, R4, L5, R2, R5, L5, R1, R2, L1, L3, R3, R1, R3, L4, \
                 R4, L4, L1, R1, L2, L2, L4, R1, L3, R4, L2, R3, L1, L5, R4, R5, R2, R5, R1, R5, \
                 R1, R3, L3, L2, L2, L5, R2, L2, R5, R5, L2, R3, L5, R5, L2, R4, R2, L1, R3, L5, \
                 R3, R2, R5, L1, R3, L2, R2, R1";

    let mut set = HashSet::new();

    let final_position = input.split(", ").fold((0, 0, 0), |acc, instruction| {
        let direction = match instruction.chars().nth(0).unwrap() {
            'R' => (acc.2 + 1) % 4,
            'L' => (acc.2 + 3) % 4,
            _ => unreachable!(),
        };
        let length: i32 = instruction[1..].parse().unwrap();
        for i in 0..length {
            let new_pos = match direction {
                0 => (acc.0, acc.1 + i),
                1 => (acc.0 + i, acc.1),
                2 => (acc.0, acc.1 - i),
                3 => (acc.0 - i, acc.1),
                _ => unreachable!(),
            };
            if !set.insert(new_pos) {
                println!("Previously visited position: {:?} => {} Blocks away",
                         new_pos,
                         new_pos.0.abs() + new_pos.1.abs());
                std::process::exit(0);
            }
        }
        let x = match direction {
            0 => (acc.0, acc.1 + length, direction),
            1 => (acc.0 + length, acc.1, direction),
            2 => (acc.0, acc.1 - length, direction),
            3 => (acc.0 - length, acc.1, direction),
            _ => unreachable!(),
        };
        x
    });
    println!("{:?}", final_position);
}
