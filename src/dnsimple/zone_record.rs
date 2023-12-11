use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct ZoneRecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
    zone_name: PrimField<String>,
}

struct ZoneRecord_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ZoneRecordData>,
}

#[derive(Clone)]
pub struct ZoneRecord(Rc<ZoneRecord_>);

impl ZoneRecord {
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

    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `regions`.\n"]
    pub fn set_regions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().regions = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `qualified_name` after provisioning.\n"]
    pub fn qualified_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.qualified_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value_normalized` after provisioning.\n"]
    pub fn value_normalized(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_normalized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_name` after provisioning.\n"]
    pub fn zone_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.extract_ref()))
    }
}

impl Referable for ZoneRecord {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ZoneRecord { }

impl ToListMappable for ZoneRecord {
    type O = ListRef<ZoneRecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ZoneRecord_ {
    fn extract_resource_type(&self) -> String {
        "dnsimple_zone_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneRecord {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<String>,
    #[doc= ""]
    pub zone_name: PrimField<String>,
}

impl BuildZoneRecord {
    pub fn build(self, stack: &mut Stack) -> ZoneRecord {
        let out = ZoneRecord(Rc::new(ZoneRecord_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ZoneRecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                name: self.name,
                priority: core::default::Default::default(),
                regions: core::default::Default::default(),
                ttl: core::default::Default::default(),
                type_: self.type_,
                value: self.value,
                zone_name: self.zone_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneRecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneRecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneRecordRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `qualified_name` after provisioning.\n"]
    pub fn qualified_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.qualified_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value_normalized` after provisioning.\n"]
    pub fn value_normalized(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_normalized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_name` after provisioning.\n"]
    pub fn zone_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.extract_ref()))
    }
}
