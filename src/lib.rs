use ecdar_protobuf_transpiler::CompileVariables;
use proc_macro::TokenStream as TS;

use proc_macro2::TokenStream;
use quote::*;

#[proc_macro]
pub fn create_functions(_: TS) -> TS {
    let functions = ecdar_protobuf_transpiler::compile(|var| {
        let CompileVariables {
            in_struct,
            in_struct_name,
            in_struct_has_body,
            rtn_struct,
            fn_name,
            client,
            endpoint_name,
            ..
        } = var;

        let payload_body = if in_struct_has_body {
            quote!(payload.body)
        } else {
            quote!(())
        };

        quote! {
            #in_struct

            #[tauri::command]
            async fn #fn_name(payload : #in_struct_name)
                -> Result<#rtn_struct, GrpcError> {
                let mut client = #client::connect(format!("http://{}", payload.ip))
                    .await
                    .map_err(|_| GrpcError::FailedToConnect)?;
                let res = client
                    .#endpoint_name(#payload_body)
                    .await
                    .map_err(|_| GrpcError::FailedResponse)?
                    .into_inner();
                Ok(res)
            }
        }
    });

    quote!(#(#functions)*).into()
}

#[proc_macro]
pub fn generate_handler(other_commands: TS) -> TS {
    let other_commands = TokenStream::from(other_commands);
    let handlers = ecdar_protobuf_transpiler::compile(|var| {
        let CompileVariables { fn_name, .. } = var;

        quote!(#fn_name)
    });

    quote! {
        tauri::generate_handler![#(#handlers),*, #other_commands]
    }
    .into()
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
