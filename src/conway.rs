use super::*;

pub struct Conway {}
impl Conway {
    pub fn prompt(&self) {
        print!("Conway> ");
        io::stdout().flush().unwrap();
    }

    pub fn read_input(&self) {
        let mut input_buffer = String::new();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read stdin");

        let astnode = parser(&&input_buffer).expect("unsuccessful parse");
        let mut int = Interpreter { env: Environment::new() };
        for node in astnode.into_iter() {
            println!("{:?}", int.eval(&node));
        }
    }

    pub fn run(&self) {
        loop {
            self.prompt();
            self.read_input();
        }
    }
}


