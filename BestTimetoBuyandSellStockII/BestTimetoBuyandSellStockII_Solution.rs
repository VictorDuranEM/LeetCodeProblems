impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
      if prices.is_empty() {
          return 0;
      }

      let mut base_number : i32 = prices[0];
      let mut profit : i32 = 0;
      let mut curr_num : i32;
      let mut prev_num : i32;

      for i in 1..prices.len() {
          curr_num = prices[i];
          prev_num = prices[i-1];
          // println!("I: {}", i);
          // println!("prices.len() -1: {}", prices.len() -1);

          if curr_num >= prev_num {
              if i == prices.len() -1 {
                  profit += curr_num - base_number;
                  continue;
              }
              continue;
          } else {
              profit += prev_num - base_number;
              base_number = curr_num;
          }
          // println!("Current Number: {}", curr_num);
          // println!("Profit at {} loop is {}: ", i, profit);
      }
      return profit;
  }
}