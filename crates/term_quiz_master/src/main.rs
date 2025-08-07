use term_quiz_master::quiz_logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    quiz_logic::run_quiz()
}
