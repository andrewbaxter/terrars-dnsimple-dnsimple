use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct ContactData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    address1: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address2: std::option::Option<terrars::PrimField<std::string::String>>,
    city: terrars::PrimField<std::string::String>,
    country: terrars::PrimField<std::string::String>,
    email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax: std::option::Option<terrars::PrimField<std::string::String>>,
    first_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    last_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_name: std::option::Option<terrars::PrimField<std::string::String>>,
    phone: terrars::PrimField<std::string::String>,
    postal_code: terrars::PrimField<std::string::String>,
    state_province: terrars::PrimField<std::string::String>,
}

struct Contact_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContactData>,
}

#[derive(Clone)]
pub struct Contact(std::rc::Rc<Contact_>);

impl Contact {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDnsimple) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `address2`.\n"]
    pub fn set_address2(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fax = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_title`.\n"]
    pub fn set_job_title(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().organization_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address1` after provisioning.\n"]
    pub fn address1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address2` after provisioning.\n"]
    pub fn address2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fax_normalized` after provisioning.\n"]
    pub fn fax_normalized(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax_normalized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_title` after provisioning.\n"]
    pub fn job_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone` after provisioning.\n"]
    pub fn phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_normalized` after provisioning.\n"]
    pub fn phone_normalized(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_normalized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_province` after provisioning.\n"]
    pub fn state_province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_province", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for Contact {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Contact { }

impl terrars::ToListMappable for Contact {
    type O = terrars::ListRef<ContactRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Contact_ {
    fn extract_resource_type(&self) -> std::string::String {
        "dnsimple_contact".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContact {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub address1: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub city: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub country: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub first_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub last_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub phone: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub postal_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_province: terrars::PrimField<std::string::String>,
}

impl BuildContact {
    pub fn build(self, stack: &mut terrars::Stack) -> Contact {
        let out = Contact(std::rc::Rc::new(Contact_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContactData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContactRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address1` after provisioning.\n"]
    pub fn address1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address2` after provisioning.\n"]
    pub fn address2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fax_normalized` after provisioning.\n"]
    pub fn fax_normalized(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax_normalized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_title` after provisioning.\n"]
    pub fn job_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone` after provisioning.\n"]
    pub fn phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_normalized` after provisioning.\n"]
    pub fn phone_normalized(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_normalized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_province` after provisioning.\n"]
    pub fn state_province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_province", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}
