#[cfg(any(feature = "bpf_c", feature = "bpf_rust"))]
mod bpf {
    use morgan_runtime::bank::Bank;
    use morgan_runtime::bank_client::BankClient;
    use morgan_runtime::loader_utils::load_program;
    use morgan_interface::genesis_block::create_genesis_block;
    use morgan_interface::native_loader;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;

    /// BPF program file extension
    const PLATFORM_FILE_EXTENSION_BPF: &str = "so";

    /// Create a BPF program file name
    fn create_bpf_path(name: &str) -> PathBuf {
        let mut pathbuf = {
            let current_exe = env::current_exe().unwrap();
            PathBuf::from(current_exe.parent().unwrap().parent().unwrap())
        };
        pathbuf.push("bpf/");
        pathbuf.push(name);
        pathbuf.set_extension(PLATFORM_FILE_EXTENSION_BPF);
        pathbuf
    }

    #[cfg(feature = "bpf_c")]
    mod bpf_c {
        use super::*;
        use morgan_runtime::loader_utils::create_invoke_instruction;
        use morgan_interface::bpf_loader;
        use morgan_interface::client::SyncClient;
        use morgan_interface::signature::KeypairUtil;
        use std::io::Read;

        #[test]
        fn test_program_bpf_c_noop() {
            morgan_logger::setup();

            let mut file = File::open(create_bpf_path("noop")).expect("file open failed");
            let mut elf = Vec::new();
            file.read_to_end(&mut elf).unwrap();

            let (genesis_block, alice_keypair) = create_genesis_block(50);
            let bank = Bank::new(&genesis_block);
            let bank_client = BankClient::new(bank);

            // Call user program
            let program_id = load_program(&bank_client, &alice_keypair, &bpf_loader::id(), elf);
            let instruction = create_invoke_instruction(alice_keypair.pubkey(), program_id, &1u8);
            bank_client
                .send_instruction(&alice_keypair, instruction)
                .unwrap();
        }

        #[test]
        fn test_program_bpf_c() {
            morgan_logger::setup();

            let programs = [
                "bpf_to_bpf",
                "multiple_static",
                "noop",
                "noop++",
                "relative_call",
                "struct_pass",
                "struct_ret",
            ];
            for program in programs.iter() {
                println!("Test program: {:?}", program);
                let mut file = File::open(create_bpf_path(program)).expect("file open failed");
                let mut elf = Vec::new();
                file.read_to_end(&mut elf).unwrap();

                let (genesis_block, alice_keypair) = create_genesis_block(50);
                let bank = Bank::new(&genesis_block);
                let bank_client = BankClient::new(bank);

                let loader_pubkey = load_program(
                    &bank_client,
                    &alice_keypair,
                    &native_loader::id(),
                    "morgan_bpf_loader".as_bytes().to_vec(),
                );

                // Call user program
                let program_id = load_program(&bank_client, &alice_keypair, &loader_pubkey, elf);
                let instruction =
                    create_invoke_instruction(alice_keypair.pubkey(), program_id, &1u8);
                bank_client
                    .send_instruction(&alice_keypair, instruction)
                    .unwrap();
            }
        }
    }

    #[cfg(feature = "bpf_rust")]
    mod bpf_rust {
        use super::*;
        use morgan_interface::client::SyncClient;
        use morgan_interface::instruction::{AccountMeta, Instruction};
        use morgan_interface::signature::{Keypair, KeypairUtil};
        use std::io::Read;

        #[test]
        fn test_program_bpf_rust() {
            morgan_logger::setup();

            let programs = [
                "morgan_bpf_rust_alloc",
                // Disable due to #4271 "morgan_bpf_rust_iter",
                "morgan_bpf_rust_noop",
            ];
            for program in programs.iter() {
                let filename = create_bpf_path(program);
                println!("Test program: {:?} from {:?}", program, filename);
                let mut file = File::open(filename).unwrap();
                let mut elf = Vec::new();
                file.read_to_end(&mut elf).unwrap();

                let (genesis_block, alice_keypair) = create_genesis_block(50);
                let bank = Bank::new(&genesis_block);
                let bank_client = BankClient::new(bank);

                let loader_pubkey = load_program(
                    &bank_client,
                    &alice_keypair,
                    &native_loader::id(),
                    "morgan_bpf_loader".as_bytes().to_vec(),
                );

                // Call user program
                let program_id = load_program(&bank_client, &alice_keypair, &loader_pubkey, elf);
                let account_metas = vec![
                    AccountMeta::new(alice_keypair.pubkey(), true),
                    AccountMeta::new(Keypair::new().pubkey(), false),
                ];
                let instruction = Instruction::new(program_id, &1u8, account_metas);
                bank_client
                    .send_instruction(&alice_keypair, instruction)
                    .unwrap();
            }
        }
    }
}
