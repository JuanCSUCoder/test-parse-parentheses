fn main() {
    println!("This is a simple test program for parenthesis parsing!!");

    let to_parse_string = "(())()(((())))".to_string();

    // Expected Result:
    // first_open_par_index = 0
    // last_close_par_index = 3

    let mut first_open_par_index: usize = 0;
    let mut last_close_par_index: usize = 0;
    let charlist: Vec<char> = to_parse_string.clone().chars().collect();
    let mut got_open_par = false;
    let mut got_close_par = false;
    let mut level: usize = 0;

    for index in 0..to_parse_string.len() {
        println!("Iter Start Value: {} Level: {} GF: {} GS: {}", charlist[index], level, got_open_par, got_close_par);
        if !got_open_par && charlist[index] == '(' {
            first_open_par_index = index;
            got_open_par = true;
        } else if charlist[index] == ')' && level == 0 && !got_close_par{
            last_close_par_index = index;
            got_close_par = true;
        } else if charlist[index] == '(' {
            level += 1;
        } else if charlist[index] == ')' {
            level -= 1;
        }
        println!("Iter End Value: {} Level: {} GF: {} GS: {}", charlist[index], level, got_open_par, got_close_par);
    }

    println!("Start: {}",first_open_par_index);
    println!("End: {}", last_close_par_index);
}
