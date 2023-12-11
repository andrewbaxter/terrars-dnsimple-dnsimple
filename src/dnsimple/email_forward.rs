use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct EmailForwardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    alias_name: PrimField<String>,
    destination_email: PrimField<String>,
    domain: PrimField<String>,
}

struct EmailForward_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmailForwardData>,
}

#[derive(Clone)]
pub struct EmailForward(Rc<EmailForward_>);

impl EmailForward {
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

    #[doc= "Get a reference to the value of field `alias_email` after provisioning.\n"]
    pub fn alias_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alias_name` after provisioning.\n"]
    pub fn alias_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_email` after provisioning.\n"]
    pub fn destination_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for EmailForward {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmailForward { }

impl ToListMappable for EmailForward {
    type O = ListRef<EmailForwardRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmailForward_ {
    fn extract_resource_type(&self) -> String {
        "dnsimple_email_forward".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmailForward {
    pub tf_id: String,
    #[doc= ""]
    pub alias_name: PrimField<String>,
    #[doc= ""]
    pub destination_email: PrimField<String>,
    #[doc= ""]
    pub domain: PrimField<String>,
}

impl BuildEmailForward {
    pub fn build(self, stack: &mut Stack) -> EmailForward {
        let out = EmailForward(Rc::new(EmailForward_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmailForwardData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alias_name: self.alias_name,
                destination_email: self.destination_email,
                domain: self.domain,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmailForwardRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailForwardRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmailForwardRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alias_email` after provisioning.\n"]
    pub fn alias_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alias_name` after provisioning.\n"]
    pub fn alias_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_email` after provisioning.\n"]
    pub fn destination_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
