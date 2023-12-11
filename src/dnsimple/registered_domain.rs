use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDnsimple;

#[derive(Serialize)]
struct RegisteredDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_renew_enabled: Option<PrimField<bool>>,
    contact_id: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dnssec_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_attributes: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    premium_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RegisteredDomainTimeouts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lock_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    whois_privacy_enabled: Option<PrimField<bool>>,
}

struct RegisteredDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RegisteredDomainData>,
}

#[derive(Clone)]
pub struct RegisteredDomain(Rc<RegisteredDomain_>);

impl RegisteredDomain {
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

    #[doc= "Set the field `auto_renew_enabled`.\n"]
    pub fn set_auto_renew_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_renew_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `dnssec_enabled`.\n"]
    pub fn set_dnssec_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dnssec_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `extended_attributes`.\n"]
    pub fn set_extended_attributes(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().extended_attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `premium_price`.\n"]
    pub fn set_premium_price(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().premium_price = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\nTimeouts for operations, given as a parsable string as in `10m` or `30s`."]
    pub fn set_timeouts(self, v: impl Into<RegisteredDomainTimeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Set the field `transfer_lock_enabled`.\n"]
    pub fn set_transfer_lock_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transfer_lock_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `whois_privacy_enabled`.\n"]
    pub fn set_whois_privacy_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().whois_privacy_enabled = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_renew_enabled` after provisioning.\n"]
    pub fn auto_renew_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_renew_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnssec_enabled` after provisioning.\n"]
    pub fn dnssec_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnssec_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_registration` after provisioning.\nThe domain registration details."]
    pub fn domain_registration(&self) -> RegisteredDomainDomainRegistrationRef {
        RegisteredDomainDomainRegistrationRef::new(
            self.shared().clone(),
            format!("{}.domain_registration", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extended_attributes` after provisioning.\n"]
    pub fn extended_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extended_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `premium_price` after provisioning.\n"]
    pub fn premium_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.premium_price", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registrant_change` after provisioning.\nThe registrant change details."]
    pub fn registrant_change(&self) -> RegisteredDomainRegistrantChangeRef {
        RegisteredDomainRegistrantChangeRef::new(
            self.shared().clone(),
            format!("{}.registrant_change", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\nTimeouts for operations, given as a parsable string as in `10m` or `30s`."]
    pub fn timeouts(&self) -> RegisteredDomainTimeoutsRef {
        RegisteredDomainTimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transfer_lock_enabled` after provisioning.\n"]
    pub fn transfer_lock_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.transfer_lock_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unicode_name` after provisioning.\n"]
    pub fn unicode_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unicode_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `whois_privacy_enabled` after provisioning.\n"]
    pub fn whois_privacy_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.whois_privacy_enabled", self.extract_ref()))
    }
}

impl Referable for RegisteredDomain {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RegisteredDomain { }

impl ToListMappable for RegisteredDomain {
    type O = ListRef<RegisteredDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RegisteredDomain_ {
    fn extract_resource_type(&self) -> String {
        "dnsimple_registered_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRegisteredDomain {
    pub tf_id: String,
    #[doc= ""]
    pub contact_id: PrimField<f64>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildRegisteredDomain {
    pub fn build(self, stack: &mut Stack) -> RegisteredDomain {
        let out = RegisteredDomain(Rc::new(RegisteredDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RegisteredDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_renew_enabled: core::default::Default::default(),
                contact_id: self.contact_id,
                dnssec_enabled: core::default::Default::default(),
                extended_attributes: core::default::Default::default(),
                name: self.name,
                premium_price: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transfer_lock_enabled: core::default::Default::default(),
                whois_privacy_enabled: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RegisteredDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for RegisteredDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RegisteredDomainRef {
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

    #[doc= "Get a reference to the value of field `auto_renew_enabled` after provisioning.\n"]
    pub fn auto_renew_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_renew_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnssec_enabled` after provisioning.\n"]
    pub fn dnssec_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnssec_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_registration` after provisioning.\nThe domain registration details."]
    pub fn domain_registration(&self) -> RegisteredDomainDomainRegistrationRef {
        RegisteredDomainDomainRegistrationRef::new(
            self.shared().clone(),
            format!("{}.domain_registration", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extended_attributes` after provisioning.\n"]
    pub fn extended_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extended_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `premium_price` after provisioning.\n"]
    pub fn premium_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.premium_price", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registrant_change` after provisioning.\nThe registrant change details."]
    pub fn registrant_change(&self) -> RegisteredDomainRegistrantChangeRef {
        RegisteredDomainRegistrantChangeRef::new(
            self.shared().clone(),
            format!("{}.registrant_change", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\nTimeouts for operations, given as a parsable string as in `10m` or `30s`."]
    pub fn timeouts(&self) -> RegisteredDomainTimeoutsRef {
        RegisteredDomainTimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transfer_lock_enabled` after provisioning.\n"]
    pub fn transfer_lock_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.transfer_lock_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unicode_name` after provisioning.\n"]
    pub fn unicode_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unicode_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `whois_privacy_enabled` after provisioning.\n"]
    pub fn whois_privacy_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.whois_privacy_enabled", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RegisteredDomainDomainRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl RegisteredDomainDomainRegistration {
    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for RegisteredDomainDomainRegistration {
    type O = BlockAssignable<RegisteredDomainDomainRegistration>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegisteredDomainDomainRegistration {}

impl BuildRegisteredDomainDomainRegistration {
    pub fn build(self) -> RegisteredDomainDomainRegistration {
        RegisteredDomainDomainRegistration { state: core::default::Default::default() }
    }
}

pub struct RegisteredDomainDomainRegistrationRef {
    shared: StackShared,
    base: String,
}

impl Ref for RegisteredDomainDomainRegistrationRef {
    fn new(shared: StackShared, base: String) -> RegisteredDomainDomainRegistrationRef {
        RegisteredDomainDomainRegistrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegisteredDomainDomainRegistrationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct RegisteredDomainRegistrantChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl RegisteredDomainRegistrantChange {
    #[doc= "Set the field `state`.\nState of the registrant change"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for RegisteredDomainRegistrantChange {
    type O = BlockAssignable<RegisteredDomainRegistrantChange>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegisteredDomainRegistrantChange {}

impl BuildRegisteredDomainRegistrantChange {
    pub fn build(self) -> RegisteredDomainRegistrantChange {
        RegisteredDomainRegistrantChange { state: core::default::Default::default() }
    }
}

pub struct RegisteredDomainRegistrantChangeRef {
    shared: StackShared,
    base: String,
}

impl Ref for RegisteredDomainRegistrantChangeRef {
    fn new(shared: StackShared, base: String) -> RegisteredDomainRegistrantChangeRef {
        RegisteredDomainRegistrantChangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegisteredDomainRegistrantChangeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nDNSimple Account ID to which the registrant change belongs to"]
    pub fn account_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc= "Get a reference to the value of field `contact_id` after provisioning.\nDNSimple contact ID for which the registrant change is being performed"]
    pub fn contact_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.base))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\nDNSimple domain ID for which the registrant change is being performed"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.base))
    }

    #[doc= "Get a reference to the value of field `extended_attributes` after provisioning.\nExtended attributes for the registrant change"]
    pub fn extended_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extended_attributes", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `irt_lock_lifted_by` after provisioning.\nDate when the registrant change lock was lifted for the domain"]
    pub fn irt_lock_lifted_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.irt_lock_lifted_by", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_owner_change` after provisioning.\nTrue if the registrant change will result in a registry owner change"]
    pub fn registry_owner_change(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_owner_change", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the registrant change"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct RegisteredDomainTimeouts {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RegisteredDomainTimeouts {
    #[doc= "Set the field `create`.\nCreate timeout."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\nDelete timeout (currently unused)."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\nUpdate timeout."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RegisteredDomainTimeouts {
    type O = BlockAssignable<RegisteredDomainTimeouts>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegisteredDomainTimeouts {}

impl BuildRegisteredDomainTimeouts {
    pub fn build(self) -> RegisteredDomainTimeouts {
        RegisteredDomainTimeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RegisteredDomainTimeoutsRef {
    shared: StackShared,
    base: String,
}

impl Ref for RegisteredDomainTimeoutsRef {
    fn new(shared: StackShared, base: String) -> RegisteredDomainTimeoutsRef {
        RegisteredDomainTimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegisteredDomainTimeoutsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\nCreate timeout."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\nDelete timeout (currently unused)."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\nUpdate timeout."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
