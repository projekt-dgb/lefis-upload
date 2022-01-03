#![allow(dead_code, unused_imports)]

use yaserde_derive::{YaSerialize, YaDeserialize};
use std::io::{Read, Write};
use log::{warn, debug};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, PartialEq, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}

#[derive(Debug, PartialEq, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soap"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}

pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;


pub mod messages {

	use yaserde::{YaSerialize, YaDeserialize};
    use yaserde::de::from_str;
    use async_trait::async_trait;
    use yaserde::ser::to_string;
    use super::*;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetVersion")]
    pub struct GetVersionSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetVersion,
    }
    #[derive(Debug, Default, PartialEq, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetVersionResponse")]
    pub struct GetVersionSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetVersionResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetSchemaVersion")]
    pub struct GetSchemaVersionSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetSchemaVersion,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetSchemaVersionResponse")]
    pub struct GetSchemaVersionSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetSchemaVersionResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Login")]
    pub struct LoginSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::Login,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LoginResponse")]
    pub struct LoginSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::LoginResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Logout")]
    pub struct LogoutSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::Logout,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LogoutResponse")]
    pub struct LogoutSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::LogoutResponse,
    }

}

pub mod types {

    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetVersion",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct GetVersion {}
    #[derive(Debug, Default, PartialEq, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetVersionResponse",
        namespace = "http://www.aed-sicad.de/namespaces/svr",

    )]
    pub struct GetVersionResponse {
        #[yaserde(rename = "GetVersionResult", default)]
        pub get_version_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetSchemaVersion",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct GetSchemaVersion {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetSchemaVersionResponse",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct GetSchemaVersionResponse {
        #[yaserde(rename = "GetSchemaVersionResult", default)]
        pub get_schema_version_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Login",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct Login {
        #[yaserde(rename = "user", default)]
        pub user: Option<String>,
        #[yaserde(rename = "password", default)]
        pub password: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LoginResponse",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct LoginResponse {
        #[yaserde(rename = "LoginResult", default)]
        pub login_result: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Logout",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct Logout {
        #[yaserde(rename = "sessionId", default)]
        pub session_id: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LogoutResponse",
        namespace = "http://www.aed-sicad.de/namespaces/svr",
      
    )]
    pub struct LogoutResponse {}
}


pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

   	pub type GetVersionSoapIn = messages::GetVersionSoapIn;
    pub type GetVersionSoapOut = messages::GetVersionSoapOut;
    pub type GetSchemaVersionSoapIn = messages::GetSchemaVersionSoapIn;
    pub type GetSchemaVersionSoapOut = messages::GetSchemaVersionSoapOut;
    pub type LoginSoapIn = messages::LoginSoapIn;
    pub type LoginSoapOut = messages::LoginSoapOut;
    pub type LogoutSoapIn = messages::LogoutSoapIn;
    pub type LogoutSoapOut = messages::LogoutSoapOut;


    #[async_trait]
    pub trait AuftragsManagerSoap {
        async fn get_version(
            &self,
            get_version_soap_in: GetVersionSoapIn,
        ) -> Result<GetVersionSoapOut, Option<SoapFault>>;
        async fn get_schema_version(
            &self,
            get_schema_version_soap_in: GetSchemaVersionSoapIn,
        ) -> Result<GetSchemaVersionSoapOut, Option<SoapFault>>;
        async fn login(
            &self,
            login_soap_in: LoginSoapIn,
        ) -> Result<LoginSoapOut, Option<SoapFault>>;
        async fn logout(
            &self,
            logout_soap_in: LogoutSoapIn,
        ) -> Result<LogoutSoapOut, Option<SoapFault>>;
    }
}


pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl AuftragsManagerSoap {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetVersionSoapIn {
        #[yaserde(rename = "GetVersion", default)]
        pub body: ports::GetVersionSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct GetVersionSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapGetVersionSoapIn,
    }

    impl GetVersionSoapInSoapEnvelope {
        pub fn new(body: SoapGetVersionSoapIn) -> Self {
            GetVersionSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, PartialEq, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetVersionSoapOut {
        #[yaserde(rename = "GetVersionResponse", default)]
        pub body: ports::GetVersionSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, PartialEq, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]

    pub struct GetVersionSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapGetVersionSoapOut,
    }

    impl GetVersionSoapOutSoapEnvelope {
        pub fn new(body: SoapGetVersionSoapOut) -> Self {
            GetVersionSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: Option::Some("http://www.w3.org/2001/XMLSchema-instance".to_string()),
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetSchemaVersionSoapIn {
        #[yaserde(rename = "GetSchemaVersion", default)]
        pub body: ports::GetSchemaVersionSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct GetSchemaVersionSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapGetSchemaVersionSoapIn,
    }

    impl GetSchemaVersionSoapInSoapEnvelope {
        pub fn new(body: SoapGetSchemaVersionSoapIn) -> Self {
            GetSchemaVersionSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetSchemaVersionSoapOut {
        #[yaserde(rename = "GetSchemaVersionSoapOut", default)]
        pub body: ports::GetSchemaVersionSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct GetSchemaVersionSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapGetSchemaVersionSoapOut,
    }

    impl GetSchemaVersionSoapOutSoapEnvelope {
        pub fn new(body: SoapGetSchemaVersionSoapOut) -> Self {
            GetSchemaVersionSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLoginSoapIn {
        #[yaserde(rename = "Login", default)]
        pub body: ports::LoginSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct LoginSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapLoginSoapIn,
    }

    impl LoginSoapInSoapEnvelope {
        pub fn new(body: SoapLoginSoapIn) -> Self {
            LoginSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLoginSoapOut {
        #[yaserde(rename = "LoginSoapOut", default)]
        pub body: ports::LoginSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct LoginSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapLoginSoapOut,
    }

    impl LoginSoapOutSoapEnvelope {
        pub fn new(body: SoapLoginSoapOut) -> Self {
            LoginSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLogoutSoapIn {
        #[yaserde(rename = "Logout", default)]
        pub body: ports::LogoutSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct LogoutSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapLogoutSoapIn,
    }

    impl LogoutSoapInSoapEnvelope {
        pub fn new(body: SoapLogoutSoapIn) -> Self {
            LogoutSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLogoutSoapOut {
        #[yaserde(rename = "LogoutSoapOut", default)]
        pub body: ports::LogoutSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soap"
    )]
    pub struct LogoutSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapLogoutSoapOut,
    }

    impl LogoutSoapOutSoapEnvelope {
        pub fn new(body: SoapLogoutSoapOut) -> Self {
            LogoutSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }


    impl Default for AuftragsManagerSoap {
        fn default() -> Self {
            AuftragsManagerSoap {
                client: reqwest::Client::new(),
                url: "http://www.aed-sicad.de/namespaces/svr".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl AuftragsManagerSoap {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            AuftragsManagerSoap {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }

    #[derive(Clone)]
    pub struct AuftragsManagerSoap {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::AuftragsManagerSoap for AuftragsManagerSoap {
        async fn get_version(
            &self,
            get_version_soap_in: ports::GetVersionSoapIn,
        ) -> Result<ports::GetVersionSoapOut, Option<SoapFault>> {
            let __request = GetVersionSoapInSoapEnvelope::new(SoapGetVersionSoapIn {
                body: get_version_soap_in,
                xmlns: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.aed-sicad.de/namespaces/svr/GetVersion",
                )
                .await
                .map_err(|err| {
                    println!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            println!("response: {}", response);

            let r: GetVersionSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                println!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_schema_version(
            &self,
            get_schema_version_soap_in: ports::GetSchemaVersionSoapIn,
        ) -> Result<ports::GetSchemaVersionSoapOut, Option<SoapFault>> {
            let __request = GetSchemaVersionSoapInSoapEnvelope::new(SoapGetSchemaVersionSoapIn {
                body: get_schema_version_soap_in,
                xmlns: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.aed-sicad.de/namespaces/svr/GetSchemaVersion",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetSchemaVersionSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn login(
            &self,
            login_soap_in: ports::LoginSoapIn,
        ) -> Result<ports::LoginSoapOut, Option<SoapFault>> {
            let __request = LoginSoapInSoapEnvelope::new(SoapLoginSoapIn {
                body: login_soap_in,
                xmlns: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.aed-sicad.de/namespaces/svr/Login")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: LoginSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn logout(
            &self,
            logout_soap_in: ports::LogoutSoapIn,
        ) -> Result<ports::LogoutSoapOut, Option<SoapFault>> {
            let __request = LogoutSoapInSoapEnvelope::new(SoapLogoutSoapIn {
                body: logout_soap_in,
                xmlns: Option::Some("http://www.aed-sicad.de/namespaces/svr".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.aed-sicad.de/namespaces/svr/Logout")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: LogoutSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }
}


pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    
    /// 3A Server AuftragsManagement web service.
    pub struct AuftragsManager {}

    impl AuftragsManager {
        pub fn new_client(url: &str, credentials: Option<(String, String)>) -> bindings::AuftragsManagerSoap {
            bindings::AuftragsManagerSoap::new(url, credentials)
        }
    }
}