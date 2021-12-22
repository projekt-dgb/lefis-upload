use std::path::Path;
use oracle::{Connection, StmtParam, sql_type::ToSql};
use serde_derive::{Serialize, Deserialize};
use core::future::Future;
use azul::{
    app::{App, AppConfig, LayoutSolver},
    css::Css,
    dialog::MsgBox,
    style::StyledDom,
    callbacks::{RefAny, LayoutCallbackInfo},
    window::{WindowCreateOptions, WindowFrame},
};

pub mod ui;
pub mod wsdl;

#[derive(Default)]
#[repr(C)]
struct Data { _min: u8 }

extern "C" fn render(_: &mut RefAny, _: &mut LayoutCallbackInfo) -> StyledDom {
    crate::ui::render()
    .style(Css::empty()) // styles are applied inline
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LefisUploadKonfiguration {
    pub lefis: LefisUploadKonfigurationLEFIS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LefisUploadKonfigurationLEFIS {
    pub webservice_url: String,
    #[serde(default)]
    pub benutzer: Option<String>,
    #[serde(default)]
    pub passwort: Option<String>,
}

impl Default for LefisUploadKonfigurationLEFIS {
    fn default() -> Self {
        Self {
            webservice_url: format!("http://dvzsn-ra1170/AaaDhkWebService/AuftragsManager.asmx"),
            benutzer: None,
            passwort: None,
        }
    }
}

#[derive(Debug)]
pub struct LefisInfo {
    pub version: String,
    pub schema_version: String,
    pub oracle: OracleDbParameter,
}

#[derive(Debug)]
pub struct OracleDbParameter {
    pub server: String,
    pub benutzer: Option<String>,
    pub passwort: Option<String>,
}

fn get_db_verbindung(lefis: &LefisUploadKonfigurationLEFIS) -> Result<LefisInfo, String> {
    
    use crate::wsdl::services::AuftragsManager;
    use crate::wsdl::ports::AuftragsManagerSoap;

    use crate::wsdl::messages::*;

    let bp = match (lefis.benutzer.clone(), lefis.passwort.clone()) {
        (Some(b), None) => Some((b.clone(), String::new())),
        (Some(b), Some(p)) => Some((b.clone(), p.clone())),
        _ => None,
    };

    let am = AuftragsManager::new_client(&lefis.webservice_url, bp);
    
    let version = exec_sync(async { am.get_version(GetVersionSoapIn::default()).await });
    println!("version: {:?}", version);

    let schema_version = exec_sync(async { am.get_schema_version(GetSchemaVersionSoapIn::default()).await });
    println!("schema_version: {:?}", schema_version);

    Err(format!("test"))
}

fn exec_sync<F, T>(future: F) -> T where F: Future<Output = T> {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.block_on(future)
    } else {
        tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
    }
} 

#[test]
fn test_decode_yaserde() {
    use crate::wsdl::messages::*;
    use crate::wsdl::types::*;
    use crate::wsdl::bindings::*;
    //         <?xml version="1.0" encoding="utf-8"?><soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" soap:encodingStyle="">                                                                            <soap:Body><GetVersionSoapOut xmlns:tns="http://www.aed-sicad.de/namespaces/svr" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" /></soap:Body></soap:Envelope>
    let s = r#"<?xml version="1.0" encoding="utf-8"?><soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema"><soap:Body><GetVersionResponse xmlns="http://www.aed-sicad.de/namespaces/svr"><GetVersionResult>3A-Server -LEFIS 6.4.17.3 ; 3A-Server  Zusatz-LEFIS 6.4.17.3</GetVersionResult></GetVersionResponse></soap:Body></soap:Envelope>"#;
    let expected = GetVersionSoapOut { parameters: GetVersionResponse { get_version_result: Some(format!("3A-Server -LEFIS 6.4.17.3 ; 3A-Server  Zusatz-LEFIS 6.4.17.3")) } };
    let result = yaserde::de::from_str::<GetVersionSoapOutSoapEnvelope>(&s);
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("encoded: {}", yaserde::ser::to_string(&expected).unwrap());
    assert_eq!(result.body.body, expected);
}

fn main() {

    let konfiguration_pfad = std::env::current_exe().ok()
        .and_then(|p| Some({
            p.parent()?.to_path_buf().join("Konfiguration.toml").to_str()?.to_string()
        }))
        .unwrap_or(format!("./Konfiguration.toml"));

    if !Path::new(&konfiguration_pfad).exists() {
        let _ = toml::to_string(&LefisUploadKonfiguration::default())
        .ok().and_then(|s| std::fs::write(&konfiguration_pfad, &s.as_bytes()).ok());
    }

    let konfig = match std::fs::read_to_string(&konfiguration_pfad) {
        Ok(o) => match toml::from_str(&o) {
            Ok(o) => o,
            Err(e) => {
                MsgBox::error(format!("Fehler in Konfiguration {}: {}", konfiguration_pfad, e).into());
                LefisUploadKonfiguration::default()
            }
        },
        Err(e) => {
            MsgBox::error(format!("Fehler beim Lesen von Konfiguration in {}: {}", konfiguration_pfad, e).into());
            LefisUploadKonfiguration::default()
        },
    };

    let db_verbindung = match get_db_verbindung(&konfig.lefis) {
        Ok(o) => Some(o),
        Err(e) => {
            MsgBox::error(format!("Fehler beim Verbinden zu Oracle-Datenbank: {}", e).into());
            None
        },
    };

    println!("DB Verbindung: {:#?}", db_verbindung);

    let app = App::new(RefAny::new(Data::default()), AppConfig::new(LayoutSolver::Default));
    let mut window = WindowCreateOptions::new(render);
    window.state.flags.frame = WindowFrame::Maximized;
    app.run(window);
}
