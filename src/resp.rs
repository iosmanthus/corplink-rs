#[derive(serde::Deserialize)]
pub struct Resp<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(unused)]
    pub action: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct RespCompany {
    #[allow(unused)]
    pub name: String,
    pub zh_name: String,
    pub en_name: String,
    pub domain: String,
    #[allow(unused)]
    pub enable_self_signed: bool,
    #[allow(unused)]
    pub self_signed_cert: String,
    #[allow(unused)]
    pub enable_public_key: bool,
    #[allow(unused)]
    pub public_key: String,
}

#[derive(serde::Deserialize)]
pub struct RespLoginMethod {
    #[allow(unused)]
    pub login_enable_ldap: bool,
    #[allow(unused)]
    pub login_enable: bool,
    pub login_orders: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct RespTpsLoginMethod {
    pub alias: String,
    pub login_url: String,
    pub token: String,
}

#[derive(serde::Deserialize)]
pub struct RespCorplinkLoginMethod {
    #[allow(unused)]
    pub mfa: bool,
    pub auth: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct RespLogin {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct RespVpnInfo {
    pub api_port: u16,
    pub vpn_port: u16,
    pub ip: String,
    // 1 for tcp, 2 for udp, we only support udp for now
    pub protocol_mode: i32,
    // useless
    #[allow(unused)]
    pub name: String,
    pub en_name: String,
    #[allow(unused)]
    pub icon: String,
    #[allow(unused)]
    pub id: i32,
    #[allow(unused)]
    pub timeout: i32,
}

#[derive(serde::Deserialize)]
pub struct RespWgExtraInfo {
    pub vpn_mtu: u32,
    pub vpn_dns: String,
    #[allow(unused)]
    pub vpn_dns_backup: String,
    #[allow(unused)]
    pub vpn_dns_domain_split: Vec<String>,
    #[allow(unused)]
    pub vpn_route_full: Vec<String>,
    pub vpn_route_split: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct RespWgInfo {
    pub ip: String,
    #[allow(unused)]
    pub ipv6: String,
    pub ip_mask: String,
    pub public_key: String,
    pub setting: RespWgExtraInfo,
    #[allow(unused)]
    pub mode: u32,
}
