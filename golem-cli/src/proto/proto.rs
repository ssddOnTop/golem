use anyhow::anyhow;
use protox::prost_reflect::prost_types::FileDescriptorSet;
use tailcall_valid::{Valid, Validator};
use crate::wit_config::config::WitConfig;
use crate::wit_config::wit_types::WitType;
use crate::proto::handle_services::handle_services;
use crate::proto::handle_types::handle_types;
use crate::proto::handle_well_known::handle_well_known;

pub struct Proto(Vec<FileDescriptorSet>);

impl Proto {
    pub fn new<T: IntoIterator<Item=FileDescriptorSet>>(v: T) -> Self {
        Self(v.into_iter().collect())
    }
    pub fn to_config<T: AsRef<str>>(&self, package_name: T) -> Valid<WitConfig, anyhow::Error, anyhow::Error> {
        Valid::succeed(WitConfig::default())
            .and_then(|config| handle_types(config, &self.0, package_name.as_ref().to_string()))
            .and_then(|config| handle_well_known(config))
            .and_then(|(config, appended_types)| handle_services(config, &self.0, Some(appended_types.appended_error)))
    }
}

pub fn process_ty(name: &str) -> Valid<WitType, anyhow::Error, anyhow::Error> {
    if !name.starts_with('.') {
        return Valid::fail(anyhow!("Expected fully-qualified name for reference type but got {}. This is a bug!", name));
    }
    let name = &name[1..];
    if let Some((_package, name)) = name.rsplit_once('.') {
        Valid::succeed(WitType::FieldTy(name.to_string()))
    }else {
        Valid::succeed(WitType::FieldTy(name.to_string()))
    }
}

#[cfg(test)]
test_r::enable!();

#[cfg(test)]
mod tests {
    use test_r::test;
    use tailcall_valid::Validator;
    use wit_parser::Resolve;
    use crate::proto::proto::Proto;

    #[test]
    fn test_nested() {
        let relative = format!("{}/src/proto/fixtures",env!("CARGO_MANIFEST_DIR"));
       let proto = protox::compile([format!("{}/address.proto", relative)], [relative]).unwrap();
        let proto = Proto::new([proto]);
        let config = proto.to_config("api:todos@1.0.0").to_result().unwrap();

        let mut resolve = Resolve::new();
        assert!(resolve.push_str("foox.wit", &config.to_wit()).is_ok());

        insta::assert_snapshot!(config.to_wit());
    }
}