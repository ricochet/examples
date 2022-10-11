use wasmbus_sender::*;

struct Component;

impl WasmbusSender for Component {
    fn send(
        msg: Message,
        contract_name: String,
        link_name: Option<String>,
    ) -> Result<Payload, RpcError> {
        println!(
            "Linkname: {}, contract_name: {}, msg: {:#?}",
            link_name.unwrap_or_else(|| "default".to_string()),
            contract_name,
            msg
        );
        Ok(serde_json::to_vec(&42).unwrap())
    }
}

wasmbus_sender::export!(Component);
