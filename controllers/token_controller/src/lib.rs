#[macro_export]
macro_rules! morgan_token_controller {
    () => {
        ("morgan_token_controller".to_string(), morgan_token_api::id())
    };
}

use morgan_token_api::token_processor::process_instruction;

morgan_interface::morgan_entrypoint!(process_instruction);
