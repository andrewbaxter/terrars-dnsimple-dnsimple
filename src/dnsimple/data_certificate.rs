use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct DataCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    certificate_id: PrimField<f64>,
    domain: PrimField<String>,
}

struct DataCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCertificateData>,
}

#[derive(Clone)]
pub struct DataCertificate(Rc<DataCertificate_>);

impl DataCertificate {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDnsimple) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Get a reference to the value of field `certificate_chain` after provisioning.\nCertificate chain"]
    pub fn certificate_chain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate ID"]
    pub fn certificate_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nDomain name"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nPrivate key"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_certificate` after provisioning.\nRoot certificate"]
    pub fn root_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_certificate` after provisioning.\nServer certificate"]
    pub fn server_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }
}

impl Referable for DataCertificate {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCertificate { }

impl ToListMappable for DataCertificate {
    type O = ListRef<DataCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCertificate_ {
    fn extract_datasource_type(&self) -> String {
        "dnsimple_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCertificate {
    pub tf_id: String,
    #[doc= "Certificate ID"]
    pub certificate_id: PrimField<f64>,
    #[doc= "Domain name"]
    pub domain: PrimField<String>,
}

impl BuildDataCertificate {
    pub fn build(self, stack: &mut Stack) -> DataCertificate {
        let out = DataCertificate(Rc::new(DataCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                certificate_id: self.certificate_id,
                domain: self.domain,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCertificateRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `certificate_chain` after provisioning.\nCertificate chain"]
    pub fn certificate_chain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate ID"]
    pub fn certificate_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nDomain name"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nPrivate key"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_certificate` after provisioning.\nRoot certificate"]
    pub fn root_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_certificate` after provisioning.\nServer certificate"]
    pub fn server_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }
}
