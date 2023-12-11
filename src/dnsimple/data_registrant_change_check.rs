use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct DataRegistrantChangeCheckData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    contact_id: PrimField<String>,
    domain_id: PrimField<String>,
}

struct DataRegistrantChangeCheck_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRegistrantChangeCheckData>,
}

#[derive(Clone)]
pub struct DataRegistrantChangeCheck(Rc<DataRegistrantChangeCheck_>);

impl DataRegistrantChangeCheck {
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

    #[doc= "Get a reference to the value of field `contact_id` after provisioning.\nDNSimple contact ID for which the registrant change check is being performed"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\nDNSimple domain ID for which the registrant change check is being performed"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extended_attributes` after provisioning.\nExtended attributes for the registrant change"]
    pub fn extended_attributes(&self) -> ListRef<DataRegistrantChangeCheckExtendedAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extended_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registry_owner_change` after provisioning.\nTrue if the registrant change will result in a registry owner change"]
    pub fn registry_owner_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_owner_change", self.extract_ref()))
    }
}

impl Referable for DataRegistrantChangeCheck {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRegistrantChangeCheck { }

impl ToListMappable for DataRegistrantChangeCheck {
    type O = ListRef<DataRegistrantChangeCheckRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRegistrantChangeCheck_ {
    fn extract_datasource_type(&self) -> String {
        "dnsimple_registrant_change_check".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRegistrantChangeCheck {
    pub tf_id: String,
    #[doc= "DNSimple contact ID for which the registrant change check is being performed"]
    pub contact_id: PrimField<String>,
    #[doc= "DNSimple domain ID for which the registrant change check is being performed"]
    pub domain_id: PrimField<String>,
}

impl BuildDataRegistrantChangeCheck {
    pub fn build(self, stack: &mut Stack) -> DataRegistrantChangeCheck {
        let out = DataRegistrantChangeCheck(Rc::new(DataRegistrantChangeCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRegistrantChangeCheckData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                contact_id: self.contact_id,
                domain_id: self.domain_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRegistrantChangeCheckRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRegistrantChangeCheckRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRegistrantChangeCheckRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `contact_id` after provisioning.\nDNSimple contact ID for which the registrant change check is being performed"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\nDNSimple domain ID for which the registrant change check is being performed"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extended_attributes` after provisioning.\nExtended attributes for the registrant change"]
    pub fn extended_attributes(&self) -> ListRef<DataRegistrantChangeCheckExtendedAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extended_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registry_owner_change` after provisioning.\nTrue if the registrant change will result in a registry owner change"]
    pub fn registry_owner_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_owner_change", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRegistrantChangeCheckExtendedAttributesElOptionsEl {
    type O = BlockAssignable<DataRegistrantChangeCheckExtendedAttributesElOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
        DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegistrantChangeCheckExtendedAttributesElOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRegistrantChangeCheckExtendedAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ListField<DataRegistrantChangeCheckExtendedAttributesElOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
}

impl DataRegistrantChangeCheckExtendedAttributesEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl Into<ListField<DataRegistrantChangeCheckExtendedAttributesElOptionsEl>>,
    ) -> Self {
        self.options = Some(v.into());
        self
    }

    #[doc= "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }
}

impl ToListMappable for DataRegistrantChangeCheckExtendedAttributesEl {
    type O = BlockAssignable<DataRegistrantChangeCheckExtendedAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataRegistrantChangeCheckExtendedAttributesElRef {
    fn new(shared: StackShared, base: String) -> DataRegistrantChangeCheckExtendedAttributesElRef {
        DataRegistrantChangeCheckExtendedAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegistrantChangeCheckExtendedAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<DataRegistrantChangeCheckExtendedAttributesElOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc= "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
}
