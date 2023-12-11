use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct ContactData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    address1: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address2: Option<PrimField<String>>,
    city: PrimField<String>,
    country: PrimField<String>,
    email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<PrimField<String>>,
    first_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    last_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_name: Option<PrimField<String>>,
    phone: PrimField<String>,
    postal_code: PrimField<String>,
    state_province: PrimField<String>,
}

struct Contact_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactData>,
}

#[derive(Clone)]
pub struct Contact(Rc<Contact_>);

impl Contact {
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

    #[doc= "Set the field `address2`.\n"]
    pub fn set_address2(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address2 = Some(v.into());
        self
    }

    #[doc= "Set the field `fax`.\n"]
    pub fn set_fax(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fax = Some(v.into());
        self
    }

    #[doc= "Set the field `job_title`.\n"]
    pub fn set_job_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().job_title = Some(v.into());
        self
    }

    #[doc= "Set the field `label`.\n"]
    pub fn set_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().label = Some(v.into());
        self
    }

    #[doc= "Set the field `organization_name`.\n"]
    pub fn set_organization_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().organization_name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address1` after provisioning.\n"]
    pub fn address1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address1", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address2` after provisioning.\n"]
    pub fn address2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fax_normalized` after provisioning.\n"]
    pub fn fax_normalized(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax_normalized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_title` after provisioning.\n"]
    pub fn job_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `phone` after provisioning.\n"]
    pub fn phone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `phone_normalized` after provisioning.\n"]
    pub fn phone_normalized(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_normalized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_province` after provisioning.\n"]
    pub fn state_province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_province", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for Contact {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Contact { }

impl ToListMappable for Contact {
    type O = ListRef<ContactRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Contact_ {
    fn extract_resource_type(&self) -> String {
        "dnsimple_contact".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContact {
    pub tf_id: String,
    #[doc= ""]
    pub address1: PrimField<String>,
    #[doc= ""]
    pub city: PrimField<String>,
    #[doc= ""]
    pub country: PrimField<String>,
    #[doc= ""]
    pub email: PrimField<String>,
    #[doc= ""]
    pub first_name: PrimField<String>,
    #[doc= ""]
    pub last_name: PrimField<String>,
    #[doc= ""]
    pub phone: PrimField<String>,
    #[doc= ""]
    pub postal_code: PrimField<String>,
    #[doc= ""]
    pub state_province: PrimField<String>,
}

impl BuildContact {
    pub fn build(self, stack: &mut Stack) -> Contact {
        let out = Contact(Rc::new(Contact_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContactData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                address1: self.address1,
                address2: core::default::Default::default(),
                city: self.city,
                country: self.country,
                email: self.email,
                fax: core::default::Default::default(),
                first_name: self.first_name,
                job_title: core::default::Default::default(),
                label: core::default::Default::default(),
                last_name: self.last_name,
                organization_name: core::default::Default::default(),
                phone: self.phone,
                postal_code: self.postal_code,
                state_province: self.state_province,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContactRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContactRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContactRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address1` after provisioning.\n"]
    pub fn address1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address1", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address2` after provisioning.\n"]
    pub fn address2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fax_normalized` after provisioning.\n"]
    pub fn fax_normalized(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax_normalized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_title` after provisioning.\n"]
    pub fn job_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `phone` after provisioning.\n"]
    pub fn phone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `phone_normalized` after provisioning.\n"]
    pub fn phone_normalized(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_normalized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_province` after provisioning.\n"]
    pub fn state_province(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_province", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}
