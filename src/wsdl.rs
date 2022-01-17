use std::fmt;
use serde_derive::{Serialize, Deserialize};
use lazy_static::lazy_static;
use regex::Regex;
use chrono::{DateTime, SecondsFormat, Utc};
use crate::{Waehrung, SchuldenArt, RechteArt};

#[derive(Debug, Clone)]
pub struct AuftragsManager {
	client: reqwest::Client,
	url: String,
	user: Option<String>,
	password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GetVersionResponseData {
	pub get_version_result: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LoginResponseData {
	pub session_id: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GetPreferenceResponseData {
	pub preference: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RegisterGzipResponseData {
	pub auftragsnummer: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListAuftragResponseData {
	pub status: isize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GetStateResponseData {
	pub state: isize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GetResultCountResponseData {
	pub result_count: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GetNResultGzipResponseData {
	pub erfolgreich: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GetProtocolGzipResponseData {
	pub protokoll_msg: Vec<ProtokollMsg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtokollMsg {
	Error(String),
	Info(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestFailure {
	FailedToSendSoap(String),
	FailedToDeserializeResponse(String),
	Soap(SoapFault),
}

impl fmt::Display for RequestFailure {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::RequestFailure::*;
		let e = match self {
			FailedToSendSoap(e) => e.clone(),
			FailedToDeserializeResponse(e) => e.clone(),
			Soap(sf) => format!("E{}: {}", sf.code, sf.error),
		};
		write!(f, "{}", e)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct SoapFault {
	pub code: reqwest::StatusCode,
	pub error: String,
}

lazy_static! {
    static ref GET_VERSION: Regex = Regex::new(r#"<GetVersionResponse(.*)?><GetVersionResult(.*?)>(.*)</GetVersionResult></GetVersionResponse>"#).unwrap();
    static ref LOGIN: Regex = Regex::new(r#"<LoginResponse(.*)?><LoginResult(.*?)>(.*)</LoginResult></LoginResponse>"#).unwrap();
    static ref LOGOUT: Regex = Regex::new(r#"<LogoutResponse(.*)?/>"#).unwrap();
    static ref GET_PREFERENCE: Regex = Regex::new(r#"<GetPreferenceResponse(.*)?><GetPreferenceResult(.*?)><string>(.*)</string></GetPreferenceResult></GetPreferenceResponse>"#).unwrap();
    static ref REGISTER_GZIP: Regex = Regex::new(r#"<RegisterGZipResponse(.*)?><RegisterGZipResult(.*?)>(.*)</RegisterGZipResult></RegisterGZipResponse>"#).unwrap();
    static ref LIST_AUFTRAG: Regex = Regex::new(r#"<ListAuftragResponse(.*)?>(.*)<status>(.*)</status>(.*)</ListAuftragResponse>"#).unwrap();
    static ref GET_STATE: Regex = Regex::new(r#"<GetStateResponse(.*)?><GetStateResult(.*?)>(.*)</GetStateResult></GetStateResponse>"#).unwrap();
    static ref GET_RESULT_COUNT: Regex = Regex::new(r#"<GetResultCountResponse(.*)?><GetResultCountResult(.*?)>(.*)</GetResultCountResult></GetResultCountResponse>"#).unwrap();
    static ref GET_N_RESULT_GZIP: Regex = Regex::new(r#"<GetNResultGZipResponse(.*)?><GetNResultGZipResult(.*?)>(.*)</GetNResultGZipResult></GetNResultGZipResponse>"#).unwrap();
    static ref GET_PROTOCOL_GZIP: Regex = Regex::new(r#"<GetProtocolGZipResponse(.*)?><GetProtocolGZipResult(.*?)>(.*)</GetProtocolGZipResult></GetProtocolGZipResponse>"#).unwrap();

    static ref ERFOLGREICH: Regex = Regex::new(r#"<erfolgreich(.*)?>(.*)</erfolgreich>"#).unwrap();
    static ref PROTOKOLL_MSG: Regex = Regex::new(r#"<q1:Message>((.|\s)*)</q1:Message>"#).unwrap();
    static ref PROTOKOLL_TXT: Regex = Regex::new(r#"<q1:MessageText>(.*)</q1:MessageText>"#).unwrap();
}

impl AuftragsManager {

    pub fn new(url: &str, user: Option<String>, password: Option<String>) -> Self {
        AuftragsManager {
            client: reqwest::Client::new(),
            url: url.to_string(),
            user,
            password,
        }
    }

    async fn send_soap_request(&self, body: &str, action: &str) -> Result<(reqwest::StatusCode, String), reqwest::Error> {

    	let req = body.lines().map(|s| s.trim().to_string()).collect::<Vec<_>>().join("");

        let mut req = self
            .client
            .post(&self.url)
            .body(req)
            .header("Content-Type", "text/xml")
            .header("Soapaction", action);

        if let Some(user) = &self.user {
            req = req.basic_auth(
                user.trim().clone(),
                self.password.clone(),
            );
        }

        let res = req.send().await?;
        let status = res.status();
        let txt = res.text().await.unwrap_or_default();
        
        Ok((status, txt))
    }

	pub async fn get_version(&self) 
	-> Result<GetVersionResponseData, RequestFailure> 
	{
		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<GetVersion xmlns=\"http://www.aed-sicad.de/namespaces/svr\" />
				</soap:Body>
			</soap:Envelope>"
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/GetVersion")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match GET_VERSION.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().to_string())) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetVersion".to_string()))
	    };
	
	    Ok(GetVersionResponseData {
	    	get_version_result: capture,
	    })
	}

	pub async fn login(&self) -> Result<LoginResponseData, RequestFailure> {

		let user = self.user.as_ref();
		let password = self.password.as_ref();

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
			<soap:Body>
				<Login xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
				{user}
				{password}
				</Login>
			</soap:Body>
		</soap:Envelope>",
			user = match user { Some(s) => format!("<user>{}</user>", s), None => format!("<user />") },
			password = match (user, password) { (Some(_), Some(s)) => format!("<password>{}</password>", s), _ => format!("<password />") },
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/Login")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match LOGIN.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().parse::<usize>().ok()?)) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("Login".to_string()))
	    };
	
	    Ok(LoginResponseData {
	    	session_id: capture,
	    })
	}

	pub async fn get_preference(&self, session_id: usize, preference: &str) -> Result<GetPreferenceResponseData, RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<GetPreference xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<preferenceName>{preference}</preferenceName>
					</GetPreference>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			preference = preference
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/GetPreference")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match GET_PREFERENCE.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().to_string())) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetPreference".to_string()))
	    };
	
	    Ok(GetPreferenceResponseData {
	    	preference: capture
	    })
	}

	pub async fn register_gzip(&self, session_id: usize, nba_xml: &str)  -> Result<RegisterGzipResponseData, RequestFailure> {
		
		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<RegisterGZip xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragGZip>{auftrag_gzip}</auftragGZip>
					</RegisterGZip>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrag_gzip = encode_gzip_base64(nba_xml.trim()).unwrap_or_default(),
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/RegisterGZip")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match REGISTER_GZIP.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().to_string())) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("RegisterGZip".to_string()))
	    };
	
	    Ok(RegisterGzipResponseData {
	    	auftragsnummer: capture
	    })
	}

	// ausführen, bis status = 18 (erfolgreich)
	// status = 3 = in Warteschleife
	pub async fn list_auftrag(&self, session_id: usize, auftrags_id: &str) -> Result<ListAuftragResponseData, RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<ListAuftrag xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragsId>{auftrags_id}</auftragsId>
					</ListAuftrag>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrags_id = auftrags_id,
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/ListAuftrag")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match LIST_AUFTRAG.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().parse::<isize>().ok()?)) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("ListAuftrag".to_string()))
	    };
	
	    Ok(ListAuftragResponseData {
	    	status: capture,
	    })
	}

	pub async fn get_state(&self, session_id: usize, auftrags_id: &str) -> Result<GetStateResponseData, RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<GetState xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragsId>{auftrags_id}</auftragsId>
					</GetState>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrags_id = auftrags_id,
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/GetState")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match GET_STATE.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().parse::<isize>().ok()?)) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetState".to_string()))
	    };
	
	    Ok(GetStateResponseData {
	    	state: capture,
	    })
	}

	pub async fn get_result_count(&self, session_id: usize, auftrags_id: &str) -> Result<GetResultCountResponseData, RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<GetResultCount xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragsId>{auftrags_id}</auftragsId>
					</GetResultCount>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrags_id = auftrags_id,
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/GetResultCount")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match GET_RESULT_COUNT.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().parse::<usize>().ok()?)) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetResultCount".to_string()))
	    };
	
	    Ok(GetResultCountResponseData {
	    	result_count: capture,
	    })
	}

	pub async fn get_n_result_gzip(&self, session_id: usize, auftrags_id: &str, n: usize) -> Result<GetNResultGzipResponseData, RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<GetNResultGZip xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragsId>{auftrags_id}</auftragsId>
						<rowNum>{row_num}</rowNum>
					</GetNResultGZip>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrags_id = auftrags_id,
			row_num = n,
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/GetNResultGZip")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match GET_N_RESULT_GZIP.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().to_string())) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetNResultGZip".to_string()))
	    };
	
		let base64_decoded = match base64::decode(&capture).ok() {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetNResultGZip".to_string()))
		};

		let unzipped_decoded = match decode_gzip_base64(&base64_decoded) {
			Some(s) => s,
			None => return Err(RequestFailure::FailedToDeserializeResponse("GetNResultGZip".to_string()))
		};

	    let capture = match ERFOLGREICH.captures_iter(&unzipped_decoded).nth(0).and_then(|c| Some(c.get(2)?.as_str().parse::<bool>().ok()?)) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetNResultGZip".to_string()))
	    };

	    Ok(GetNResultGzipResponseData {
	    	erfolgreich: capture,
	    })
	}

	pub async fn get_protocol_gzip(&self, session_id: usize, auftrags_id: &str) -> Result<GetProtocolGzipResponseData, RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<GetProtocolGZip xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragsId>{auftrags_id}</auftragsId>
					</GetProtocolGZip>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrags_id = auftrags_id,
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/GetProtocolGZip")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match GET_PROTOCOL_GZIP.captures_iter(&response).nth(0).and_then(|c| Some(c.get(3)?.as_str().to_string())) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetProtocolGZip".to_string()))
	    };
				
		let base64_decoded = match base64::decode(&capture).ok() {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("GetProtocolGZip".to_string()))
		};

		let unzipped_decoded = match decode_gzip_base64(&base64_decoded) {
			Some(s) => s,
			None => return Err(RequestFailure::FailedToDeserializeResponse("GetProtocolGZip".to_string()))
		};

		let protokoll_msg = unzipped_decoded.split("</q1:Message>").filter_map(|msg| {
			
			let txt = PROTOKOLL_TXT.captures_iter(msg).nth(0).and_then(|c| Some(c.get(1)?.as_str()))?;
			
			if msg.contains("<q1:MessageLevel>Error</q1:MessageLevel>") {
				Some(ProtokollMsg::Error(txt.to_string()))
			} else if msg.contains("<q1:MessageLevel>Info</q1:MessageLevel>") {
				Some(ProtokollMsg::Info(txt.to_string()))
			} else {
				None
			}

		}).collect();

	    Ok(GetProtocolGzipResponseData {
	    	protokoll_msg,
	    })
	}
	
	pub async fn logout(&self, session_id: usize) -> Result<(), RequestFailure> {

		let request = format!("
			<?xml version=\"1.0\" encoding=\"utf-8\"?>
			<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
				<soap:Body>
					<Logout xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
					</Logout>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
		);

        let (status, response) = self
            .send_soap_request(&request, "http://www.aed-sicad.de/namespaces/svr/Logout")
            .await
            .map_err(|e| RequestFailure::FailedToSendSoap(format!("{}", e)))?;

        if status != reqwest::StatusCode::OK {
        	return Err(RequestFailure::Soap(SoapFault {
        		code: status,
				error: response, 		
        	}));
        }

	    let capture = match LOGOUT.captures_iter(&response).nth(0) {
	    	Some(s) => s,
	    	None => return Err(RequestFailure::FailedToDeserializeResponse("Logout".to_string()))
	    };
	
	    Ok(())
	}
}

fn encode_gzip_base64(s: &str) -> Option<String> {
   	use flate2::{write::GzEncoder, Compression};
   	use std::io::Write;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes()).ok()?;
    Some(base64::encode(&encoder.finish().ok()?))
}

fn decode_gzip_base64(bytes: &[u8]) -> Option<String> {
   	use flate2::read::GzDecoder;
   	use std::io::Read;

   let mut gz = GzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s).ok()?;
   Some(s)
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FortfuehrungsAuftrag {
	pub verfahren_name: String,
	pub verfahren_id: usize,
	pub insert: Vec<FfaInsert>,
	pub replace: Vec<FfaReplace>,
	pub delete: Vec<FfaDelete>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxAbteilung2 {
	pub grundbuch_name: String,
	pub neue_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub kan: KennzeichnungAlterNeuerBestand,
	pub verfahren_uuid: String,
	pub rechtsinhaber: Vec<String>,
	pub buchungsstelle_uuid: String,
	pub lfd_nr: usize,
	pub textlicher_teil: String,
	pub rechteart: RechteArt,
	pub rangvermerk: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxAbteilung3 {
	pub grundbuch_name: String,
	pub neue_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub kan: KennzeichnungAlterNeuerBestand, 
	pub verfahren_uuid: String, 
	pub rechtsinhaber: Vec<String>,
	pub buchungsstelle_uuid: String,
	pub lfd_nr: usize,
	pub textlicher_teil: String,
	pub waehrung: Waehrung,
	pub betrag: String,
	pub schuldenart: SchuldenArt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxBuchungsstelleBelastetAbt2 {
	pub neue_buchungsstelle_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub kan: KennzeichnungAlterNeuerBestand,
	pub verfahren_uuid: String,
	pub grundstuecke_belastet: Vec<String>,
	pub anteil_belastet_zaehler: usize,
	pub anteil_belastet_nenner: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxBuchungsstelleBelastetAbt3 {
	pub neue_buchungsstelle_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub kan: KennzeichnungAlterNeuerBestand,
	pub verfahren_uuid: String,
	pub grundstuecke_belastet: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FfaInsert {
	Abteilung2(FfaLxAbteilung2),
	Abteilung3(FfaLxAbteilung3),
	BuchungsstelleBelastetAbt2(FfaLxBuchungsstelleBelastetAbt2),
	BuchungsstelleBelastetAbt3(FfaLxBuchungsstelleBelastetAbt3),
	NebenbeteiligterNeu(FfaLxOrdnungsNummer),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FfaDelete {
	Abteilung2 { 
		grundbuch_name: String,
		lfd_nr: usize,
		uuid: String, 
		erstellt_am: DateTime<Utc>, 
	},
	Abteilung3 { 
		grundbuch_name: String,
		lfd_nr: usize,
		uuid: String,
		erstellt_am: DateTime<Utc>, 
	},
	BuchungsstelleBelastet { 
		uuid: String,
		erstellt_am: DateTime<Utc>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum FfaReplace {
	Abteilung2 { 
		grundbuch_name: String,
		lfd_nr: usize,
		uuid: String, 
		erstellt_am: DateTime<Utc>,
		insert: FfaLxAbteilung2,
	},
	Abteilung3 { 
		grundbuch_name: String,
		lfd_nr: usize,
		uuid: String,
		erstellt_am: DateTime<Utc>,
		insert: FfaLxAbteilung3,
	},
	BuchungsstelleBelastetAbt2 { 
		uuid: String,
		erstellt_am: DateTime<Utc>,
		insert: FfaLxBuchungsstelleBelastetAbt2,
	},
	BuchungsstelleBelastetAbt3 { 
		uuid: String,
		erstellt_am: DateTime<Utc>,
		insert: FfaLxBuchungsstelleBelastetAbt3,
	},
	NebenbeteiligterReplace {
		nebenbeteiligter_stammnr: usize,

		lx_person_rolle_erstellt_am: DateTime<Utc>,
		lx_person_rolle: FfaLxPersonRolle,

		lx_person_erstellt_am: DateTime<Utc>,
		lx_person: FfaLxPerson,

		ax_person_erstellt_am: DateTime<Utc>,
		ax_person: FfaAxPerson,
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum KennzeichnungAlterNeuerBestand {
	AlterBestand,
	NeuerBestand,
	Vorerfassung,
	MigrationAlterBestand,
}

impl KennzeichnungAlterNeuerBestand {
	pub fn from_usize(s: usize) -> Option<KennzeichnungAlterNeuerBestand> {
		use self::KennzeichnungAlterNeuerBestand::*;
		match s {
			0 => Some(AlterBestand),
			1 => Some(NeuerBestand),
			2 => Some(Vorerfassung),
			3 => Some(MigrationAlterBestand),
			_ => None,
		}
	}
	pub fn code(&self) -> &'static str {
		match self {
			KennzeichnungAlterNeuerBestand::AlterBestand => "A",
			KennzeichnungAlterNeuerBestand::NeuerBestand => "N",
			KennzeichnungAlterNeuerBestand::Vorerfassung => "V",
			KennzeichnungAlterNeuerBestand::MigrationAlterBestand => "M",
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AktionsStatus {
	AltesRechtIstZuLoeschen,
	AltesRechtIstZuLoeschenUndNeuZuBegruenden,
	NeuesRecht,
	UebertragenInDenNeuenBestand,
}

impl AktionsStatus {
	pub fn code(&self) -> usize {
		match self {
			AktionsStatus::AltesRechtIstZuLoeschen => 1001,
			AktionsStatus::AltesRechtIstZuLoeschenUndNeuZuBegruenden => 1002,
			AktionsStatus::NeuesRecht => 1003,
			AktionsStatus::UebertragenInDenNeuenBestand => 1000,
		}
	}
}

impl FfaLxAbteilung2 {
	pub fn get_xml(&self) -> String {

		let FfaLxAbteilung2 {
			grundbuch_name,
			neue_uuid,
			beginnt_datum,
			kan,
			verfahren_uuid,
			rechtsinhaber,
			buchungsstelle_uuid,
			lfd_nr,
			textlicher_teil,
			rechteart,
			rangvermerk,
		} = self;

		format!("
		<lefis:LX_Abteilung2 gml:id=\"{neue_uuid}\"> <!-- {grundbuch_name} Abt. 2 Recht {lfd_nr} -->
		  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{neue_uuid}</gml:identifier>
		  <lebenszeitintervall>
		    <AA_Lebenszeitintervall>
		      <beginnt>{beginnt_datum}</beginnt>
		    </AA_Lebenszeitintervall>
		  </lebenszeitintervall>
		  <modellart>
		    <AA_Modellart>
		      <sonstigesModell>LEFIS</sonstigesModell>
		    </AA_Modellart>
		  </modellart>
		  <lefis:kan>{kan}</lefis:kan>
		  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
		  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
		  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
		  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
		  {rechtsinhaber}
		  <lefis:beziehtSichAuf2 xlink:href=\"urn:adv:oid:{buchungsstelle_uuid}\" />
		  <lefis:lfdNr>{lfd_nr}</lefis:lfdNr>
		  <lefis:textlicherTeil>{textlicher_teil}</lefis:textlicherTeil>
		  <lefis:rechteArt>{rechteart}</lefis:rechteArt>
		  <lefis:aktionsStatus>{aktionsstatus}</lefis:aktionsStatus>
		  <lefis:rangvermerk>{rangvermerk}</lefis:rangvermerk>
		</lefis:LX_Abteilung2>
		",
		grundbuch_name = grundbuch_name,
	    neue_uuid = neue_uuid,
	    beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
	    kan = kan.code(),
	    verfahren_uuid = verfahren_uuid,
		rechtsinhaber = rechtsinhaber.iter().map(|uuid| {
			format!("<lefis:istRechtsinhaber xlink:href=\"urn:adv:oid:{}\" />", uuid)
		}).collect::<Vec<_>>().join("\r\n"),
		buchungsstelle_uuid = buchungsstelle_uuid,
		lfd_nr = lfd_nr,
		textlicher_teil = xml_clean_text(textlicher_teil.trim()),
		rechteart = rechteart.code(),
		aktionsstatus = rechteart.get_aktionsstatus().code(),
		rangvermerk = rangvermerk.trim())
	}
}

impl FfaLxAbteilung3 {
	pub fn get_xml(&self) -> String {

		let FfaLxAbteilung3 {
			grundbuch_name,
			neue_uuid,
			beginnt_datum,
			kan, 
			verfahren_uuid, 
			rechtsinhaber,
			buchungsstelle_uuid,
			lfd_nr,
			textlicher_teil,
			waehrung,
			betrag,
			schuldenart,
		} = self;

		format!("
	        <lefis:LX_Abteilung3 gml:id=\"{neue_uuid}\"> <!-- {grundbuch_name} Abt. 3 Recht {lfd_nr} -->
	          <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{neue_uuid}</gml:identifier>
	          <lebenszeitintervall>
	            <AA_Lebenszeitintervall>
	              <beginnt>{beginnt_datum}</beginnt>
	            </AA_Lebenszeitintervall>
	          </lebenszeitintervall>
	          <modellart>
	            <AA_Modellart>
	              <sonstigesModell>LEFIS</sonstigesModell>
	            </AA_Modellart>
	          </modellart>
	          <lefis:kan>{kan}</lefis:kan>
	          <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
	          <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
	          <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
	          <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
	          {rechtsinhaber}
	          <lefis:beziehtSichAuf3 xlink:href=\"urn:adv:oid:{buchungsstelle_uuid}\" />
	          <lefis:lfdNr>{lfd_nr}</lefis:lfdNr>
	          <lefis:textlicherTeil>{textlicher_teil}</lefis:textlicherTeil>
	          <lefis:aktionsStatus>{aktionsstatus}</lefis:aktionsStatus>
	          <lefis:betrag>
	            <lefis:Waehrung uom=\"urn:lefis:uom:currency:{waehrung}\">{betrag}</lefis:Waehrung>
	          </lefis:betrag>
	          <lefis:rechteArt>{schuldenart}</lefis:rechteArt>
	        </lefis:LX_Abteilung3>
			",
			grundbuch_name = grundbuch_name,
			neue_uuid = neue_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			kan = kan.code(),
			verfahren_uuid = verfahren_uuid,
			rechtsinhaber = rechtsinhaber.iter().map(|uuid| {
				format!("<lefis:beguenstigterRechtsinhaber xlink:href=\"urn:adv:oid:{}\" />", uuid)
			}).collect::<Vec<_>>().join("\r\n"),
			buchungsstelle_uuid = buchungsstelle_uuid,
			lfd_nr = lfd_nr,
			textlicher_teil = xml_clean_text(textlicher_teil.trim()),
			aktionsstatus = schuldenart.get_aktionsstatus().code(),
			waehrung = waehrung.code(),
			betrag = betrag,
			schuldenart = schuldenart.code())
	}
}

impl FfaLxBuchungsstelleBelastetAbt2 {
	pub fn get_xml(&self) -> String {

		let FfaLxBuchungsstelleBelastetAbt2 {
			neue_buchungsstelle_uuid,
			beginnt_datum,
			kan,
			verfahren_uuid,
			grundstuecke_belastet,
			anteil_belastet_zaehler,
			anteil_belastet_nenner,
		} = self;

		format!("
			<lefis:LX_BuchungsstelleBelastet gml:id=\"{neue_buchungsstelle_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{neue_buchungsstelle_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>{kan}</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  {grundstuecke_belastet}
			  <lefis:anteil>
			    <AX_Anteil>
			      <zaehler>{anteil_belastet_zaehler}</zaehler>
			      <nenner>{anteil_belastet_nenner}</nenner>
			    </AX_Anteil>
			  </lefis:anteil>
			</lefis:LX_BuchungsstelleBelastet>
		",
			neue_buchungsstelle_uuid = neue_buchungsstelle_uuid,
	    	beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
	    	kan = kan.code(),
	    	verfahren_uuid = verfahren_uuid,
	    	grundstuecke_belastet = grundstuecke_belastet.iter().map(|uuid| {
	    		format!("<lefis:belastetDasGrundstueck xlink:href=\"urn:adv:oid:{}\" />", uuid)
	    	}).collect::<Vec<_>>().join("\r\n"),
	    	anteil_belastet_zaehler = anteil_belastet_zaehler,
	    	anteil_belastet_nenner = anteil_belastet_nenner.clone().max(1),
		)	
	}
}

impl FfaLxBuchungsstelleBelastetAbt3 {
	pub fn get_xml(&self) -> String {

		let FfaLxBuchungsstelleBelastetAbt3 {
			neue_buchungsstelle_uuid,
			beginnt_datum,
			kan,
			verfahren_uuid,
			grundstuecke_belastet,
		} = self;

		format!("
			<lefis:LX_BuchungsstelleBelastet gml:id=\"{neue_buchungsstelle_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{neue_buchungsstelle_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>{kan}</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  {grundstuecke_belastet}
			</lefis:LX_BuchungsstelleBelastet>
		",
			neue_buchungsstelle_uuid = neue_buchungsstelle_uuid,
	    	beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
	    	kan = kan.code(),
	    	verfahren_uuid = verfahren_uuid,
	    	grundstuecke_belastet = grundstuecke_belastet.iter().map(|uuid| {
	    		format!("<lefis:belastetDasGrundstueck xlink:href=\"urn:adv:oid:{}\" />", uuid)
	    	}).collect::<Vec<_>>().join("\r\n"),
		)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxPersonRolle {
	pub personenrolle_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub kan: KennzeichnungAlterNeuerBestand,
	pub verfahren_uuid: String,
}

impl FfaLxPersonRolle {
	pub fn get_xml(&self) -> String {
		let FfaLxPersonRolle {
			personenrolle_uuid,
			beginnt_datum,
			kan,
			verfahren_uuid,
		} = self;

		format!("
			<lefis:LX_PersonRolle gml:id=\"{personenrolle_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{personenrolle_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>{kan}</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  <lefis:artDerEntstehung>1003</lefis:artDerEntstehung>
			  <lefis:bearbeitungsStatus>true</lefis:bearbeitungsStatus>
			  <lefis:legitimationsgrundlage>1060</lefis:legitimationsgrundlage>
			  <lefis:laden>true</lefis:laden>
			  <lefis:zahlungspflichtigNach19>false</lefis:zahlungspflichtigNach19>
			  <lefis:zahlungspflichtigFestsetzungen>false</lefis:zahlungspflichtigFestsetzungen>
			</lefis:LX_PersonRolle>
		",
			personenrolle_uuid = personenrolle_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			kan = kan.code(),
			verfahren_uuid = verfahren_uuid,
		)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxPerson {
	pub lx_person_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub verfahren_uuid: String,
	pub personenrolle_uuid: String,
	pub ax_person_uuid: String,
}

impl FfaLxPerson {
	pub fn get_xml(&self) -> String {
		let FfaLxPerson {
			lx_person_uuid,
			beginnt_datum,
			verfahren_uuid,
			personenrolle_uuid,
			ax_person_uuid,
		} = self;

		format!("
			<lefis:LX_Person gml:id=\"{lx_person_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{lx_person_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>A</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  <lefis:hatEine xlink:href=\"urn:adv:oid:{personenrolle_uuid}\" />
			  <lefis:ergaenzt xlink:href=\"urn:adv:oid:{ax_person_uuid}\" />
			</lefis:LX_Person>
		",
			lx_person_uuid = lx_person_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			verfahren_uuid = verfahren_uuid,
			personenrolle_uuid = personenrolle_uuid,
			ax_person_uuid = ax_person_uuid,
		)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaAxPerson {
	pub ax_person_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub nachname_oder_firma: String,
	pub anrede: Option<Anrede>,
	pub vorname: String,
	pub titel: String,
	pub geburtsname: String,
	pub geburtsdatum: Option<DateTime<Utc>>,
	pub wohnort: String,
}

impl FfaAxPerson {
	pub fn get_xml(&self) -> String {
		let FfaAxPerson {
			ax_person_uuid,
			beginnt_datum,
			nachname_oder_firma,
			anrede,
			vorname,
			titel,
			geburtsname,
			geburtsdatum,
			wohnort,
		} = self;

		format!("
			<AX_Person gml:id=\"{ax_person_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{ax_person_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <advStandardModell>DLKM</advStandardModell>
			    </AA_Modellart>
			  </modellart>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <nachnameOderFirma>{nachname_oder_firma}</nachnameOderFirma>
			  {anrede}
			  <vorname>{vorname}</vorname>
			  <namensbestandteil>{titel}</namensbestandteil>
			  <geburtsname>{geburtsname}</geburtsname>
			  <wohnortOderSitz>{wohnort}</wohnortOderSitz>
			  <qualitaetsangaben>
			    <AX_DQOhneDatenerhebung>
			      <herkunft>
			        <gmd:LI_Lineage>
			          <gmd:processStep>
			            <gmd:LI_ProcessStep>
			              <gmd:description>
			                <AX_LI_ProcessStep_MitDatenerhebung_Description>Erhebung</AX_LI_ProcessStep_MitDatenerhebung_Description>
			              </gmd:description>
			              <gmd:dateTime>
			                <gco:DateTime>{beginnt_datum}</gco:DateTime>
			              </gmd:dateTime>
			              <gmd:processor>
			                <gmd:CI_ResponsibleParty>
			                  <gmd:organisationName>
			                    <gco:CharacterString>Flurbereinigungsbehörde</gco:CharacterString>
			                  </gmd:organisationName>
			                  <gmd:role>
			                    <gmd:CI_RoleCode codeList=\"http://www.isotc211.org/2005/resources/CodeList/gmxCodelists.xml#CI_RoleCode\" codeListValue=\"http://www.isotc211.org/2005/resources/CodeList/gmxCodelists.xml#CI_RoleCode_processor\">processor</gmd:CI_RoleCode>
			                  </gmd:role>
			                </gmd:CI_ResponsibleParty>
			              </gmd:processor>
			            </gmd:LI_ProcessStep>
			          </gmd:processStep>
			        </gmd:LI_Lineage>
			      </herkunft>
			    </AX_DQOhneDatenerhebung>
			  </qualitaetsangaben>
			  <zeigtAuf xlink:href=\"urn:adv:oid:{ax_person_uuid}\" />
			</AX_Person>
		",
			ax_person_uuid = ax_person_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			nachname_oder_firma = xml_clean_text(nachname_oder_firma),
			anrede = anrede.map(|s| format!("<anrede>{}</anrede>", s.code())).unwrap_or_default(),
			vorname = xml_clean_text(vorname),
			titel = xml_clean_text(titel),
			geburtsname = xml_clean_text(geburtsname),
			wohnort = xml_clean_text(wohnort),
		)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct FfaLxOrdnungsNummer {
	pub ordnungsnummer_bodenordnung_uuid: String,
	pub beginnt_datum: DateTime<Utc>,
	pub kan: KennzeichnungAlterNeuerBestand,
	pub verfahren_uuid: String,
	pub stammnummer: usize,
	pub unternummer: usize,
	pub personenrolle_uuid: String,
	pub lx_person_uuid: String,
	pub ax_person_uuid: String,
	pub nachname_oder_firma: String,
	pub anrede: Option<Anrede>,
	pub vorname: String,
	pub titel: String,
	pub geburtsname: String,
	pub geburtsdatum: Option<DateTime<Utc>>,
	pub wohnort: String,
	pub buchungsblatt_uuid: String,
	pub bb_land: String,
	pub bb_bezirk: String,
	pub bbn_mit_erweiterung: String,
	pub buchungsblatt_bodenordnung_uuid: String,
	pub ax_namensnummer_uuid: String,
	pub lx_namensnummer_uuid: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Anrede {
	Herr,
	Frau,
	Firma,
}

impl Anrede {
	pub fn from_usize(u: usize) -> Option<Self> {
		match u {
			2000 => Some(Anrede::Herr),
			1000 => Some(Anrede::Frau),
			3000 => Some(Anrede::Firma),
			_ => None,
		}
	}

	pub fn code(&self) -> usize {
		use self::Anrede::*;
		match self {
			Herr => 2000,
			Frau => 1000,
			Firma => 3000,
		}
	}
}

impl FfaLxOrdnungsNummer {
	pub fn get_xml(&self) -> String {

		let FfaLxOrdnungsNummer {
			ordnungsnummer_bodenordnung_uuid,
			beginnt_datum,
			kan,
			verfahren_uuid,
			stammnummer,
			unternummer,
			personenrolle_uuid,
			lx_person_uuid,
			ax_person_uuid,
			nachname_oder_firma,
			anrede,
			vorname,
			titel,
			geburtsname,
			geburtsdatum,
			wohnort,
			buchungsblatt_uuid,
			bb_land,
			bb_bezirk,
			bbn_mit_erweiterung,
			buchungsblatt_bodenordnung_uuid,
			ax_namensnummer_uuid,
			lx_namensnummer_uuid,
		} = self;

		let lx_ordnungsnummer_bodenordnung = format!("
			<lefis:LX_OrdnungsnummerBodenordnung gml:id=\"{ordnungsnummer_bodenordnung_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{ordnungsnummer_bodenordnung_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>{kan}</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  <lefis:hatGrundbucheigentum xlink:href=\"urn:adv:oid:{buchungsblatt_uuid}\" />
			  <lefis:anspruchsWert>0</lefis:anspruchsWert>
			  <lefis:einlageWert>0</lefis:einlageWert>
			  <lefis:ordnungsnummer>
			    <lefis:LX_Ordnungsnummer>
			      <lefis:stammnummer>{stammnummer}</lefis:stammnummer>
			      <lefis:unternummer>{unternummer}</lefis:unternummer>
			      <lefis:rechtsverhaeltnis>0</lefis:rechtsverhaeltnis>
			    </lefis:LX_Ordnungsnummer>
			  </lefis:ordnungsnummer>
			  <lefis:rechtsbehelf>9999</lefis:rechtsbehelf>
			  <lefis:ansprechpartner>(Nachname, Vorname)</lefis:ansprechpartner>
			  <lefis:funktion>9999</lefis:funktion>
			</lefis:LX_OrdnungsnummerBodenordnung>
		",
			ordnungsnummer_bodenordnung_uuid = ordnungsnummer_bodenordnung_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			kan = kan.code(),
			verfahren_uuid = verfahren_uuid,
			buchungsblatt_uuid = buchungsblatt_uuid,
			stammnummer = stammnummer,
			unternummer = unternummer,
		);

		let personenrolle = FfaLxPersonRolle {
			personenrolle_uuid: personenrolle_uuid.clone(),
			beginnt_datum: beginnt_datum.clone(),
			kan: kan.clone(),
			verfahren_uuid: verfahren_uuid.clone(),
		}.get_xml();

		let lx_person = FfaLxPerson {
			lx_person_uuid: lx_person_uuid.clone(),
			beginnt_datum: beginnt_datum.clone(),
			verfahren_uuid: verfahren_uuid.clone(),
			personenrolle_uuid: personenrolle_uuid.clone(),
			ax_person_uuid: ax_person_uuid.clone(),
		}.get_xml();

		let ax_person = FfaAxPerson {
			ax_person_uuid: ax_person_uuid.clone(),
			beginnt_datum: beginnt_datum.clone(),
			nachname_oder_firma: nachname_oder_firma.clone(),
			anrede: anrede.clone(),
			vorname: vorname.clone(),
			titel: titel.clone(),
			geburtsdatum: geburtsdatum.clone(),
			geburtsname: geburtsname.clone(),
			wohnort: wohnort.clone(),
		}.get_xml();

		let buchungsblatt = format!("
			<AX_Buchungsblatt gml:id=\"{buchungsblatt_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{buchungsblatt_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <advStandardModell>DLKM</advStandardModell>
			    </AA_Modellart>
			  </modellart>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <buchungsblattkennzeichen>{bb_land}{bb_bezirk}{bbn_mit_erweiterung}</buchungsblattkennzeichen>
			  <buchungsblattbezirk>
			    <AX_Buchungsblattbezirk_Schluessel>
			      <land>{bb_land}</land>
			      <bezirk>{bb_bezirk}</bezirk>
			    </AX_Buchungsblattbezirk_Schluessel>
			  </buchungsblattbezirk>
			  <buchungsblattnummerMitBuchstabenerweiterung>{bbn_mit_erweiterung}</buchungsblattnummerMitBuchstabenerweiterung>
			  <blattart>5000</blattart>
			</AX_Buchungsblatt>
		",
			buchungsblatt_uuid = buchungsblatt_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			bb_land = bb_land,
			bb_bezirk = bb_bezirk,
			bbn_mit_erweiterung = bbn_mit_erweiterung,
		);

		let lx_buchungsblatt_bodenordnung = format!("
			<lefis:LX_BuchungsblattBodenordnung gml:id=\"{buchungsblatt_bodenordnung_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{buchungsblatt_bodenordnung_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>A</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  <lefis:ergaenzt xlink:href=\"urn:adv:oid:{buchungsblatt_uuid}\" />
			  <lefis:nebenbeteiligtenBlatt>true</lefis:nebenbeteiligtenBlatt>
			  <lefis:grundbuchvergleichDurchgefuehrt>false</lefis:grundbuchvergleichDurchgefuehrt>
			  <lefis:vollmigriertesGrundbuchblatt>false</lefis:vollmigriertesGrundbuchblatt>
			</lefis:LX_BuchungsblattBodenordnung>
		",
			buchungsblatt_bodenordnung_uuid = buchungsblatt_bodenordnung_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			verfahren_uuid = verfahren_uuid,
			buchungsblatt_uuid = buchungsblatt_uuid,
		);

		let ax_namensnummer = format!("
			<AX_Namensnummer gml:id=\"{ax_namensnummer_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{ax_namensnummer_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <advStandardModell>DLKM</advStandardModell>
			    </AA_Modellart>
			  </modellart>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <laufendeNummerNachDIN1421>0001.00.00.00.00</laufendeNummerNachDIN1421>
			  <istBestandteilVon xlink:href=\"urn:adv:oid:{buchungsblatt_uuid}\" />
			  <benennt xlink:href=\"urn:adv:oid:{ax_person_uuid}\" />
			</AX_Namensnummer>
		",
			ax_namensnummer_uuid = ax_namensnummer_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			buchungsblatt_uuid = buchungsblatt_uuid,
			ax_person_uuid = ax_person_uuid,
		);

		let lx_namensnummer = format!("
			<lefis:LX_Namensnummer gml:id=\"{lx_namensnummer_uuid}\">
			  <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{lx_namensnummer_uuid}</gml:identifier>
			  <lebenszeitintervall>
			    <AA_Lebenszeitintervall>
			      <beginnt>{beginnt_datum}</beginnt>
			    </AA_Lebenszeitintervall>
			  </lebenszeitintervall>
			  <modellart>
			    <AA_Modellart>
			      <sonstigesModell>LEFIS</sonstigesModell>
			    </AA_Modellart>
			  </modellart>
			  <lefis:kan>{kan}</lefis:kan>
			  <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
			  <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			  <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			  <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			  <lefis:hat xlink:href=\"urn:adv:oid:{personenrolle_uuid}\" />
			  <lefis:ergaenzt xlink:href=\"urn:adv:oid:{ax_namensnummer_uuid}\" />
			</lefis:LX_Namensnummer>
		",
			lx_namensnummer_uuid = lx_namensnummer_uuid,
			beginnt_datum = beginnt_datum.to_rfc3339_opts(SecondsFormat::Secs, true),
			kan = kan.code(),
			verfahren_uuid = verfahren_uuid,
			personenrolle_uuid = personenrolle_uuid,
			ax_namensnummer_uuid = ax_namensnummer_uuid,
		);

		format!("
			{lx_ordnungsnummer_bodenordnung}
			{personenrolle}
			{lx_person}
			{ax_person}
			{buchungsblatt}
			{lx_buchungsblatt_bodenordnung}
			{ax_namensnummer}
			{lx_namensnummer}
		", 
			lx_ordnungsnummer_bodenordnung = lx_ordnungsnummer_bodenordnung,
			personenrolle = personenrolle,
			lx_person = lx_person,
			ax_person = ax_person,
			buchungsblatt = buchungsblatt,
			lx_buchungsblatt_bodenordnung = lx_buchungsblatt_bodenordnung,
			ax_namensnummer = ax_namensnummer,
			lx_namensnummer = lx_namensnummer,
		)
	}
}

impl FortfuehrungsAuftrag {
	pub fn get_xml(&self) -> String {

		let delete = self.delete.iter().map(|d| {
			match d {
				FfaDelete::Abteilung2 { uuid, erstellt_am, grundbuch_name, lfd_nr } => {
					format!("
						<wfs:Delete typeName=\"LX_Abteilung2\"> <!-- {grundbuch_name} Abt. 2 Recht {lfd_nr} -->
							<ogc:Filter>
							  	<ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
							</ogc:Filter>
						</wfs:Delete>
					", 
						uuid = uuid, 
						erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"),
						grundbuch_name = grundbuch_name,
						lfd_nr = lfd_nr,
					)
				},
				FfaDelete::Abteilung3 { uuid, erstellt_am, grundbuch_name, lfd_nr } => {
				format!("
					<wfs:Delete typeName=\"LX_Abteilung3\"> <!-- {grundbuch_name} Abt. 3 Recht {lfd_nr} -->
						<ogc:Filter>
							<ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
						</ogc:Filter>
					</wfs:Delete>", 
					uuid = uuid, 
					erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"), 
					grundbuch_name = grundbuch_name, 
					lfd_nr = lfd_nr, 
					)
				},
				FfaDelete::BuchungsstelleBelastet { uuid, erstellt_am } => {
					format!("
					<wfs:Delete typeName=\"LX_BuchungsstelleBelastet\">
						<ogc:Filter>
						  <ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
						</ogc:Filter>
					</wfs:Delete>", uuid = uuid, erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"))
				}
			}
		}).collect::<Vec<_>>().join("\r\n");

		let replace = self.replace.iter().map(|i| {
			match i {
				FfaReplace::Abteilung2 { grundbuch_name, lfd_nr, uuid, erstellt_am, insert } => {
					format!("
					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						<!-- Ändere {grundbuch_name} Abt. 2 lfd. Nr. {lfd_nr} -->
						{insert}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>
					", 
						grundbuch_name = grundbuch_name,
						lfd_nr = lfd_nr,
						insert = insert.get_xml(),
						uuid = uuid,
						erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"),
					)
				},
				FfaReplace::Abteilung3 {  grundbuch_name, lfd_nr, uuid, erstellt_am, insert } => {
					format!("
					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						<!-- Ändere {grundbuch_name} Abt. 3 lfd. Nr. {lfd_nr} -->
						{insert}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>
					", 	
						grundbuch_name = grundbuch_name,
						lfd_nr = lfd_nr,
						insert = insert.get_xml(),
						uuid = uuid,
						erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"),
					)
				},
				FfaReplace::BuchungsstelleBelastetAbt2 { uuid, erstellt_am, insert } => {
					format!("
					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						{insert}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>
					", insert = insert.get_xml(),
						uuid = uuid,
						erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"),
					)
				},
				FfaReplace::BuchungsstelleBelastetAbt3 { uuid, erstellt_am, insert } => {
					format!("
					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						{insert}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{uuid}{erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>
					", insert = insert.get_xml(),
						uuid = uuid,
						erstellt_am = erstellt_am.format("%Y%m%dT%H%M%SZ"),
					)
				},
				FfaReplace::NebenbeteiligterReplace {
					nebenbeteiligter_stammnr,
					lx_person_rolle_erstellt_am,
					lx_person_rolle,
					lx_person_erstellt_am,
					lx_person,
					ax_person_erstellt_am,
					ax_person,
				} => {
					format!("
					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						<!-- Ordnungsnummer {nebenbeteiligter_stammnr}/00 -->
						{lx_person_rolle_xml}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{lx_person_rolle_uuid}{lx_person_rolle_erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>

					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						{lx_person_xml}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{lx_person_uuid}{lx_person_erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>

					<wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
						{ax_person_xml}
						<ogc:Filter>
						  	<ogc:FeatureId fid=\"{ax_person_uuid}{ax_person_erstellt_am}\" />
						</ogc:Filter>
					</wfsext:Replace>
					",
						nebenbeteiligter_stammnr = nebenbeteiligter_stammnr,
						lx_person_rolle_xml = lx_person_rolle.get_xml(),
						lx_person_rolle_uuid = lx_person_rolle.personenrolle_uuid,
						lx_person_rolle_erstellt_am = lx_person_rolle_erstellt_am.format("%Y%m%dT%H%M%SZ"),
						lx_person_xml = lx_person.get_xml(),
						lx_person_uuid =lx_person.lx_person_uuid,
						lx_person_erstellt_am = lx_person_erstellt_am.format("%Y%m%dT%H%M%SZ"),
						ax_person_xml = ax_person.get_xml(),
						ax_person_uuid = ax_person.ax_person_uuid,
						ax_person_erstellt_am = ax_person_erstellt_am.format("%Y%m%dT%H%M%SZ"),
					)
				}
			}
		})
		.collect::<Vec<_>>()
		.join("\r\n");

		let insert = self.insert.iter().map(|i| {
			match i {
				FfaInsert::Abteilung2(a) => a.get_xml(),
				FfaInsert::Abteilung3(a) => a.get_xml(),
				FfaInsert::BuchungsstelleBelastetAbt2(a) => a.get_xml(),
				FfaInsert::BuchungsstelleBelastetAbt3(a) => a.get_xml(),
				FfaInsert::NebenbeteiligterNeu(a) => a.get_xml(),
			}
		}).collect::<Vec<_>>().join("\r\n");

		format!("
		<?xml version=\"1.0\" encoding=\"utf-8\"?>
		<!--Die NAS-Datei wurde mit der FI-Version 6.4.3.19200 erstellt.-->
		<AX_Fortfuehrungsauftrag xsi:schemaLocation=\"http://www.landentwicklung.de/namespaces/lefis/1.5 NAS-LEFIS-Operationen.xsd http://www.adv-online.de/namespaces/adv/gid/6.0 aaa.xsd\" xmlns=\"http://www.adv-online.de/namespaces/adv/gid/6.0\" xmlns:gsr=\"http://www.isotc211.org/2005/gsr\" xmlns:fc=\"http://www.adv-online.de/namespaces/adv/gid/fc/6.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:gml=\"http://www.opengis.net/gml/3.2\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xmlns:lefis=\"http://www.landentwicklung.de/namespaces/lefis/1.5\" xmlns:wfsext=\"http://www.adv-online.de/namespaces/adv/gid/wfsext\" xmlns:gco=\"http://www.isotc211.org/2005/gco\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xmlns:gts=\"http://www.isotc211.org/2005/gts\" xmlns:ogc=\"http://www.adv-online.de/namespaces/adv/gid/ogc\" xmlns:wfs=\"http://www.adv-online.de/namespaces/adv/gid/wfs\" xmlns:gmd=\"http://www.isotc211.org/2005/gmd\" xmlns:gss=\"http://www.isotc211.org/2005/gss\">
			  <empfaenger>
			    <AA_Empfaenger>
			      <direkt>true</direkt>
			    </AA_Empfaenger>
			  </empfaenger>
			  <ausgabeform>application/gzip</ausgabeform>
			  <koordinatenangaben>
			    <AA_Koordinatenreferenzsystemangaben>
			      <crs xlink:href=\"urn:adv:crs:ETRS89_UTM33\" />
			      <anzahlDerNachkommastellen>3</anzahlDerNachkommastellen>
			      <standard>true</standard>
			    </AA_Koordinatenreferenzsystemangaben>
			  </koordinatenangaben>
			  <geaenderteObjekte>
			    <wfs:Transaction version=\"1.0.0\" service=\"WFS\">
			      {delete}
			      {replace}
			      {insert}
			    </wfs:Transaction>
			  </geaenderteObjekte>
			  <profilkennung>AED Sicad</profilkennung>
			  <antragsnummer>LefisUpload-{auftragsnummer}-{antragsnummer}</antragsnummer>
			  <auftragsnummer>{auftragsnummer}</auftragsnummer>
			  <impliziteLoeschungDerReservierung>4000</impliziteLoeschungDerReservierung>
			  <verarbeitungsart>1000</verarbeitungsart>
			  <geometriebehandlung>false</geometriebehandlung>
			  <mitTemporaeremArbeitsbereich>false</mitTemporaeremArbeitsbereich>
			  <mitObjektenImFortfuehrungsgebiet>false</mitObjektenImFortfuehrungsgebiet>
			  <mitFortfuehrungsnachweis>false</mitFortfuehrungsnachweis>
		  </AX_Fortfuehrungsauftrag>
		", 
		delete = delete, 
		replace = replace,
		insert = if !insert.is_empty() { 
			format!("<wfs:Insert>{}</wfs:Insert>", insert) 
		} else { 
			String::new() 
		}, 
		antragsnummer = self.verfahren_name, // Wilmersdorf-Weesow_...
		auftragsnummer = format!("{}_0099", self.verfahren_id)) // 500108_005
	}
}

fn xml_clean_text(s: &str) -> String {
	s.chars().map(|c| match xml_replace_special_char(c) {
		Some(s) => s.to_string(),
		None => format!("{}", c),
	}).collect::<Vec<_>>().join("")
}

// https://docs.oracle.com/cd/A97335_02/apps.102/bc4j/developing_bc_projects/obcCustomXml.htm
fn xml_replace_special_char(c: char) -> Option<&'static str> {
	match c {
		'<' => Some("&lt;"),
		'>' => Some("&rt;"),
		'&' => Some("&#38;"),
		'\'' => Some("&#39;"),
		'"' => Some("&#34;"),
		_ => None,
	}
}