pub mod genesis_block_util;

#[macro_export]
macro_rules! morgan_storage_program {
    () => {
        (
            "morgan_storage_program".to_string(),
            morgan_storage_api::id(),
        )
    };
}

use morgan_storage_api::storage_processor::process_instruction;
morgan_sdk::morgan_entrypoint!(process_instruction);