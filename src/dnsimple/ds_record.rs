use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct DsRecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    algorithm: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest_type: Option<PrimField<String>>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keytag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
}

struct DsRecord_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DsRecordData>,
}

#[derive(Clone)]
pub struct DsRecord(Rc<DsRecord_>);

impl DsRecord {
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

    #[doc= "Set the field `digest`.\n"]
    pub fn set_digest(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().digest = Some(v.into());
        self
    }

    #[doc= "Set the field `digest_type`.\n"]
    pub fn set_digest_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().digest_type = Some(v.into());
        self
    }

    #[doc= "Set the field `keytag`.\n"]
    pub fn set_keytag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().keytag = Some(v.into());
        self
    }

    #[doc= "Set the field `public_key`.\n"]
    pub fn set_public_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_key = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_type` after provisioning.\n"]
    pub fn digest_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keytag` after provisioning.\n"]
    pub fn keytag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keytag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for DsRecord {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DsRecord { }

impl ToListMappable for DsRecord {
    type O = ListRef<DsRecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DsRecord_ {
    fn extract_resource_type(&self) -> String {
        "dnsimple_ds_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDsRecord {
    pub tf_id: String,
    #[doc= ""]
    pub algorithm: PrimField<String>,
    #[doc= ""]
    pub domain: PrimField<String>,
}

impl BuildDsRecord {
    pub fn build(self, stack: &mut Stack) -> DsRecord {
        let out = DsRecord(Rc::new(DsRecord_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DsRecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                algorithm: self.algorithm,
                digest: core::default::Default::default(),
                digest_type: core::default::Default::default(),
                domain: self.domain,
                keytag: core::default::Default::default(),
                public_key: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DsRecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for DsRecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DsRecordRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_type` after provisioning.\n"]
    pub fn digest_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keytag` after provisioning.\n"]
    pub fn keytag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keytag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}
