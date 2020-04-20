fn main() {
    let boolean = backspace_compare("ab#c".to_string(), "b#ac".to_string());
    println!("{}", boolean);
}

pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_map: Vec<char> = Vec::new();
        let mut t_map: Vec<char> = Vec::new();
        for i in s.chars() {
            if i != '#' {
                s_map.push(i);
            } else {
                s_map.pop();
            }
        }
        
        for i in t.chars() {
            if i != '#' {
                t_map.push(i);
            } else {
                t_map.pop();
            }
        }
        
        s_map == t_map
    }