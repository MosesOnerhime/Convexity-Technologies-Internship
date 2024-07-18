pub struct TaskView;

impl TaskView {
    pub fn display_message(message: &str) {
        println!("{}", message);
    }

    pub fn prompt_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
}
