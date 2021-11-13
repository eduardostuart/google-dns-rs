use crate::error;
use serde::Deserialize;

/// The comments here described were (partially) copied from
/// JSON API specification reference:
/// https://developers.google.com/speed/public-dns/docs/doh/json

/// Result using default error type error::Error [`Error`]
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

const GOOGLEDNS_BASE_URL: &str = "https://dns.google";

#[derive(Debug, Clone, Deserialize)]
pub struct DnsQuestion {
    /// Fully qualified domain name with trailing dot
    pub name: String,
    /// Standard DNS RR type
    pub r#type: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DnsAnswer {
    /// RR type represented as a number
    pub r#type: u32,
    /// Record's time-to-live in seconds
    #[serde(rename = "TTL")]
    pub ttl: u32,
    /// The value for "type"
    pub data: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dns {
    #[serde(rename = "Status")]
    /// NOERROR - Standard DNS response code (32 bit integer).
    pub status: u32,
    /// The response will be trucated if Google DNS cannot get complete
    /// and un-truncated responses from authoritative name servers or in cases
    /// where the DNS response (binary DNS message form) would exceed the 64 KiB
    /// limit for TCP DNS messages
    #[serde(rename = "TC")]
    pub tc: bool,
    /// Whether all response data was validated with DNSSEC
    #[serde(rename = "AD")]
    pub ad: bool,
    /// Whether the client asked to disable DNSSEC
    #[serde(rename = "CD")]
    pub cd: bool,
    #[serde(rename = "Question")]
    pub question: Vec<DnsQuestion>,
    #[serde(rename = "Answer")]
    pub answer: Option<Vec<DnsAnswer>>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DoH<'a> {
    // Google DNS api base URL
    base_url: &'a str,

    pub name: String,
    /// RR type
    /// default = 1
    pub r#type: Option<u32>,
    /// The checking disabled flag
    /// Use true to disable DNSSEC or false to enable DNSSEC validation
    /// default = false
    pub cd: Option<bool>,
    /// Desired content type
    /// ct=application/dns-message for binary DNS-message
    /// ct=application/x-javascript for JSON text
    /// default = empty
    pub ct: Option<String>,
    /// DNSSEC OK flag
    /// if true, the DNSSEC records will be included in the response (RRSIG, NSEC, NSEC3)
    /// or false to ommit these values in the response
    /// default = false
    pub r#do: Option<bool>,
    /// Use IP address format with a subnet mask
    /// Examples: 1.2.3.4/24, 2001:700:300::/48
    /// default = empty
    pub edns_client_subnet: Option<String>,
}

impl<'a> DoH<'a> {
    pub fn new(name: String) -> Self {
        DoH {
            base_url: GOOGLEDNS_BASE_URL,
            name,
            r#type: None,
            cd: None,
            ct: None,
            r#do: None,
            edns_client_subnet: None,
        }
    }

    pub fn set_base_url(mut self, value: &'a str) -> Self {
        self.base_url = value;
        self
    }

    /// Sets the desired content type
    pub fn set_ct(mut self, value: String) -> Self {
        self.ct = Some(value);
        self
    }

    /// Sets the RR type
    pub fn set_type(mut self, value: u32) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Disable or enable DNSSEC check
    pub fn set_cd(mut self, value: bool) -> Self {
        self.cd = Some(value);
        self
    }

    /// Include or ommit DNSSEC records
    pub fn set_do(mut self, value: bool) -> Self {
        self.r#do = Some(value);
        self
    }

    /// Include or ommit DNSSEC records
    pub fn set_edns_client_subnet(mut self, value: String) -> Self {
        self.edns_client_subnet = Some(value);
        self
    }

    pub async fn resolve(&self) -> Result<Dns> {
        let url = format!(
            "{}/resolve?name={name}{cd}{ct}{edns_client_subnet}{type}",
            &self.base_url,
            name = &self.name,
            r#type = match &self.r#type {
                Some(v) => format!("&type={}", v),
                None => "".to_string(),
            },
            ct = match &self.ct {
                Some(v) => format!("&ct={}", v),
                None => "".to_string(),
            },
            cd = match &self.cd {
                Some(v) => format!("&cd={}", v),
                None => "".to_string(),
            },
            edns_client_subnet = match &self.edns_client_subnet {
                Some(v) => format!("&edns_client_subnet={}", v),
                None => "".to_string(),
            }
        );

        Ok(ureq::get(&url).call()?.into_json()?)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::*;

    async fn setup_mock_api(response: ResponseTemplate) -> MockServer {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/resolve"))
            .respond_with(response)
            .mount(&server)
            .await;
        server
    }

    #[async_std::test]
    async fn should_return_dns_information() {
        let body: Value = serde_json::from_str(include_str!("../samples/google_A.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(body);
        let server = setup_mock_api(template).await;
        let result = DoH::new("google.com".to_string())
            .set_base_url(&server.uri())
            .resolve()
            .await
            .unwrap();

        assert_eq!(result.status, 0);
        assert_eq!(result.tc, false);
        assert_eq!(result.ad, false);
        assert_eq!(result.cd, false);
        assert_eq!(
            result.comment,
            Some("Response from 2001:4860:4802:32::a.".to_string())
        );

        let answer = result.answer.unwrap().into_iter().nth(0).unwrap();
        assert_eq!(answer.r#type, 1);
        assert_eq!(answer.ttl, 300);
        assert_eq!(answer.data, "216.58.208.110".to_string());

        let question = result.question.into_iter().nth(0).unwrap();
        assert_eq!(question.r#type, 1);
        assert_eq!(question.name, "google.com.");
    }
}
