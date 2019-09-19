#[macro_export]
macro_rules! morgan_exchange_controller {
    () => {
        (
            "morgan_exchange_controller".to_string(),
            morgan_exchange_api::id(),
        )
    };
}
use morgan_exchange_api::exchange_processor::process_instruction;

morgan_interface::morgan_entrypoint!(process_instruction);
