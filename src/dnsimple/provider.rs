use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderDnsimpleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefetch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<PrimField<String>>,
}

struct ProviderDnsimple_ {
    data: RefCell<ProviderDnsimpleData>,
}

pub struct ProviderDnsimple(Rc<ProviderDnsimple_>);

impl ProviderDnsimple {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "dnsimple", alias)
        } else {
            "dnsimple".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `account`.\nThe account for API operations."]
    pub fn set_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account = Some(v.into());
        self
    }

    #[doc= "Set the field `prefetch`.\nFlag to enable the prefetch of zone records."]
    pub fn set_prefetch(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prefetch = Some(v.into());
        self
    }

    #[doc= "Set the field `sandbox`.\nFlag to enable the sandbox API."]
    pub fn set_sandbox(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sandbox = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\nThe API v2 token for API operations."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }

    #[doc= "Set the field `user_agent`.\nCustom string to append to the user agent used for sending HTTP requests to the API."]
    pub fn set_user_agent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_agent = Some(v.into());
        self
    }
}

impl Provider for ProviderDnsimple_ {
    fn extract_type_tf_id(&self) -> String {
        "dnsimple".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "dnsimple/dnsimple",
            "version": "1.3.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDnsimple {}

impl BuildProviderDnsimple {
    pub fn build(self, stack: &mut Stack) -> ProviderDnsimple {
        let out = ProviderDnsimple(Rc::new(ProviderDnsimple_ { data: RefCell::new(ProviderDnsimpleData {
            alias: None,
            account: core::default::Default::default(),
            prefetch: core::default::Default::default(),
            sandbox: core::default::Default::default(),
            token: core::default::Default::default(),
            user_agent: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
