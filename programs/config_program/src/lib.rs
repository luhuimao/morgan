#[macro_export]
macro_rules! morgan_config_program {
    () => {
        ("morgan_config_program".to_string(), morgan_config_api::id())
    };
}
use morgan_config_api::config_processor::process_instruction;

morgan_sdk::morgan_entrypoint!(process_instruction);
