use cainome::rs::Abigen;
use std::collections::HashMap;

fn main() {
    // Aliases added from the ABI
    let mut aliases = HashMap::new();

    let usdc_abigen =
        Abigen::new("usdc", "./abi/usdc_contract.abi.json").with_types_aliases(aliases).with_derives(vec!["serde::Serialize".to_string(), "serde::Deserialize".to_string()]);

        usdc_abigen
            .generate()
            .expect("Fail to generate bindings")
            .write_to_file("./src/abi/usdc_contract.rs")
            .unwrap();
}