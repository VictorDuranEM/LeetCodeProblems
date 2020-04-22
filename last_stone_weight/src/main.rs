fn main() {
    let vec = vec![7,6,7,6,9];
    println!("{}", last_stone_weight(vec));
}

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    if stones.len() == 0 {
        return 0;
    }

    if stones.len() == 1 {
        return stones[0];
    }

    let mut stones_copy = stones.clone();

    loop {
        stones_copy.sort();
        stones_copy.reverse();
        let first_element = stones_copy[0];
        let second_element = stones_copy[1];

        println!("{:?}", stones_copy);
        if first_element > second_element {
            stones_copy[0] = first_element - second_element;
            stones_copy.remove(1);
        } else if second_element > first_element {
            stones_copy[1] = second_element - first_element;
            stones_copy.remove(0);
        } else {
            stones_copy.remove(0);
            stones_copy.remove(0);

        }

        println!("{:?}", stones_copy);

        if stones_copy.len() == 0 {
            return 0;
        }

        if stones_copy.len() == 1 {
            break;
        }
    }

    return stones_copy[0];
}