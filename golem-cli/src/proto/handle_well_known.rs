use anyhow::anyhow;
use tailcall_valid::{Valid, Validator};
use crate::proto::DEFAULT_INTERFACE_NAME;
use crate::wit_config::config::{Interface, WitConfig};
use crate::wit_config::wit_types::well_known::grpc_errors;

#[derive(derive_setters::Setters, Default)]
pub struct AppendedTypes {
    pub appended_error: String,
}

pub fn handle_well_known(config: WitConfig) -> Valid<(WitConfig, AppendedTypes), anyhow::Error, anyhow::Error> {
    Valid::succeed(config)
        .and_then(|mut config| {
            Valid::from_option(config.interfaces.iter().find(|v| v.name == DEFAULT_INTERFACE_NAME), anyhow!("Expected default interface to be present"))
                .map(|v| (*v).clone())
                .and_then(append_grpc_err)
                .map(|(interface, err_ty)| {
                    config.interfaces.remove(&interface);
                    config.interfaces.insert(interface);
                    (config, AppendedTypes::default().appended_error(err_ty))
                })
        })
}

fn append_grpc_err(interface: Interface) -> Valid<(Interface, String), anyhow::Error, anyhow::Error> {
    let mut error_ty = "grpc-errors".to_string();
    Valid::succeed(interface)
        .and_then(|mut interface| {
            if !interface.varients.contains_key(&error_ty) {
                interface.varients.insert(error_ty.clone(), grpc_errors());
            }else {
                while interface.varients.contains_key(&error_ty) {
                    error_ty = format!("{}-grpc-errors", error_ty);
                }

                interface.varients.insert(error_ty.clone(), grpc_errors());
            }

            Valid::succeed((interface, error_ty))
        })
}
