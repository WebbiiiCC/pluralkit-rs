use reqwest::Method;
use crate::{PKClient, PKResult, PKToken};
use crate::model::members::Member;
use crate::ratelimit::scope::RateLimitScope;

pub async fn get_system_members(client: &PKClient, token: Option<PKToken<'_>>, system_ref: &str) -> PKResult<Vec<Member>> {
    client.get(token, RateLimitScope::GenericGet, format!("/systems/{system_ref}/members")).await
}

pub async fn create_member(client: &PKClient, token: PKToken<'_>, member: &Member) -> PKResult<Member> {
    client.request(
        Some(token),
        RateLimitScope::GenericUpdate,
        Method::POST,
        "/members",
        Some(member),
        true
    ).await.transpose().unwrap()
}

pub async fn get_member(client: &PKClient, token: Option<PKToken<'_>>, member_ref: &str) -> PKResult<Member> {
    client.get(token, RateLimitScope::GenericGet, format!("/members/{member_ref}")).await
}

pub async fn update_member(client: &PKClient, token: PKToken<'_>, member: &Member) -> PKResult<Member> {
    client.request(
        Some(token),
        RateLimitScope::GenericUpdate,
        Method::PATCH,
        format!("/members/{}", member.uuid),
        Some(member),
        true
    ).await.transpose().unwrap()
}

pub async fn delete_member(client: &PKClient, token: PKToken<'_>, member_ref: &str) -> PKResult<()> {
    client.delete(token, RateLimitScope::GenericUpdate, format!("/members/{member_ref}")).await
}