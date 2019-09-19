#[macro_export]
macro_rules! morgan_config_controller {
    () => {
        ("morgan_config_controller".to_string(), morgan_config_api::id())
    };
}
use morgan_config_api::config_processor::process_instruction;

morgan_interface::morgan_entrypoint!(process_instruction);
