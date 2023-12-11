use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct DataZoneData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
}

struct DataZone_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataZoneData>,
}

#[derive(Clone)]
pub struct DataZone(Rc<DataZone_>);

impl DataZone {
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

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nDNSimple Account ID to which the zone belongs to"]
    pub fn account_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nZone Name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reverse` after provisioning.\nTrue if the zone is a reverse zone"]
    pub fn reverse(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reverse", self.extract_ref()))
    }
}

impl Referable for DataZone {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataZone { }

impl ToListMappable for DataZone {
    type O = ListRef<DataZoneRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataZone_ {
    fn extract_datasource_type(&self) -> String {
        "dnsimple_zone".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZone {
    pub tf_id: String,
    #[doc= "Zone Name"]
    pub name: PrimField<String>,
}

impl BuildDataZone {
    pub fn build(self, stack: &mut Stack) -> DataZone {
        let out = DataZone(Rc::new(DataZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataZoneData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataZoneRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nDNSimple Account ID to which the zone belongs to"]
    pub fn account_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nZone Name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reverse` after provisioning.\nTrue if the zone is a reverse zone"]
    pub fn reverse(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reverse", self.extract_ref()))
    }
}
