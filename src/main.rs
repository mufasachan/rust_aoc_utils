use presets::read_lines_from_file;

fn main() {
    if let Ok(lines) = read_lines_from_file("./one_int_per_line.txt") {
        for line in lines {
            if let Ok(int_string) = line {
                let int_value: i32 = int_string.parse().unwrap();
                if (int_value % 10) == 0 {
                    println!("{}", int_value);
                }
            }
        }
    }
}
