use std::io::{self, Read};

// pointer used is $t0

struct BFCompiler {
    code: Vec<char>, // the code characters are stored here
    code_ptr: usize, // where are we when reading the brainfuck code?
    output: String, // output is built here
    label_counter: i32, //used to track what loop we are in, only goes up
    label_stack: Vec<i32> //used to track what loop we are in
}

impl BFCompiler {
    fn new() -> Self {
        BFCompiler {
            code: Vec::new(),
            code_ptr: 0,
            output: String::new(),
            label_counter: 0,
            label_stack: Vec::new(),
        }
    }

    fn compile(&mut self) {
        self.output.push_str(".data\n");
        self.output.push_str("myram: .space 16384\n"); //tell mips to use memory that is 16384 in size, google says this should make la use the right memory address which is 0x10010000
        self.output.push_str(".text\n");
        self.output.push_str("la $t0, myram\n"); //point to the data segment 0x10010000

        while self.code_ptr < self.code.len() {
            match self.code[self.code_ptr] {
                '>' => self.output.push_str("addi $t0, $t0, 1\n"), //incr ptr value
                '<' => self.output.push_str("addi $t0, $t0, -1\n"), //decr ptr value
                '+' => self.output.push_str("lb $t1, 0($t0)\naddi $t1, $t1, 1\nsb $t1, 0($t0)\n"), //incr byte point use $t1 as local
                '-' => self.output.push_str("lb $t1, 0($t0)\naddi $t1, $t1, -1\nsb $t1, 0($t0)\n"), //decr byte point use $t1 as local
                '.' => self.output.push_str("lb $a0, 0($t0)\nli $v0, 11\nsyscall\n"), // read the contents of $t0 as ASCII
                ',' => self.output.push_str(""), //??
                '[' => { //code_block loop logic
                    let loop_label = format!("loop_{}", self.label_counter);
                    let end_loop_label = format!("end_loop_{}", self.label_counter);
                    self.label_stack.push(self.label_counter);
                    self.label_counter += 1;                    
                    self.output.push_str(&format!("{}:\n", loop_label));
                    self.output.push_str("lb $t1, 0($t0)\nbeqz $t1, ");
                    self.output.push_str(&end_loop_label);
                    self.output.push_str("\n");
                }
                ']' => {
                    let label_pop = self.label_stack.pop().unwrap().to_string();
                    let loop_label = format!("loop_{}", label_pop);
                    let end_loop_label = format!("end_loop_{}", label_pop);
                    self.output.push_str("j ");
                    self.output.push_str(&loop_label);
                    self.output.push_str("\n");
                    self.output.push_str(&format!("{}:\n", end_loop_label));
                }
                _ => {}
            }
            self.code_ptr += 1;
        }

        self.output.push_str("li $v0, 10\nsyscall\n");
    }
}

fn main() {
    let mut code = String::new();
    io::stdin().read_to_string(&mut code).unwrap();

    let mut compiler = BFCompiler::new();
    compiler.code = code.chars().collect();
    compiler.compile();

    println!("{}", compiler.output);
}
