pub use crate::prelude::*;

/// Credit balances for the organization's subscription, subscription surcharge, and one-off purchases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetCreditsUsageResponse {
    /// The API key the credit balances belong to.
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Status of the user account.
    #[serde(rename = "userStatus")]
    pub user_status: GetCreditsUsageResponseUserStatus,
    /// Whether a subscription plan is currently active. Null if the account has no subscription.
    #[serde(rename = "subActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_active: Option<bool>,
    /// Status of the subscription credits. Null if the account has no subscription.
    #[serde(rename = "subStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<GetCreditsUsageResponseSubStatus>,
    /// Total credits allowed under the subscription plan. Null if the account has no subscription.
    #[serde(rename = "subAllowedCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_allowed_credits: Option<i64>,
    /// Credits consumed under the subscription plan. Null if the account has no subscription.
    #[serde(rename = "subUsedCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_used_credits: Option<i64>,
    /// Status of the subscription surcharge (overage) credits. Null if the account has no subscription.
    #[serde(rename = "subSurchargeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_surcharge_status: Option<GetCreditsUsageResponseSubSurchargeStatus>,
    /// Total surcharge credits allowed. Null if the account has no subscription.
    #[serde(rename = "subSurchargeAllowedCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_surcharge_allowed_credits: Option<i64>,
    /// Surcharge credits consumed. Null if the account has no subscription.
    #[serde(rename = "subSurchargeUsedCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_surcharge_used_credits: Option<i64>,
    /// Whether surcharge (overage) usage is allowed on the subscription. Null if the account has no subscription.
    #[serde(rename = "subSurchargeAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_surcharge_allowed: Option<bool>,
    /// Whether the account has active one-off (pay-as-you-go) credits.
    #[serde(rename = "oneOffCreditsActive")]
    #[serde(default)]
    pub one_off_credits_active: bool,
    /// Total one-off credits purchased.
    #[serde(rename = "oneOffAllowedCredits")]
    #[serde(default)]
    pub one_off_allowed_credits: i64,
    /// One-off credits consumed.
    #[serde(rename = "oneOffUsedCredits")]
    #[serde(default)]
    pub one_off_used_credits: i64,
}

impl GetCreditsUsageResponse {
    pub fn builder() -> GetCreditsUsageResponseBuilder {
        <GetCreditsUsageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCreditsUsageResponseBuilder {
    api_key: Option<String>,
    user_status: Option<GetCreditsUsageResponseUserStatus>,
    sub_active: Option<bool>,
    sub_status: Option<GetCreditsUsageResponseSubStatus>,
    sub_allowed_credits: Option<i64>,
    sub_used_credits: Option<i64>,
    sub_surcharge_status: Option<GetCreditsUsageResponseSubSurchargeStatus>,
    sub_surcharge_allowed_credits: Option<i64>,
    sub_surcharge_used_credits: Option<i64>,
    sub_surcharge_allowed: Option<bool>,
    one_off_credits_active: Option<bool>,
    one_off_allowed_credits: Option<i64>,
    one_off_used_credits: Option<i64>,
}

impl GetCreditsUsageResponseBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn user_status(mut self, value: GetCreditsUsageResponseUserStatus) -> Self {
        self.user_status = Some(value);
        self
    }

    pub fn sub_active(mut self, value: bool) -> Self {
        self.sub_active = Some(value);
        self
    }

    pub fn sub_status(mut self, value: GetCreditsUsageResponseSubStatus) -> Self {
        self.sub_status = Some(value);
        self
    }

    pub fn sub_allowed_credits(mut self, value: i64) -> Self {
        self.sub_allowed_credits = Some(value);
        self
    }

    pub fn sub_used_credits(mut self, value: i64) -> Self {
        self.sub_used_credits = Some(value);
        self
    }

    pub fn sub_surcharge_status(
        mut self,
        value: GetCreditsUsageResponseSubSurchargeStatus,
    ) -> Self {
        self.sub_surcharge_status = Some(value);
        self
    }

    pub fn sub_surcharge_allowed_credits(mut self, value: i64) -> Self {
        self.sub_surcharge_allowed_credits = Some(value);
        self
    }

    pub fn sub_surcharge_used_credits(mut self, value: i64) -> Self {
        self.sub_surcharge_used_credits = Some(value);
        self
    }

    pub fn sub_surcharge_allowed(mut self, value: bool) -> Self {
        self.sub_surcharge_allowed = Some(value);
        self
    }

    pub fn one_off_credits_active(mut self, value: bool) -> Self {
        self.one_off_credits_active = Some(value);
        self
    }

    pub fn one_off_allowed_credits(mut self, value: i64) -> Self {
        self.one_off_allowed_credits = Some(value);
        self
    }

    pub fn one_off_used_credits(mut self, value: i64) -> Self {
        self.one_off_used_credits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCreditsUsageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetCreditsUsageResponseBuilder::api_key)
    /// - [`user_status`](GetCreditsUsageResponseBuilder::user_status)
    /// - [`one_off_credits_active`](GetCreditsUsageResponseBuilder::one_off_credits_active)
    /// - [`one_off_allowed_credits`](GetCreditsUsageResponseBuilder::one_off_allowed_credits)
    /// - [`one_off_used_credits`](GetCreditsUsageResponseBuilder::one_off_used_credits)
    pub fn build(self) -> Result<GetCreditsUsageResponse, BuildError> {
        Ok(GetCreditsUsageResponse {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            user_status: self
                .user_status
                .ok_or_else(|| BuildError::missing_field("user_status"))?,
            sub_active: self.sub_active,
            sub_status: self.sub_status,
            sub_allowed_credits: self.sub_allowed_credits,
            sub_used_credits: self.sub_used_credits,
            sub_surcharge_status: self.sub_surcharge_status,
            sub_surcharge_allowed_credits: self.sub_surcharge_allowed_credits,
            sub_surcharge_used_credits: self.sub_surcharge_used_credits,
            sub_surcharge_allowed: self.sub_surcharge_allowed,
            one_off_credits_active: self
                .one_off_credits_active
                .ok_or_else(|| BuildError::missing_field("one_off_credits_active"))?,
            one_off_allowed_credits: self
                .one_off_allowed_credits
                .ok_or_else(|| BuildError::missing_field("one_off_allowed_credits"))?,
            one_off_used_credits: self
                .one_off_used_credits
                .ok_or_else(|| BuildError::missing_field("one_off_used_credits"))?,
        })
    }
}
