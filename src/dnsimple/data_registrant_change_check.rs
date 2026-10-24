use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct DataRegistrantChangeCheckData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    contact_id: terrars::PrimField<std::string::String>,
    domain_id: terrars::PrimField<std::string::String>,
}

struct DataRegistrantChangeCheck_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRegistrantChangeCheckData>,
}

#[derive(Clone)]
pub struct DataRegistrantChangeCheck(std::rc::Rc<DataRegistrantChangeCheck_>);

impl DataRegistrantChangeCheck {
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

    #[doc =
        "Get a reference to the value of field `contact_id` after provisioning.\nDNSimple contact ID for which the registrant change check is being performed"]
    pub fn contact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_id` after provisioning.\nDNSimple domain ID for which the registrant change check is being performed"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `extended_attributes` after provisioning.\nExtended attributes for the registrant change"]
    pub fn extended_attributes(&self) -> terrars::ListRef<DataRegistrantChangeCheckExtendedAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_owner_change` after provisioning.\nTrue if the registrant change will result in a registry owner change"]
    pub fn registry_owner_change(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_owner_change", self.extract_ref()))
    }
}

impl terrars::Referable for DataRegistrantChangeCheck {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRegistrantChangeCheck { }

impl terrars::ToListMappable for DataRegistrantChangeCheck {
    type O = terrars::ListRef<DataRegistrantChangeCheckRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRegistrantChangeCheck_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "dnsimple_registrant_change_check".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRegistrantChangeCheck {
    pub tf_id: std::string::String,
    #[doc = "DNSimple contact ID for which the registrant change check is being performed"]
    pub contact_id: terrars::PrimField<std::string::String>,
    #[doc = "DNSimple domain ID for which the registrant change check is being performed"]
    pub domain_id: terrars::PrimField<std::string::String>,
}

impl BuildDataRegistrantChangeCheck {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRegistrantChangeCheck {
        let out = DataRegistrantChangeCheck(std::rc::Rc::new(DataRegistrantChangeCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRegistrantChangeCheckData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                contact_id: self.contact_id,
                domain_id: self.domain_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRegistrantChangeCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistrantChangeCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRegistrantChangeCheckRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `contact_id` after provisioning.\nDNSimple contact ID for which the registrant change check is being performed"]
    pub fn contact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_id` after provisioning.\nDNSimple domain ID for which the registrant change check is being performed"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `extended_attributes` after provisioning.\nExtended attributes for the registrant change"]
    pub fn extended_attributes(&self) -> terrars::ListRef<DataRegistrantChangeCheckExtendedAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_owner_change` after provisioning.\nTrue if the registrant change will result in a registry owner change"]
    pub fn registry_owner_change(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_owner_change", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    type O = terrars::BlockAssignable<DataRegistrantChangeCheckExtendedAttributesElOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegistrantChangeCheckExtendedAttributesElOptionsEl {}

impl BuildDataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    pub fn build(self) -> DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
        DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
            description: core::default::Default::default(),
            title: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
        DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRegistrantChangeCheckExtendedAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::ListField<DataRegistrantChangeCheckExtendedAttributesElOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
}

impl DataRegistrantChangeCheckExtendedAttributesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRegistrantChangeCheckExtendedAttributesElOptionsEl>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRegistrantChangeCheckExtendedAttributesEl {
    type O = terrars::BlockAssignable<DataRegistrantChangeCheckExtendedAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegistrantChangeCheckExtendedAttributesEl {}

impl BuildDataRegistrantChangeCheckExtendedAttributesEl {
    pub fn build(self) -> DataRegistrantChangeCheckExtendedAttributesEl {
        DataRegistrantChangeCheckExtendedAttributesEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            options: core::default::Default::default(),
            required: core::default::Default::default(),
        }
    }
}

pub struct DataRegistrantChangeCheckExtendedAttributesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistrantChangeCheckExtendedAttributesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRegistrantChangeCheckExtendedAttributesElRef {
        DataRegistrantChangeCheckExtendedAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegistrantChangeCheckExtendedAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<DataRegistrantChangeCheckExtendedAttributesElOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
}
