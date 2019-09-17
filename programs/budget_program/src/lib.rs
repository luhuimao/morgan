#[macro_export]
macro_rules! morgan_budget_program {
    () => {
        ("morgan_budget_program".to_string(), morgan_budget_api::id())
    };
}

use morgan_budget_api::budget_processor::process_instruction;
morgan_interface::morgan_entrypoint!(process_instruction);
