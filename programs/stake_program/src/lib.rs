#[macro_export]
macro_rules! morgan_stake_program {
    () => {
        ("morgan_stake_program".to_string(), morgan_stake_api::id())
    };
}

use morgan_stake_api::stake_instruction::process_instruction;
morgan_interface::morgan_entrypoint!(process_instruction);
