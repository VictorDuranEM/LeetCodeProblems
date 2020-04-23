fn main() {
    let vec = vec![0,1,1,1,1,0,0];
    println!("{}", find_max_length(vec));
}

pub fn find_max_length(nums: Vec<i32>) -> i32 {

    if nums.len() < 2 {
        return 0;
    }

    let mut max: usize = 0;
    let mut starting_position: usize = 0;

    loop {

        max = find_max_length_single_iteration(&mut max, &mut starting_position, &nums);

        if starting_position == nums.len() - 2 {
            break;
        }

        starting_position += 1;
    }

    max as i32
}

pub fn find_max_length_single_iteration(max: &mut usize, starting_position: &mut usize, nums: &Vec<i32>) -> usize {
    let mut sum: i32 = 0;

    // println!("{:?}", *nums);
    for i in *starting_position..nums.len() {

        // println!("Value of i: {}", i);
        let num = nums[i as usize];
        
        if num == 1 {
            sum += 1;
        } else {
            sum -= 1;
        }

        // println!("Value of sum: {}", sum);

        let max_found = (i + 1) - *starting_position;

        if sum == 0 && max_found > *max {
            *max = max_found;
        }

        // println!("Value of max {}", max);
        // println!("");
    }
    
    *max
}