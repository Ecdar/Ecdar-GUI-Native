use proc_macro::TokenStream;

use convert_case::*;

#[proc_macro]
pub fn create_functions(_: TokenStream) -> TokenStream {
    ecdar_protobuf_transpiler::compile(|var|
        [
            format!("#[derive(serde::Serialize, serde::Deserialize)]"),
            format!("{}", var.in_struct),
            format!("#[tauri::command]"),
            format!("async fn {}(payload : {})", var.fn_name, var.in_struct_name),
            format!("-> Result<{}, GrpcError>{{", var.rtn_struct),
            format!("Ok("),
            format!("ecdar_protobuf::services::{}_client::{}Client::connect", 
                    var.service_name.to_case(Case::Snake),
                    var.service_name.to_case(Case::Pascal)
            ),
            format!("(format!(\"http://{{}}\", payload.ip)).await.map_err(|_|GrpcError::FailedToConnect)?"),
            format!(".{}({}).await.map_err(|_|GrpcError::FailedResponce)?.into_inner()", 
                    var.endpoint_name.to_case(Case::Snake),
                    if var.in_struct_has_body { "payload.body" } else { "()" }
            ),
            format!(")}}"),
        ].join("\n")
    ).parse().unwrap()
}

#[proc_macro]
pub fn generate_handler(other_commands: TokenStream) -> TokenStream {
    format!(
        "tauri::generate_handler![{}{}]",
        ecdar_protobuf_transpiler::compile(|var| format!("{},", var.fn_name)).as_str(),
        other_commands.to_string().as_str()
    )
    .parse()
    .unwrap()
}

/** For macro debugging **/
trait Print {
    fn print(self) -> Self;
}

impl Print for String {
    fn print(self) -> Self {
        println!("{self}");
        self
    }
}
