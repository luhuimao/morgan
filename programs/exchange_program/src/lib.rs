#[macro_export]
macro_rules! morgan_exchange_program {
    () => {
        (
            "morgan_exchange_program".to_string(),
            morgan_exchange_api::id(),
        )
    };
}
use morgan_exchange_api::exchange_processor::process_instruction;

morgan_interface::morgan_entrypoint!(process_instruction);
