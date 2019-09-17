#[macro_export]
macro_rules! morgan_vote_program {
    () => {
        ("morgan_vote_program".to_string(), morgan_vote_api::id())
    };
}

use morgan_vote_api::vote_instruction::process_instruction;
morgan_interface::morgan_entrypoint!(process_instruction);
