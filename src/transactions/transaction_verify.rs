use crate::fwcall::{FwCall, ToFwCall};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyTransByIdReq {
    pub trans_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyTransByTxRefReq {
    pub tx_ref: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTransRes {
    pub status: String,
    pub message: String,
    pub data: VerifyTransResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTransResData {
    pub id: i64,
    pub tx_ref: String,
    pub flw_ref: String,
    pub device_fingerprint: String,
    pub amount: i64,
    pub currency: String,
    pub charged_amount: i64,
    pub app_fee: f64,
    pub merchant_fee: i64,
    pub processor_response: String,
    pub auth_model: String,
    pub ip: String,
    pub narration: String,
    pub status: String,
    pub payment_type: String,
    pub created_at: String,
    pub account_id: i64,
    pub card: Card,
    pub meta: Option<HashMap<String, serde_json::Value>>,
    pub amount_settled: f64,
    pub customer: Customer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "first_6digits")]
    pub first_6_digits: String,
    #[serde(rename = "last_4digits")]
    pub last_4_digits: String,
    pub issuer: String,
    pub country: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub token: String,
    pub expiry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub created_at: String,
}

impl<'a> ToFwCall<'a> for VerifyTransByIdReq {
    type ApiRequest = Self;

    type ApiResponse = VerifyTransRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/transactions/{}/verify", self.trans_id)),
            reqwest::Method::GET,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for VerifyTransByTxRefReq {
    type ApiRequest = Self;

    type ApiResponse = VerifyTransRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/transactions/verify_by_reference?tx_ref={}",
                self.tx_ref
            )),
            reqwest::Method::GET,
            None,
        )
    }
}
