use lazy_static::lazy_static;
use regex::Regex;
use crate::{Waehrung, SchuldenArt};

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

#[derive(Debug, Clone, PartialEq)]
pub enum RequestFailure {
	FailedToSendSoap(String),
	FailedToDeserializeResponse(String),
	Soap(SoapFault),
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
					<GetPreference xmlns=\"http://www.aed-sicad.de/namespaces/svr\">
						<sessionId>{session_id}</sessionId>
						<auftragGZip>{auftrag_gzip}</auftragGZip>
					</GetPreference>
				</soap:Body>
			</soap:Envelope>",
			session_id = session_id,
			auftrag_gzip = encode_gzip_base64(nba_xml).unwrap_or_default(),
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

#[derive(Debug, Clone, PartialEq)]
pub struct FortfuehrungsAuftrag {
	pub verfahren_name: String,
	pub verfahren_id: usize,
	pub insert: Vec<FfaInsert>,
	pub replace: Vec<FfaReplace>,
	pub delete: Vec<FfaDelete>,
}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub enum FfaInsert {
	Abteilung2 { uuid: String },
	Abteilung3 { 
		gml_oid: String, 
		gml_oid_plus_eins: String,
		kan: KennzeichnungAlterNeuerBestand, 
		verfahren_oid: String, 
		beguenstigter_rechtsinhaber_oid: String, 
		lfd_nr: usize,
		textlicher_teil: String,
		aktionsstatus: AktionsStatus,
		waehrung: Waehrung,
		betrag: usize,
		schuldenart: SchuldenArt,
	},
	BuchungsstelleBelastet { uuid: String },
}

// TODO
#[derive(Debug, Clone, PartialEq)]
pub enum FfaReplace {
	Abteilung2 { uuid: String },
	Abteilung3 { uuid: String },
	BuchungsstelleBelastet { uuid: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FfaDelete {
	Abteilung2 { uuid: String },
	Abteilung3 { uuid: String },
	BuchungsstelleBelastet { uuid: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum KennzeichnungAlterNeuerBestand {
	AlterBestand,
	NeuerBestand,
	Vorerfassung,
	MigrationAlterBestand,
}

impl KennzeichnungAlterNeuerBestand {
	pub fn code(&self) -> &'static str {
		match self {
			KennzeichnungAlterNeuerBestand::AlterBestand => "A",
			KennzeichnungAlterNeuerBestand::NeuerBestand => "N",
			KennzeichnungAlterNeuerBestand::Vorerfassung => "V",
			KennzeichnungAlterNeuerBestand::MigrationAlterBestand => "M",
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
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

impl FortfuehrungsAuftrag {
	pub fn get_xml(&self) -> String {
		use chrono::Utc;

		let delete = self.delete.iter().map(|d| {
			match d {
				FfaDelete::Abteilung2 { uuid } => {
					format!("
						<wfs:Delete typeName=\"LX_Abteilung2\">
							<ogc:Filter>
							  	<ogc:FeatureId fid=\"{uuid}\" />
							</ogc:Filter>
						</wfs:Delete>
					", uuid = uuid)
				},
				FfaDelete::Abteilung3 { uuid } => {
					format!("
						<wfs:Delete typeName=\"LX_Abteilung3\">
							<ogc:Filter>
								<ogc:FeatureId fid=\"{uuid}\" />
							</ogc:Filter>
						</wfs:Delete>
					", uuid = uuid)
				},
				FfaDelete::BuchungsstelleBelastet { uuid } => {
					format!("
						<wfs:Delete typeName=\"LX_BuchungsstelleBelastet\">
							<ogc:Filter>
							  <ogc:FeatureId fid=\"{uuid}\" />
							</ogc:Filter>
						</wfs:Delete>
					", uuid = uuid)
				}
			}
		}).collect::<Vec<_>>().join("\r\n");

		let insert = self.insert.iter().map(|i| {
			match i {
				FfaInsert::Abteilung3 { 
					gml_oid, 
					gml_oid_plus_eins,
					kan, 
					verfahren_oid, 
					beguenstigter_rechtsinhaber_oid, 
					lfd_nr,
					textlicher_teil,
					aktionsstatus,
					waehrung,
					betrag,
					schuldenart,
				} => {
					format!("
			        <lefis:LX_Abteilung3 gml:id=\"{gml_oid}\">
			          <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{gml_oid}</gml:identifier>
			          <lebenszeitintervall>
			            <AA_Lebenszeitintervall>
			              <beginnt>{datum_jetzt}</beginnt>
			            </AA_Lebenszeitintervall>
			          </lebenszeitintervall>
			          <modellart>
			            <AA_Modellart>
			              <sonstigesModell>LEFIS</sonstigesModell>
			            </AA_Modellart>
			          </modellart>
			          <lefis:kan>{kan}</lefis:kan>
			          <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_oid}\" />
			          <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
			          <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
			          <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
			          <lefis:beguenstigterRechtsinhaber xlink:href=\"urn:adv:oid:{beguenstigter_rechtsinhaber_oid}\" />
			          <lefis:beziehtSichAuf3 xlink:href=\"urn:adv:oid:{gml_oid_plus_eins}\" />
			          <lefis:lfdNr>{lfd_nr}</lefis:lfdNr>
			          <lefis:textlicherTeil>{textlicher_teil}</lefis:textlicherTeil>
			          <lefis:aktionsStatus>{aktionsstatus}</lefis:aktionsStatus>
			          <lefis:betrag>
			            <lefis:Waehrung uom=\"urn:lefis:uom:currency:{waehrung}\">{betrag}</lefis:Waehrung>
			          </lefis:betrag>
			          <lefis:rechteArt>{schuldenart}</lefis:rechteArt>
			        </lefis:LX_Abteilung3>
					", 
					gml_oid = gml_oid,
					kan = kan.code(),
					verfahren_oid = verfahren_oid,
					beguenstigter_rechtsinhaber_oid = beguenstigter_rechtsinhaber_oid,
					lfd_nr = lfd_nr,
					textlicher_teil = textlicher_teil,
					aktionsstatus = aktionsstatus.code(),
					waehrung = waehrung.code(),
					betrag = betrag,
					schuldenart = schuldenart.code(),
					datum_jetzt = Utc::now())
				},
				FfaInsert::Abteilung2 { .. } => {
					format!("") // TODO
				},
				FfaInsert::BuchungsstelleBelastet { .. } => {
					format!("") // TODO
				}
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
			  <profilkennung>LEFIS Upload</profilkennung>
			  <antragsnummer>{antragsnummer}</antragsnummer>
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
		replace = "",
		insert = if !insert.is_empty() { 
			format!("<wfs:Insert>{}</wfs:Insert>", insert) 
		} else { 
			String::new() 
		}, 
		antragsnummer = self.verfahren_name, // Wilmersdorf-Weesow_...
		auftragsnummer = format!("{:06}_0099", self.verfahren_id)) // 500108_005

		/*
	      <wfs:Insert>
	        <lefis:LX_Abteilung3 gml:id=\"DE_0000000000001\">
	          <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:DE_0000000000001</gml:identifier>
	          <lebenszeitintervall>
	            <AA_Lebenszeitintervall>
	              <beginnt>2021-12-17T09:12:25Z</beginnt>
	            </AA_Lebenszeitintervall>
	          </lebenszeitintervall>
	          <modellart>
	            <AA_Modellart>
	              <sonstigesModell>LEFIS</sonstigesModell>
	            </AA_Modellart>
	          </modellart>
	          <lefis:kan>A</lefis:kan>
	          <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:DEBBLX99vz00003m\" />
	          <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
	          <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
	          <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
	          <lefis:beguenstigterRechtsinhaber xlink:href=\"urn:adv:oid:DEBBLX991EW000az\" />
	          <lefis:beziehtSichAuf3 xlink:href=\"urn:adv:oid:DE_0000000000002\" />
	          <lefis:lfdNr>1</lefis:lfdNr>
	          <lefis:textlicherTeil>TEST ABTEILUNG DREI BITTE LOESCHEN</lefis:textlicherTeil>
	          <lefis:aktionsStatus>1000</lefis:aktionsStatus>
	          <lefis:betrag>
	            <lefis:Waehrung uom=\"urn:lefis:uom:currency:EUR\">10</lefis:Waehrung>
	          </lefis:betrag>
	          <lefis:rechteArt>7010</lefis:rechteArt>
	        </lefis:LX_Abteilung3>
	        <lefis:LX_BuchungsstelleBelastet gml:id=\"DE_0000000000002\">
	          <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:DE_0000000000002</gml:identifier>
	          <lebenszeitintervall>
	            <AA_Lebenszeitintervall>
	              <beginnt>2021-12-17T09:14:35Z</beginnt>
	            </AA_Lebenszeitintervall>
	          </lebenszeitintervall>
	          <modellart>
	            <AA_Modellart>
	              <sonstigesModell>LEFIS</sonstigesModell>
	            </AA_Modellart>
	          </modellart>
	          <lefis:kan>A</lefis:kan>
	          <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:DEBBLX99vz00003m\" />
	          <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
	          <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
	          <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
	          <lefis:belastetDasGrundstueck xlink:href=\"urn:adv:oid:DEBBLX99vw000582\" />
	          <lefis:anteil>
	            <AX_Anteil>
	              <zaehler>1</zaehler>
	              <nenner>1</nenner>
	            </AX_Anteil>
	          </lefis:anteil>
	        </lefis:LX_BuchungsstelleBelastet>
	      </wfs:Insert>
		*/

	}
}