fn tower_of_hanoi(n: i32, source: char, destination: char, auxiliary: char) {
    if n == 1 {
        return;
    }

    tower_of_hanoi(n - 1, source, auxiliary, destination);
    tower_of_hanoi(n - 1, auxiliary, destination, source);
}

fn display_final_configuration(n: i32) {
    let mut final_config = String::new();
    for i in (0..n).rev() {
        final_config += &"1".repeat(i as usize);
        final_config += &"0".repeat((n - i) as usize);
    }
    println!("{}", final_config);
}

fn main() {
    let mut n = String::new();
    loop {
        println!("Enter the number of disks:");
        n.clear();
        std::io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = match n.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => {
                println!("Number of disks should be a positive integer.");
                continue;
            }
        };
        
        tower_of_hanoi(n, 'A', 'B', 'C');
        display_final_configuration(n);
        break;
    }
}
