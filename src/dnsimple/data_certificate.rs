use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct DataCertificateData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    certificate_id: terrars::PrimField<f64>,
    domain: terrars::PrimField<std::string::String>,
}

struct DataCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCertificateData>,
}

#[derive(Clone)]
pub struct DataCertificate(std::rc::Rc<DataCertificate_>);

impl DataCertificate {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDnsimple) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\nCertificate chain"]
    pub fn certificate_chain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate ID"]
    pub fn certificate_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain name"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nPrivate key"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_certificate` after provisioning.\nRoot certificate"]
    pub fn root_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\nServer certificate"]
    pub fn server_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }
}

impl terrars::Referable for DataCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCertificate { }

impl terrars::ToListMappable for DataCertificate {
    type O = terrars::ListRef<DataCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCertificate_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "dnsimple_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCertificate {
    pub tf_id: std::string::String,
    #[doc = "Certificate ID"]
    pub certificate_id: terrars::PrimField<f64>,
    #[doc = "Domain name"]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildDataCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCertificate {
        let out = DataCertificate(std::rc::Rc::new(DataCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCertificateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                certificate_id: self.certificate_id,
                domain: self.domain,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCertificateRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\nCertificate chain"]
    pub fn certificate_chain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate ID"]
    pub fn certificate_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain name"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nPrivate key"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_certificate` after provisioning.\nRoot certificate"]
    pub fn root_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\nServer certificate"]
    pub fn server_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }
}
