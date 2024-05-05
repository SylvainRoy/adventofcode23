use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("./data/input.txt");

    // Part 1
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: i32 = parts[0][5..].parse().unwrap();
        let sets: Vec<&str> = parts[1].split("; ").collect();

        let mut possible = true;
        for set in sets {
            let cubes: Vec<&str> = set.split(", ").collect();
            for cube in cubes {
                let cube_parts: Vec<&str> = cube.split(" ").collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                if (color == "red" && count > 12)
                    || (color == "green" && count > 13)
                    || (color == "blue" && count > 14)
                {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            sum += game_id;
        }
    }
    println!("Part 1: {}", sum);

    // Part 2
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_power = 0;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        let sets: Vec<&str> = parts[1].split("; ").collect();
        let mut min_red = i32::MIN;
        let mut min_green = i32::MIN;
        let mut min_blue = i32::MIN;
        for set in sets {
            let cubes: Vec<&str> = set.split(", ").collect();
            for cube in cubes {
                let cube_parts: Vec<&str> = cube.split(" ").collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];
                if color == "red" {
                    min_red = min_red.max(count);
                } else if color == "green" {
                    min_green = min_green.max(count);
                } else if color == "blue" {
                    min_blue = min_blue.max(count);
                }
            }
        }
        let power = min_red * min_green * min_blue;
        total_power += power;
        println!("line: {:?}", line);
        println!("mins: {:?}, {:?}, {:?}", min_red, min_green, min_blue);
        println!("power: {:?}", power);
    }
    println!("Part 2: {}", total_power);
    Ok(())
}
