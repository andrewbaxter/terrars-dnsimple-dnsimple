use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct LetsEncryptCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_names: Option<ListField<PrimField<String>>>,
    auto_renew: PrimField<bool>,
    domain_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature_algorithm: Option<PrimField<String>>,
}

struct LetsEncryptCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LetsEncryptCertificateData>,
}

#[derive(Clone)]
pub struct LetsEncryptCertificate(Rc<LetsEncryptCertificate_>);

impl LetsEncryptCertificate {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDnsimple) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `alternate_names`.\n"]
    pub fn set_alternate_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().alternate_names = Some(v.into());
        self
    }

    #[doc= "Set the field `signature_algorithm`.\n"]
    pub fn set_signature_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().signature_algorithm = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `alternate_names` after provisioning.\n"]
    pub fn alternate_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authority_identifier` after provisioning.\n"]
    pub fn authority_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authority_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_renew` after provisioning.\n"]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_renew", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `csr` after provisioning.\n"]
    pub fn csr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signature_algorithm` after provisioning.\n"]
    pub fn signature_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature_algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `years` after provisioning.\n"]
    pub fn years(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.years", self.extract_ref()))
    }
}

impl Referable for LetsEncryptCertificate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LetsEncryptCertificate { }

impl ToListMappable for LetsEncryptCertificate {
    type O = ListRef<LetsEncryptCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LetsEncryptCertificate_ {
    fn extract_resource_type(&self) -> String {
        "dnsimple_lets_encrypt_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLetsEncryptCertificate {
    pub tf_id: String,
    #[doc= ""]
    pub auto_renew: PrimField<bool>,
    #[doc= ""]
    pub domain_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildLetsEncryptCertificate {
    pub fn build(self, stack: &mut Stack) -> LetsEncryptCertificate {
        let out = LetsEncryptCertificate(Rc::new(LetsEncryptCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LetsEncryptCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alternate_names: core::default::Default::default(),
                auto_renew: self.auto_renew,
                domain_id: self.domain_id,
                name: self.name,
                signature_algorithm: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LetsEncryptCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for LetsEncryptCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LetsEncryptCertificateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alternate_names` after provisioning.\n"]
    pub fn alternate_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authority_identifier` after provisioning.\n"]
    pub fn authority_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authority_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_renew` after provisioning.\n"]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_renew", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `csr` after provisioning.\n"]
    pub fn csr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signature_algorithm` after provisioning.\n"]
    pub fn signature_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature_algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `years` after provisioning.\n"]
    pub fn years(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.years", self.extract_ref()))
    }
}
