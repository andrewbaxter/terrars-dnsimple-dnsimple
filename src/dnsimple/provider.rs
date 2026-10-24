use serde::Serialize;

#[derive(Serialize)]
struct ProviderDnsimpleData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefetch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sandbox: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderDnsimple_ {
    data: std::cell::RefCell<ProviderDnsimpleData>,
}

pub struct ProviderDnsimple(std::rc::Rc<ProviderDnsimple_>);

impl ProviderDnsimple {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "dnsimple", alias)
        } else {
            "dnsimple".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `account`.\nThe account for API operations."]
    pub fn set_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefetch`.\nFlag to enable the prefetch of zone records."]
    pub fn set_prefetch(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prefetch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sandbox`.\nFlag to enable the sandbox API."]
    pub fn set_sandbox(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sandbox = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\nThe API v2 token for API operations."]
    pub fn set_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_agent`.\nCustom string to append to the user agent used for sending HTTP requests to the API."]
    pub fn set_user_agent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderDnsimple_ {
    fn extract_type_tf_id(&self) -> std::string::String {
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
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderDnsimple {
        let out =
            ProviderDnsimple(std::rc::Rc::new(ProviderDnsimple_ { data: std::cell::RefCell::new(ProviderDnsimpleData {
                alias: std::option::Option::None,
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
