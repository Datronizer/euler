use tailcall::tailcall;

fn main() {
    choose_question()
}

fn choose_question() {
    println!("Choose the question whose answer you wish to see!
      1. Sum of multiples of 3 or 5 less than 1000");
    
    let mut question_choice = String::new();
    std::io::stdin()
        .read_line(&mut question_choice)
        .expect("Failed to read line");

    let choice: u32 = question_choice
        .trim()
        .parse()
        .expect("Failed to parse input");

    match choice {
        1 => println!("{}", euler_1()),
        _ => println!("Sorry. This question is not implemented yet"),
    }
}

// Tail recursive solution
fn euler_1() -> u32 {
    #[tailcall]
    fn multiple_inner(acc: u32, n: u32) -> u32 {
        if n >= 1000 {
            return acc;
        }

        if n % 3 == 0 || n % 5 == 0 {
            multiple_inner(acc + n, n + 1)
        } else {
            multiple_inner(acc, n + 1)
        }
    }

    multiple_inner(0, 0)
}