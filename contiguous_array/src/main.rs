use std::collections::HashMap;

fn main() {
    let vec = vec![1,0,1,1,1,1,0,0,1,0,1,0,0,1,0,1,1,1];
    println!("{}", find_max_length(vec));
}

pub fn find_max_length(nums: Vec<i32>) -> i32 {

    // HashMap<position, first_index_in_position>
    let mut map: HashMap<i32, usize> = HashMap::new();
    map.insert(0, 0);

    if nums.len() < 2 {
        return 0;
    }

    let mut position: i32 = 0;
    let mut max: usize = 0;

    for (i, num) in nums.iter().enumerate() {
        if *num == 1 {
            position += 1;
        } else {
            position -= 1;
        }
        // println!("Iteration: {}", i);
        // println!("Map before loop: {:?}", map);
        // println!("Position: {}", position);
        if map.contains_key(&position) {
            let distance: usize = if i == 1 { 2 } else { i - map.get(&position).unwrap() };
            // println!("Position found in the map");
            // println!("Distance from first position: {}", distance);
            if distance > max {
                max = distance;
            }

        } else {
            // println!("Position not found in the map");
            map.insert(position, i);
        }
        // println!("Max: {}", max);
        // println!("Map after loop {:?}", map);
        // println!("");
    }

    max as i32
}