use data_stock::{read_file_to_line, split_line, write_line};

fn main() {
    let file = read_file_to_line("data/ac/bweb_1t_AC_051020221321.csv");
    match file {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line_str) => {
                        let data = split_line(&line_str);
                        println!("{}", &data.to_string());
                    }
                    Err(err) => {
                        println!("Fail to read line: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            println!("Error to read file: {}", err);
        }
    }
}
