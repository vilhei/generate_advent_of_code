use std::fs;
fn main() {
    let mut bin_parent_location =
        std::env::current_exe().expect("Failed to read path of the binary");
    bin_parent_location.pop();
    bin_parent_location.pop();

    let dir_res = fs::create_dir(bin_parent_location.join("inputs"));
    match dir_res {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to create inputs directory \nNOTE! if the inputs directory already exists ignore this\n -- reason :\n{:?}",e),
    }

    let mut advent_location = bin_parent_location.join("advent");
    let mut inputs_location = bin_parent_location.join("inputs");

    let days = 24;

    for day in 1..=days {
        advent_location.push(format!("day{day}.rs"));
        let content = generate_file_content(day);
        let file_res = fs::write(&advent_location, content);
        advent_location.pop();

        match file_res {
            Ok(_) => println!("Day {day} rust file generated"),
            Err(e) => eprintln!("Failed to generate day {day} file -- reason :\n{}", e),
        }

        inputs_location.push(format!("day{day}.txt"));
        let input_file_res = fs::File::create(&inputs_location);
        inputs_location.pop();

        match input_file_res {
            Ok(_) => println!("Day {day} input file generated"),
            Err(e) => eprintln!("Failed to generate day {day} input file -- reason :\n{}", e),
        }
    }
}

fn generate_file_content(day: u32) -> String {
    format!(
        "use crate::utils::{{read_task_input_file, Task, TaskError}};

pub struct Day{day};

impl Task for Day{day} {{
    fn task_part_one(&self, input_file: &str) -> Result<String, TaskError> {{
        let file_content = read_task_input_file(input_file)?;
        Err(TaskError::NotImplemented(1))
    }}

    fn task_part_two(&self, input_file: &str) -> Result<String, TaskError> {{
        let file_content = read_task_input_file(input_file)?;
        Err(TaskError::NotImplemented(2))
    }}
}}
"
    )
}
