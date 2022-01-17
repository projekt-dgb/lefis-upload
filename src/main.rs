#![windows_subsystem = "windows"]

use std::path::Path;
use oracle::StmtParam;
use serde_derive::{Serialize, Deserialize};
use core::future::Future;
use lazy_static::lazy_static;
use regex::Regex;
use azul::{
    app::{App, AppConfig, LayoutSolver},
    css::Css,
    dialog::MsgBox,
    style::StyledDom,
    callbacks::{RefAny, LayoutCallbackInfo},
    window::{WindowCreateOptions, WindowFrame},
};
use crate::wsdl::{
    Anrede, ProtokollMsg, 
    RequestFailure, KennzeichnungAlterNeuerBestand
};
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

pub mod ui;
pub mod wsdl;

#[derive(Default)]
pub struct AppData {
    verfahren_filter: Option<String>,
    konfiguration: LefisUploadKonfiguration,
    lefis_info: Option<LefisInfo>,
    dhk_verbindung: Option<DhkVerbindung>,
    geladene_verfahren: GeladeneVerfahren,
    ausgewaehltes_verfahren: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct GeladeneVerfahren {
    pub verfahren: Vec<VerfahrenGeladen>,
    pub flurstuecke_ohne_verfahren: Vec<AxFlurstueckOhneVerfahren>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LefisUploadKonfiguration {
    pub lefis: LefisUploadKonfigurationLEFIS,
}
impl Default for LefisUploadKonfiguration {
    fn default() -> Self {
        Self {
            lefis: LefisUploadKonfigurationLEFIS::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LefisUploadKonfigurationLEFIS {
    #[serde(default)]
    pub https: bool,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: usize,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub benutzer: Option<String>,
    #[serde(default)]
    pub passwort: Option<String>,
}

impl Default for LefisUploadKonfigurationLEFIS {
    fn default() -> Self {
        Self {
            https: false,
            host: format!("dvzsn-ra1170"),
            port: 80,
            service: format!("AaaDhkWebService/AuftragsManager.asmx"),
            benutzer: None,
            passwort: None,
        }
    }
}

impl LefisUploadKonfigurationLEFIS {
    pub fn get_webservice_url(&self) -> String {
        format!("{protocol}://{host}:{port}/{service}",
            protocol = if self.https {
                "https"
            } else {
                "http"
            },
            host = self.host.trim(),
            port = self.port,
            service = self.service.trim(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LefisInfo {
    pub version: String,
    pub oracle: OracleDbParameter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OracleDbParameter {
    pub server: String,
    pub benutzer: Option<String>,
    pub passwort: Option<String>,
}

lazy_static! {
    // pcserver,sde:oracle11g:oracleInstanzName:eigentuemer_repository,eigentuemer_repository,passwort,eigentuemer_repository.DEFAULT
    static ref DB: Regex = Regex::new(r#"(.*),(.*):(.*):(.*):(.*),(.*),(.*)@(.*),(.*).(.*)"#).unwrap();
    static ref DB_2: Regex = Regex::new(r#"(.*),(.*):(.*):(.*):(.*),(.*),(.*),(.*).(.*)"#).unwrap();
}

impl OracleDbParameter {
    pub fn new(s: &str) -> Option<Self> {
        if s.contains("@") {
            DB.captures_iter(s).nth(0).and_then(|cap| {
                
                let host = cap.get(1)?.as_str().to_string();
                let dienst = cap.get(8)?.as_str().to_string();
                let benutzer = cap.get(6)?.as_str().to_string();
                let passwort = cap.get(7)?.as_str().to_string();
                
                Some(OracleDbParameter {
                    server: format!("{}/{}", host, dienst),
                    benutzer: if benutzer.is_empty() { None } else { Some(benutzer) },
                    passwort: if passwort.is_empty() { None } else { Some(passwort) },
                })
            })
        } else {
            DB_2.captures_iter(s).nth(0).and_then(|cap| {
                
                let host = cap.get(4)?.as_str().to_string();
                let benutzer = cap.get(6)?.as_str().to_string();
                let passwort = cap.get(7)?.as_str().to_string();
                
                Some(OracleDbParameter {
                    server: host,
                    benutzer: if benutzer.is_empty() { None } else { Some(benutzer) },
                    passwort: if passwort.is_empty() { None } else { Some(passwort) },
                })
            })
        }
    }
}

#[derive(Debug)]
pub struct DhkVerbindung {
    pub conn: oracle::Connection,
    pub zugangsdaten: OracleDbParameter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerfahrenGeladen {
    pub ui: UiState,
    pub nummer: usize,
    pub name: String,
    pub dhk_verbindung: String,
    pub uuid: String,

    pub flurstuecke: Vec<AxFlurstueck>,
    pub ordnungsnummern: Vec<OrdnungsnummerBodenordnung>,
    pub personen_rollen: Vec<LxPersonRolle>,
    pub personen: Vec<LxPerson>,
    pub gemarkungen: BTreeMap<String, Vec<Gemarkung>>,
    pub buchungsblattbezirke: BTreeMap<String, Vec<BuchungsblattBezirk>>,
    pub buchungsblatt_bodenordnung: Vec<LxBuchungsblattBodenordnung>,
    pub abt2_rechte: Vec<LxAbteilung2>,
    pub abt3_rechte: Vec<LxAbteilung3>,
    
    pub auftragsstatus: Option<Auftragsstatus>,
    pub lefis_geladen: Option<Vec<LefisDatei>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum LxPersonRolleArt {
    Katastereigentuemer,
    Grundbucheigentuemer,
    LegitimierterEigentuemer,
    Nebenbeteiligter,
    LegitimierterNebenbeteiligter,
}

impl LxPersonRolleArt {

    pub fn from_usize(u: usize) -> Option<Self> {
        use self::LxPersonRolleArt::*;
        match u {
            1000 => Some(Katastereigentuemer),
            1001 => Some(Grundbucheigentuemer),
            1002 => Some(LegitimierterEigentuemer),
            1003 => Some(Nebenbeteiligter),
            1004 => Some(LegitimierterNebenbeteiligter),
            _ => None,
        }
    }

    pub fn code(&self) -> usize {
        use self::LxPersonRolleArt::*;
        match self {
            Grundbucheigentuemer => 1001,
            Katastereigentuemer => 1000,
            LegitimierterEigentuemer => 1002,
            Nebenbeteiligter => 1003,
            LegitimierterNebenbeteiligter => 1004,
        }
    }
}

// LX21001
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LxPerson {
    pub uuid: String,
    pub beg: DateTime<Utc>,
    pub ax_person: AxPerson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxPerson {
    pub uuid: String,
    pub beg: DateTime<Utc>,
    // Optional, da Firmen auch ohne Anrede 
    // angeschrieben werden können
    pub anrede: Option<Anrede>,
    pub titel: Option<String>,
    pub vorname: Option<String>,
    pub nachname_oder_firma: String,
    pub geburtsname: Option<String>,
    pub wohnort: Option<String>,
}

// LX22004
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LxPersonRolle {
    pub uuid: String,
    pub beg: DateTime<Utc>,
    // Nicht jede Person hat eine PersonenRolle, z.B. Vertretung
    pub person_uuid: String,
    // LX22004.ADET
    pub art: LxPersonRolleArt,
    // LX21006 (LX21006-LX22004)
    pub lx_namensnummer_uuid: String,
}

// LX22003
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdnungsnummerBodenordnung {
    pub uuid: String,
    pub stammnummer: usize,
    pub unternummer: usize,
    // A_LX22003_LX21007
    pub ordnungsnummer_personen: Vec<BeteiligterLxBuchungsblattBodenordnung>,
}

impl OrdnungsnummerBodenordnung {
    pub fn get_lx_namensnummern(&self) -> Vec<String> {
        self.ordnungsnummer_personen
        .iter()
        .flat_map(|r| {
            r.ax_buchungsblatt.buchungsstellen
            .iter()
            .map(|b| b.lx_namensnummer.clone())
        }).collect()
    }
}

// LX21007
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeteiligterLxBuchungsblattBodenordnung {
    pub uuid: String,
    // Falsches "Nebenbeteiligten-Blatt"
    pub ax_buchungsblatt: BeteiligterAxBuchungsblattBodenordnung,
}

// AX21007
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeteiligterAxBuchungsblattBodenordnung {
    pub uuid: String,
    // AX21007 -> [AX21006] via
    pub buchungsstellen: Vec<BeteiligterAxNamensnummer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeteiligterAxNamensnummer {
    // AX21006 - AX_Namensnummer
    pub uuid: String,
    // LX21006
    pub lx_namensnummer: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Gemarkung {
    pub lan19: usize,
    pub gmn19: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BuchungsblattBezirk {
    pub lan16: usize, 
    pub bbb: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UiState {
    tab: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerfahrenGeladenExport {
    pub name: String,
    pub nummer: usize,
    pub uuid: String,
    pub flurstuecke: Vec<AxFlurstueck>,
    pub ordnungsnummern: Vec<OrdnungsnummerBodenordnung>,
    pub personen_rollen: Vec<LxPersonRolle>,
    pub personen: Vec<LxPerson>,
    pub buchungsblatt_bodenordnung: Vec<LxBuchungsblattBodenordnung>,
    pub abt2_rechte: Vec<LxAbteilung2>,
    pub abt3_rechte: Vec<LxAbteilung3>,
}

// AX11001
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct AxFlurstueck {
    pub uuid: String,
    pub kennzeichen: String,
    pub ax21008: String,
    pub lx21008: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct AxFlurstueckOhneVerfahren {
    pub uuid: String,
    pub kennzeichen: String,
    pub ax21008: Option<String>,
    pub lx21008: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxFlurstueckMitVerfahren {
    pub uuid: String,
    pub kennzeichen: String,
    pub ax21008: Option<String>,
    pub lx21008: Option<String>,
    pub verfahren: Option<String>,
}

impl VerfahrenGeladen {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&VerfahrenGeladenExport {
            name: self.name.clone(),
            nummer: self.nummer.clone(),
            uuid: self.uuid.clone(),
            flurstuecke: self.flurstuecke.clone(),
            ordnungsnummern: self.ordnungsnummern.clone(),
            personen: self.personen.clone(),
            personen_rollen: self.personen_rollen.clone(),
            buchungsblatt_bodenordnung: self.buchungsblatt_bodenordnung.clone(),
            abt2_rechte: self.abt2_rechte.clone(),
            abt3_rechte: self.abt3_rechte.clone(),
        })
        .unwrap_or_default()
        .lines()
        .collect::<Vec<_>>()
        .join("\r\n")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxBuchungsblattBezirk {
    pub lan16: String,
    pub bbb: usize,
    pub bez: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxBuchungsstelle {
    pub lx21004: String,
    pub lx21004_erstellt_am: DateTime<Utc>,
    pub lfd_nr_grundbuch: usize,
    pub lx21008: String,
    pub ax21008: String,
    pub ax21007: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Auftragsstatus {
    AuftragWirdFortgefuehrt { prozent: usize },
    Fehler { log: Vec<ProtokollMsg> },
    ErfolgreichFortgefuehrt,
}

/// LX_23001
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LxAbteilung2 {
    pub uuid: String,
    pub erstellt_am: DateTime<Utc>,
    // Recht gelöscht: ENDE != NULL
    pub ende: Option<DateTime<Utc>>,
    pub lfd_nr: usize,
    pub buchungsstellen: Vec<AxBuchungsstelle>,
}

/// LX_23002
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LxAbteilung3 {
    pub uuid: String,
    pub erstellt_am: DateTime<Utc>,
    // Recht gelöscht: ENDE != NULL
    pub ende: Option<DateTime<Utc>>,
    pub lfd_nr: usize,
    pub buchungsstellen: Vec<AxBuchungsstelle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LxBuchungsblattBodenordnung {
    pub uuid: String,
    // Ist dieses Blatt ein fiktives Blatt für NB?
    pub nebenbeteiligten_blatt: bool,
    // GBVE: Haken
    pub grundbuchvergleich_durchgefuehrt: bool,
    // grundbuchvergleich durchgeführt: bool
    // LX_OrdnungsnummerBodenordnung auswählen
    pub ax_buchungsblatt: AxBuchungsblatt,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AxBuchungsblatt {
    pub uuid: String,
    // land
    pub lan16: usize,
    // bezirk (ID)
    pub bbb: usize,
    // bezirk (name)
    pub bbb_name: Option<String>,
    // blattnummer
    pub bbn: usize,
    // blatttyp. 1000 = Grundbuchblatt
    pub blt: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AxBestehendeBuchungsstelle {
    pub uuid: String,
    /// Wichtig, da das Programm NUR Buchungsstellen des AB löschen sollten,
    /// niemals Buchungsstellen des NB
    pub kan: KennzeichnungAlterNeuerBestand,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LefisDatei {
    pub titelblatt: Titelblatt,
    pub rechte: GrundbuchAnalysiert,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Titelblatt {
    pub amtsgericht: String,
    pub grundbuch_von: String,
    pub blatt: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GrundbuchAnalysiert {
    pub abt2: Vec<Abt2Analysiert>,
    pub abt3: Vec<Abt3Analysiert>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct BvEintrag {
    pub lfd_nr: usize,
    pub bisherige_lfd_nr: Option<usize>,
    pub flur: usize,
    // "87" oder "87/2"
    pub flurstueck: String,
    pub gemarkung: Option<String>,
    pub bezeichnung: Option<String>,
    pub groesse: FlurstueckGroesse,
    #[serde(default)]
    pub automatisch_geroetet: bool,
    #[serde(default)]
    pub manuell_geroetet: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
#[serde(tag = "typ", content = "wert")]
pub enum FlurstueckGroesse {
    #[serde(rename = "m")]
    Metrisch { 
        m2: Option<usize>
    },
    #[serde(rename = "ha")]
    Hektar { 
        ha: Option<usize>, 
        a: Option<usize>, 
        m2: Option<usize>,
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum RechteArt {
    SpeziellVormerkung { rechteverweis: usize },                          // Vormerkung zur Sicherung
    Abwasserleitungsrecht,                                                //     Abwasserleitungsrecht
    Auflassungsvormerkung,                                                //     Auflassungsvormerkung
    Ausbeutungsrecht,                                                     //     Ausbeutungsrecht
    AusschlussDerAufhebungDerGemeinschaftGem1010BGB,                      //     Ausschluss der Aufhebung der Gemeinschaft gem. $ 1010 BGB
    Baubeschraenkung,                                                     //     Baubeschränkung
    Bebauungsverbot,                                                      //     Bebauungsverbot
    Benutzungsrecht,                                                      //     Benutzungsrecht
    BenutzungsregelungGem1010BGB,                                         //     Benutzungsregelung gem. §1010 BGB
    Bepflanzungsverbot,                                                   //     Bepflanzungsverbot
    Bergschadenverzicht,                                                  //     Bergschadenverzicht
    Betretungsrecht,                                                      //     Betretungsrecht
    Bewässerungsrecht,                                                    //     Bewässerungsrecht
    BpD,                                                                  //     beschrankte persönliche Dienstbarkeit
    BesitzrechtNachEGBGB,                                                  //     Besitzrecht nach EGBGB
    BohrUndSchuerfrecht,                                                  //     Bohr- und Schürfrecht
    Brunnenrecht,                                                         //     Brunnenrecht
    Denkmalschutz,                                                        //     Denkmalschutz
    DinglichesNutzungsrecht,                                              //     dingliches Nutzungsrecht
    DuldungVonEinwirkungenDurchBaumwurf,                                  //     Duldung von Einwirkungen durch Baumwurf
    DuldungVonFernmeldeanlagen,                                            //     Duldung von Femmeldeanlagen
    Durchleitungsrecht,                                                   //     Durchleitungsrecht
    EinsitzInsitzrecht,                                                   //     Einsitz-/ Insitzrecht
    Entwasserungsrecht,                                                   //     Entwasserungsrecht
    Erbbaurecht,                                                          //     Erbbaurecht
    Erwerbsvormerkung,                                                    //     Erwerbsvormerkung
    Fensterrecht,                                                         //     Fensterrecht
    Fensterverbot,                                                        //     Fensterverbot
    Fischereirecht,                                                       //     Fischereirecht
    Garagenrecht,                                                         //     Garagenrecht
    Gartenbenutzungsrecht,                                                //     Gartenbenutzungsrecht
    GasleitungGasreglerstationFerngasltg,                                 //     Gasleitung‚ Gasreglerstation, Ferngasltg.
    GehWegeFahrOderLeitungsrecht,                                         //     Geh-, Wege-, Fahr- oder Leitungsrecht
    Gewerbebetriebsbeschrankung,                                          //     Gewerbebetriebsbeschrankung
    GewerblichesBenutzungsrecht,                                          //     gewerbliches Benutzungsrecht
    Grenzbebauungsrecht,                                                  //     Grenzbebauungsrecht
    Grunddienstbarkeit,                                                   //     Grunddienstbarkeit
    Hochspannungsleitungsrecht,                                           //     Hochspannungsleitungsrecht
    Immissionsduldungsverpflichtung,                                      //     Immissionsduldungsverpflichtung
    Insolvenzvermerk,                                                     //     Insolvenzvermerk
    Kabelrecht,                                                           //     Kabelrecht
    Kanalrecht,                                                           //     Kanalrecht
    Kiesabbauberechtigung,                                                //     Kiesabbauberechtigung
    Kraftfahrzeugabstellrecht,                                            //     Kraftfahrzeugabstellrecht
    LeibgedingAltenteilsrechtAuszugsrecht,                                //     LeibgedingAttenteilsrechtAuszugsrecht
    LeitungsOderAnlagenrecht,                                             //     LeitungsOderAnlagenrecht
    Mauerrecht,                                                           //     Mauerrecht
    Mitbenutzungsrecht,                                                   //     Mitbenutzungsrecht
    Mobilfunkstationsrecht,                                               //     Mobilfunkstationsrecht
    Muehlenrecht,                                                         //     Mühlenrecht
    Mulltonnenabstellrecht,                                               //     Mulltonnenabstellrecht
    Nacherbenvermerk,                                                     //     Nacherbenvermerk
    Niessbrauchrecht,                                                     //     Nießbrauchrecht
    Nutzungsbeschrankung,                                                 //     Nutzungsbeschrankung
    Pfandung,                                                             //     Pfandung
    Photovoltaikanlagenrecht,                                             //     Photovoltaikanlagenrecht
    Pumpenrecht,                                                          //     Pumpenrecht
    Reallast,                                                             //     Reallast
    RegelungUeberDieHöheDerNotwegrenteGemaess912Bgb,                      //     Regelung über die Höhe der Notwegrente gemaß 8 912 BGB
    RegelungUeberDieHöheDerUeberbaurenteGemaess912Bgb,                    //     Regelung über die Höhe der Überbaurente gemaß $ 912 BGB
    Rueckauflassungsvormerkung,                                           //     Rueckauflassungsvormerkung
    Ruckerwerbsvormerkung,                                                //     Ruckerwerbsvormerkung
    Sanierungsvermerk,                                                    //     Sanierungsvermerk
    Schachtrecht,                                                         //     Schachtrecht
    SonstigeDabagrechteart,                                               //     sonstige dabag-Rechteart
    SonstigeRechte,                                                       //     Sonstige Rechte
    Tankstellenrecht,                                                     //     Tankstellenrecht
    Testamentsvollstreckervermerk,                                        //     Testamentsvollstreckervermerk
    Transformatorenrecht,                                                 //     Transformatorenrecht
    Ueberbaurecht,                                                        //     Überbaurecht
    UebernahmeVonAbstandsflachen,                                         //     Übernahme von Abstandsflachen
    Umlegungsvermerk,                                                     //     Umlegungsvermerk
    Umspannanlagenrecht,                                                  //     Umspannanlagenrecht
    Untererbbaurecht,                                                     //     Untererbbaurecht
    VerausserungsBelastungsverbot,                                        //     Veraußerungs-/Belastungsverbot
    Verfuegungsverbot,                                                    //     Verfügungsverbot
    VerwaltungsUndBenutzungsregelung,                                     //     Verwaltungs- und Benutzungsregelung
    VerwaltungsregelungGem1010Bgb,                                        //     Verwaltungsregelung gem. & 1010 BGB
    VerzichtAufNotwegerente,                                              //     Verzicht auf Notwegerente
    VerzichtAufUeberbaurente,                                             //     Verzicht auf Überbaurente
    Viehtrankerecht,                                                      //     Viehtrankerecht
    Viehtreibrecht,                                                       //     Viehtreibrecht
    Vorkaufsrecht,                                                        //     Vorkaufsrecht
    Wasseraufnahmeverpflichtung,                                          //     Wasseraufnahmeverpflichtung
    Wasserentnahmerecht,                                                  //     Wasserentnahmerecht
    Weiderecht,                                                           //     Weiderecht
    Widerspruch,                                                          //     Widerspruch
    Windkraftanlagenrecht,                                                //     Windkraftanlagenrecht
    Wohnrecht,                                                            //     Wohnrecht
    WohnungsOderMitbenutzungsrecht,                                       //     Wohnungs- oder Mitbenutzungsrecht
    Wohnungsbelegungsrecht,                                               //     Wohnungsbelegungsrecht
    WohnungsrechtNach1093Bgb,                                             //     Wohnungsrecht nach 81093 BGB
    Zaunerrichtungsverbot,                                                 //     Zaunemichtungsverbot
    Zaunrecht,                                                            //     Zaunrecht
    Zustimmungsvorbehalt,                                                 //     Zustimmungsvorbehalt
    Zwangsversteigerungsvermerk,                                          //     Zwangsversteigerungsvermerk
    Zwangsverwaltungsvermerk,                                             //     Zwangsverwaltungsvermerk
}

impl RechteArt {
    pub fn code(&self) -> usize {
        match self {
            RechteArt::Baubeschraenkung => 2000,
            RechteArt::GehWegeFahrOderLeitungsrecht => 4000,
            RechteArt::Grunddienstbarkeit => 3080,
            RechteArt::LeitungsOderAnlagenrecht => 5000,
            RechteArt::Fensterrecht => 2040,
            RechteArt::Grenzbebauungsrecht => 2030,
            RechteArt::Bebauungsverbot => 2020,
            RechteArt::Mitbenutzungsrecht => 3030,
            RechteArt::WohnungsOderMitbenutzungsrecht => 3070,
            RechteArt::Wohnrecht => 3090,
            RechteArt::Benutzungsrecht => 3060,
            RechteArt::Niessbrauchrecht => 7002,
            RechteArt::Reallast => 3100,
            RechteArt::LeibgedingAltenteilsrechtAuszugsrecht => 3130,
            RechteArt::BpD => 3120,
            RechteArt::Abwasserleitungsrecht => 5010,
            RechteArt::Durchleitungsrecht => 6040,
            RechteArt::Wasseraufnahmeverpflichtung => 6030,
            RechteArt::Fischereirecht => 6050,
            RechteArt::Entwasserungsrecht => 6020,
            RechteArt::Muehlenrecht => 6060,
            RechteArt::Kanalrecht => 5020,
            RechteArt::Betretungsrecht => 4010,
            RechteArt::Insolvenzvermerk => 8040,
            RechteArt::GasleitungGasreglerstationFerngasltg => 5060,
            RechteArt::VerausserungsBelastungsverbot => 8020,
            RechteArt::Vorkaufsrecht => 8030,
            RechteArt::Zwangsverwaltungsvermerk => 8051,
            RechteArt::SonstigeRechte => 8000,
            // RechteArt::NichtDefiniert => 9999,
            RechteArt::Erbbaurecht => 2101,
            RechteArt::Untererbbaurecht => 2102,
            RechteArt::Auflassungsvormerkung => 1530,
            RechteArt::Bewässerungsrecht => 1550,
            RechteArt::BohrUndSchuerfrecht => 1560,
            RechteArt::Brunnenrecht => 1570,
            RechteArt::Erwerbsvormerkung => 8010,
            RechteArt::EinsitzInsitzrecht => 1580,
            RechteArt::Kiesabbauberechtigung => 1561,
            RechteArt::Nacherbenvermerk => 1562,
            RechteArt::Windkraftanlagenrecht => 1565,
            RechteArt::Ausbeutungsrecht => 3125,
            RechteArt::Bepflanzungsverbot => 3135,
            RechteArt::Fensterverbot => 2035,
            RechteArt::UebernahmeVonAbstandsflachen => 7000,
            RechteArt::Nutzungsbeschrankung => 7001,
            RechteArt::DinglichesNutzungsrecht => 7004,
            RechteArt::BesitzrechtNachEGBGB => 7005,
            RechteArt::Verfuegungsverbot => 7006,
            RechteArt::Zustimmungsvorbehalt => 7007,
            RechteArt::Rueckauflassungsvormerkung => 7008,
            RechteArt::Wasserentnahmerecht => 7009,
            RechteArt::Sanierungsvermerk => 7010,
            RechteArt::Umlegungsvermerk => 7012,
            RechteArt::Testamentsvollstreckervermerk => 7016,
            RechteArt::Bergschadenverzicht => 7013,
            RechteArt::VerwaltungsUndBenutzungsregelung => 7014,
            RechteArt::Widerspruch => 7015,
            RechteArt::Denkmalschutz => 1541,
            RechteArt::DuldungVonEinwirkungenDurchBaumwurf => 1542,
            RechteArt::DuldungVonFernmeldeanlagen => 1543,
            RechteArt::Garagenrecht => 2041,
            RechteArt::Gartenbenutzungsrecht => 2045,
            RechteArt::Gewerbebetriebsbeschrankung => 2050,
            RechteArt::GewerblichesBenutzungsrecht => 2051,
            RechteArt::Hochspannungsleitungsrecht => 2061,
            RechteArt::Immissionsduldungsverpflichtung => 2070,
            RechteArt::Kabelrecht => 2062,
            RechteArt::Kraftfahrzeugabstellrecht => 2052,
            RechteArt::Mauerrecht => 2053,
            RechteArt::Mobilfunkstationsrecht => 2080,
            RechteArt::Mulltonnenabstellrecht => 2054,
            RechteArt::Photovoltaikanlagenrecht => 2069,
            RechteArt::Pumpenrecht => 2063,
            RechteArt::RegelungUeberDieHöheDerNotwegrenteGemaess912Bgb => 2091,
            RechteArt::RegelungUeberDieHöheDerUeberbaurenteGemaess912Bgb => 2092,
            RechteArt::Schachtrecht => 2065,
            RechteArt::Tankstellenrecht => 2066,
            RechteArt::Transformatorenrecht => 2067,
            RechteArt::Ueberbaurecht => 2096,
            RechteArt::Umspannanlagenrecht => 2068,
            RechteArt::WohnungsrechtNach1093Bgb => 3075,
            RechteArt::VerzichtAufNotwegerente => 2098,
            RechteArt::VerzichtAufUeberbaurente => 2097,
            RechteArt::Viehtreibrecht => 2071,
            RechteArt::Viehtrankerecht => 2074,
            RechteArt::Weiderecht => 2073,
            RechteArt::Wohnungsbelegungsrecht => 3071,
            RechteArt::Zaunerrichtungsverbot => 2057,
            RechteArt::Zaunrecht => 2058,
            RechteArt::Zwangsversteigerungsvermerk => 8050,
            RechteArt::AusschlussDerAufhebungDerGemeinschaftGem1010BGB => 7113,
            RechteArt::Ruckerwerbsvormerkung => 8011,
            RechteArt::BenutzungsregelungGem1010BGB => 7110,
            RechteArt::VerwaltungsregelungGem1010Bgb => 7111,
            RechteArt::Pfandung => 8005,
            RechteArt::SpeziellVormerkung { .. } |
            RechteArt::SonstigeDabagrechteart => 9998,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Abt2Analysiert {
    pub lfd_nr: usize,
    pub text_kurz: String,
    pub rechteart: RechteArt,
    pub rechtsinhaber: String,
    pub rangvermerk: Option<String>,
    pub spalte_2: String,
    // Flur, Flurstück
    pub belastete_flurstuecke: Vec<BvEintrag>,
    pub text_original: String,
    pub nebenbeteiligter: Nebenbeteiligter,
    pub warnungen: Vec<String>,
    pub fehler: Vec<String>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum SchuldenArt {
    Grundschuld,
    Hypothek,
    Rentenschuld,
    Aufbauhypothek,
    Sicherungshypothek,
    Widerspruch,
    Arresthypothek,
    SicherungshypothekGem128ZVG,
    Hoechstbetragshypothek,
    Sicherungsgrundschuld,
    Zwangssicherungshypothek,
    NichtDefiniert,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum AktionsStatus {
    AltesRechtIstZuLoeschen,
    AltesRechtIstZuLoeschenUndNeuZuBegruenden,
    NeuesRecht,
    UebertragenInDenNeuenBestand,
}

impl AktionsStatus {
    pub fn code(&self) -> usize {
        use self::AktionsStatus::*;
        match self {
            AltesRechtIstZuLoeschen => 1001,
            AltesRechtIstZuLoeschenUndNeuZuBegruenden => 1002,
            NeuesRecht => 1003,
            UebertragenInDenNeuenBestand => 1000,
        }
    }
}

impl RechteArt {
    pub fn get_aktionsstatus(&self) -> AktionsStatus {
        match self {
            // Leitungsrechte: zu löschen u. neu
            RechteArt::LeitungsOderAnlagenrecht |
            RechteArt::Abwasserleitungsrecht |
            RechteArt::Durchleitungsrecht |
            RechteArt::GasleitungGasreglerstationFerngasltg |
            RechteArt::Hochspannungsleitungsrecht => AktionsStatus::AltesRechtIstZuLoeschenUndNeuZuBegruenden,

            // Wegerecht: zu löschen
            RechteArt::GehWegeFahrOderLeitungsrecht |
            RechteArt::Verfuegungsverbot
            => AktionsStatus::AltesRechtIstZuLoeschen,

            // Alles andere: zu übertragen
            _ => AktionsStatus::UebertragenInDenNeuenBestand // TODO
        }
    }
}

impl SchuldenArt {
    pub fn get_aktionsstatus(&self) -> AktionsStatus {
        match self {
            _ => AktionsStatus::UebertragenInDenNeuenBestand
        }
    }

    pub fn code(&self) -> usize {
        match self {
            SchuldenArt::Grundschuld => 7010,
            SchuldenArt::Hypothek => 7030,
            SchuldenArt::Rentenschuld => 7060,
            SchuldenArt::Aufbauhypothek => 7032,
            SchuldenArt::Sicherungshypothek => 7037,
            SchuldenArt::Widerspruch => 6001,
            SchuldenArt::Arresthypothek => 7034,
            SchuldenArt::SicherungshypothekGem128ZVG => 7038,
            SchuldenArt::Hoechstbetragshypothek => 7036,
            SchuldenArt::Sicherungsgrundschuld => 7015,
            SchuldenArt::Zwangssicherungshypothek => 7039,
            SchuldenArt::NichtDefiniert => 9999
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Abt3Analysiert {
    pub lfd_nr: usize,
    pub text_kurz: String,
    pub betrag: Betrag,
    pub schuldenart: SchuldenArt,
    pub rechtsinhaber: String,
    pub spalte_2: String,
    // Flur, Flurstück
    pub belastete_flurstuecke: Vec<BvEintrag>,
    pub text_original: String,
    pub nebenbeteiligter: Nebenbeteiligter,
    pub warnungen: Vec<String>,
    pub fehler: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Waehrung { 
    Euro,
    DMark,
    MarkDDR,
    Goldmark,
    Rentenmark,
    Reichsmark,
    GrammFeingold,
}

impl Waehrung {
    pub fn code(&self) -> &'static str {
        use self::Waehrung::*;
        match self {
            Euro => "EUR",
            DMark => "DEM",
            MarkDDR => "MAR",
            Goldmark => "GOM",
            Rentenmark => "REM",
            Reichsmark => "RHM",
            GrammFeingold => "GRF",
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Betrag {
    pub wert: usize,
    pub nachkomma: usize,
    pub waehrung: Waehrung,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Nebenbeteiligter {
    // ONr., falls bereits vergeben
    pub ordnungsnummer: Option<usize>,
    // Typ des NB, wichtig für ONr.
    pub typ: Option<NebenbeteiligterTyp>,
    // Name des NB im Grundbuch
    pub name: String,
    // Informationen für das Anlegen der Ordnungsnummer 
    // (wird 1:1 in LEFIS übernommen)
    #[serde(default)]
    pub extra: NebenbeteiligterExtra,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct NebenbeteiligterExtra {
    pub anrede: Option<Anrede>,
    pub titel: Option<String>,
    pub vorname: Option<String>,
    pub nachname_oder_firma: Option<String>,
    pub geburtsname: Option<String>,
    pub geburtsdatum: Option<DateTime<Utc>>,
    pub wohnort: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum NebenbeteiligterTyp {
    #[serde(rename="OEFFENTLICH")]
    Oeffentlich,
    #[serde(rename="BANK")]
    Bank,
    #[serde(rename="AGRAR")]
    AgrarGenossenschaft,
    #[serde(rename="PRIVAT")]
    PrivateigentuemerMehrere,
    #[serde(rename="PRIVAT-M")]
    PrivateigentuemerHerr,
    #[serde(rename="PRIVAT-F")]
    PrivateigentuemerFrau,
    #[serde(rename="JEW-EIGENT")]
    JewEigentuemerDesFlurstuecks,
    #[serde(rename="LEITUNG")]
    Leitungsbetreiber,
    #[serde(rename="GMBH")]
    GmbH
}

impl LefisUploadKonfiguration {

    fn konfiguration_pfad() -> String {
        std::env::current_exe().ok()
        .and_then(|p| Some({
            p.parent()?.to_path_buf().join("Konfiguration.toml").to_str()?.to_string()
        }))
        .unwrap_or(format!("./Konfiguration.toml"))
    }

    pub fn speichern(&self) {
        let _ = toml::to_string(self).ok().and_then(|s| {
            let s = s.replace("\n", "\r\n");
            std::fs::write(&Self::konfiguration_pfad(), &s.as_bytes()).ok()
        });

    }

    pub fn neu_laden() -> Result<Self, String> {

        if !Path::new(&Self::konfiguration_pfad()).exists() {
            LefisUploadKonfiguration::default().speichern();
        }

        let konfig = match std::fs::read_to_string(&Self::konfiguration_pfad()) {
            Ok(o) => match toml::from_str(&o) {
                Ok(o) => o,
                Err(e) => return Err(format!("Fehler in Konfiguration {}: {}", Self::konfiguration_pfad(), e)),
            },
            Err(e) => return Err(format!("Fehler beim Lesen von Konfiguration in {}: {}", Self::konfiguration_pfad(), e)),
        };

        Ok(konfig)
    }
}

impl LefisInfo {
    // Holt die DB-Verbindungs Parameter vom 3A-Server
    pub fn new(lefis: &LefisUploadKonfigurationLEFIS) -> Result<LefisInfo, RequestFailure> {

        use crate::wsdl::AuftragsManager;

        let am = AuftragsManager::new(
            &lefis.get_webservice_url(), 
            lefis.benutzer.clone(), 
            lefis.passwort.clone()
        );
        
        // TODO: Benutzer ist nicht ausgeloggt wenn Fehler passiert
        exec_sync(async {
            
            let version = am.get_version().await?;
            let login = am.login().await?;
            let sde_connection = am.get_preference(login.session_id, "AaaServer.AaaManagementServer.SdeConn#Connect#String#").await?;
            
            let db_parameter = OracleDbParameter::new(&sde_connection.preference)
                .ok_or(RequestFailure::FailedToDeserializeResponse(sde_connection.preference.clone()))?;

            am.logout(login.session_id).await?;
            Ok(LefisInfo {
                version: version.get_version_result,
                oracle: db_parameter,
            })
        })
    }
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

impl Drop for DhkVerbindung {
    fn drop(&mut self) {
        let _ = self.conn.close();
    }
}

impl DhkVerbindung {
    pub fn new(zugangsdaten: &OracleDbParameter) -> Result<Self, String> {
        match oracle::Connection::connect(
            zugangsdaten.benutzer.clone().unwrap_or_default(), 
            zugangsdaten.passwort.clone().unwrap_or_default(),
            zugangsdaten.server.clone(), 
        ) {
            Ok(o) => Ok(Self { conn: o, zugangsdaten: zugangsdaten.clone() }),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn lade_verfahren(&self) -> Result<GeladeneVerfahren, String> {
        
        // alle Tabellen mit Verfahren laden
        let query = "SELECT owner FROM sys.dba_tables WHERE table_name = 'LX91003'";

        let mut stmt = self.conn.prepare(query, &[StmtParam::FetchArraySize(10000)])
        .map_err(|e| {
            format!("FEHLER in conn.prepare(\"SELECT owner, table_name FROM sys.dba_tables WHERE table_name = 'LX91003'\"): {}", e)
        })?;

        let rr = stmt.query_as::<String>(&[])
        .map_err(|e| {
            format!("FEHLER in conn.query(\"SELECT owner, table_name FROM sys.dba_tables WHERE table_name = 'LX91003'\"): {}", e)
        })?;

        let mut verfahren_tabellen = rr
        .into_iter()
        .filter_map(|r| r.ok())
        .collect::<Vec<String>>();

        verfahren_tabellen.sort();
        verfahren_tabellen.dedup();

        let mut verfahren = Vec::new();

        let mut flurstuecke_ohne_verfahren = Vec::new();

        for schema in verfahren_tabellen {

            // Lade alle LX_Person / AxPerson aus dem Verfahren
            let query_lx_person = format!("
                SELECT a.UUID, a.BEG, a.LX91003, b.UUID, b.BEG, b.ANR, b.VNA, b.NBA, b.AKD, b.GNA, b.GEB, b.WOS, b.BER, b.SOS, b.HLG, b.NOF FROM {schema}.LX21001 a 
                INNER JOIN {schema}.AX21001 b ON 
                a.AX21001 = b.UUID", 
                schema = schema
            );

            let mut stmt = self.conn.prepare(&query_lx_person, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_lx_person, e)
            })?;

            let mut lx_person_map = BTreeMap::new();
            if let Ok(rr) = stmt.query_as::<(
                String, 
                DateTime<Utc>, 
                String, 

                String,
                DateTime<Utc>, 
                Option<usize>,
                Option<String>, 
                Option<String>, 
                Option<String>, 
                Option<String>, 
                Option<DateTime<Utc>>, 
                Option<String>, 
                Option<String>, 
                Option<String>, 
                Option<String>, 
                String, 
            )>(&[]) {
                
                for (
                    uuid, 
                    beg, 
                    verfahren_uuid, 

                    ax_uuid, 
                    ax_beg,
                    ax_anrede, 
                    ax_vorname, 
                    ax_nba, 
                    ax_akd, 
                    ax_geburtsname, 
                    ax_geburtsdatum,
                    ax_wohnort, 
                    ax_beruf, 
                    ax_sonstiges,
                    ax_hlg,
                    ax_nachname
                ) in rr.into_iter().filter_map(|r| r.ok()) {
                    
                    let ax_anrede = ax_anrede.and_then(|a| Anrede::from_usize(a));

                    lx_person_map
                    .entry(verfahren_uuid)
                    .or_insert_with(|| Vec::new())
                    .push(LxPerson {
                        uuid,
                        beg,
                        ax_person: AxPerson {
                            uuid: ax_uuid,
                            beg: ax_beg,
                            anrede: ax_anrede,
                            titel: ax_nba,
                            vorname: ax_vorname,
                            nachname_oder_firma: ax_nachname,
                            geburtsname: ax_geburtsname,
                            wohnort: ax_wohnort,
                        }
                    })
                }
            }

            // Lade alle LX_PersonRolle mit LX_Namensnummer aus dem Verfahren
            // Achtung: Vertretungen werden nicht geladen
            let query_lx_person_rolle = format!("
                SELECT a.UUID, a.BEG, a.ADET, a.LX91003, c.UUID, e.UUID FROM {schema}.LX22004 a 
                INNER JOIN {schema}.A_LX21001_LX22004 b ON b.DFID = a.UUID 
                INNER JOIN {schema}.LX21001 c ON c.UUID = b.OFID
                INNER JOIN {schema}.A_LX21006_LX22004 d ON d.DFID = a.UUID
                INNER JOIN {schema}.LX21006 e ON e.UUID = d.OFID", 
                schema = schema
            );
            
            let mut stmt = self.conn.prepare(&query_lx_person_rolle, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_lx_person_rolle, e)
            })?;

            let mut lx_personen_rollen_map = BTreeMap::new();
            match stmt.query_as::<(String, DateTime<Utc>, usize, String, String, String)>(&[]) {
                Ok(rr) => {
                    for (person_rolle_uuid, person_rolle_beg, adet, verfahren_uuid, person_uuid, lx_namensnummer_uuid) in rr.into_iter().filter_map(|r| r.ok()) {
                         
                         let art = match LxPersonRolleArt::from_usize(adet) {
                             Some(s) => s,
                             None => continue,
                         };

                         lx_personen_rollen_map
                         .entry(verfahren_uuid)
                         .or_insert_with(|| Vec::new())
                         .push(LxPersonRolle {
                             uuid: person_rolle_uuid,
                             beg: person_rolle_beg,
                             art,
                             person_uuid: person_uuid,
                             lx_namensnummer_uuid: lx_namensnummer_uuid,
                         });
                    }   
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }

            // Lade alle AX_Buchungsblatt => AX_Namensnummer <=> LX_Namensnummer
            let query_ax_namensnummer = format!("
                SELECT b.UUID, a.UUID, a.AX21007 FROM {schema}.AX21006 a 
                INNER JOIN {schema}.LX21006 b ON a.UUID = b.AX21006", schema = schema);

            let mut stmt = self.conn.prepare(&query_ax_namensnummer, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_ax_namensnummer, e)
            })?;

            let mut buchungsblatt_namensnummer_map = BTreeMap::new();
            match stmt.query_as::<(String, String, String)>(&[]) {
                Ok(rr) => {
                    for (lx_namensnummer, ax_namensnummer, ax_buchungsblatt) in rr.into_iter().filter_map(|r| r.ok()) {
                        buchungsblatt_namensnummer_map.entry(ax_buchungsblatt)
                        .or_insert_with(|| Vec::new())
                        .push((ax_namensnummer, lx_namensnummer));
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }

            // Lade alle LX_Ordnungsnummer_Bodenordnung inkl. Lx_BuchungsblattBodenordnung, gruppiert nach LX_Verfahren
            let query_lx_onr = format!("
                SELECT a.UUID, a.OSNR, a.OUNR, a.LX91003, c.UUID, d.UUID FROM {schema}.LX22003 a
                LEFT JOIN {schema}.A_LX22003_LX21007 b ON a.UUID = b.OFID
                INNER JOIN {schema}.LX21007 c ON c.UUID = b.DFID
                INNER JOIN {schema}.AX21007 d ON c.AX21007 = d.UUID
            ", schema = schema);
            
            let mut stmt = self.conn.prepare(&query_lx_onr, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_lx_onr, e)
            })?;

            let mut ordnungsnummer_map = BTreeMap::new();

            if let Ok(rr) = stmt.query_as::<(String, usize, usize, String, String, String)>(&[]) {
                
                let mut onr = BTreeMap::new();

                for (uuid, stammnummer, unternummer, verfahren_uuid, lx_buchungsblatt_uuid, ax_buchungsblatt_uuid) in rr.into_iter().filter_map(|r| r.ok()) {
                    
                    let buchungsstellen = match buchungsblatt_namensnummer_map.get(&ax_buchungsblatt_uuid) {
                        Some(s) => s.clone(),
                        None => continue,
                    };

                    onr.entry(verfahren_uuid)
                    .or_insert_with(|| BTreeMap::new())
                    .entry((uuid, stammnummer, unternummer))
                    .or_insert_with(|| Vec::new())
                    .push(BeteiligterLxBuchungsblattBodenordnung {
                        uuid: lx_buchungsblatt_uuid,
                        ax_buchungsblatt: BeteiligterAxBuchungsblattBodenordnung {
                            uuid: ax_buchungsblatt_uuid,
                            buchungsstellen: buchungsstellen
                            .into_iter()
                            .map(|(ax_namensnummer, lx_namensnummer)| {
                                BeteiligterAxNamensnummer {
                                    uuid: ax_namensnummer,
                                    lx_namensnummer,
                                }
                            }).collect(),
                        }
                    });
                }

                for (verfahren_uuid, onr_in_verfahren) in onr {
                    for ((uuid, stammnummer, unternummer), ordnungsnummer_personen) in onr_in_verfahren {
                        ordnungsnummer_map
                        .entry(verfahren_uuid.clone())
                        .or_insert_with(|| Vec::new())
                        .push(OrdnungsnummerBodenordnung {
                            uuid, 
                            stammnummer, 
                            unternummer, 
                            ordnungsnummer_personen,
                        });
                    }
                }
            }

            // Lade alle AX_Flurstuecke gruppiert nach LX_Verfahren
            let query_ax_flurstuecke = format!("
                SELECT a.UUID, a.FSK, b.LX91003, b.UUID, b.AX21008 
                FROM {schema}.AX11001 a 
                LEFT JOIN {schema}.LX21008 b 
                ON a.AX21008 = b.AX21008");
            
            let mut stmt = self.conn.prepare(&query_ax_flurstuecke, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_ax_flurstuecke, e)
            })?;

            let mut flurstuecke_map = BTreeMap::new();
            match stmt.query_as::<(String, String, Option<String>, Option<String>, Option<String>)>(&[]) {
                Ok(rr) => {

                    let flurstuecke = rr
                    .into_iter()
                    .filter_map(|r| r.ok())
                    .map(|(ax_uuid, ax_fsk, lx_verfahren, lx_uuid, lx_ax_uuid)| {
                        AxFlurstueckMitVerfahren {
                            uuid: ax_uuid,
                            kennzeichen: ax_fsk,
                            ax21008: lx_ax_uuid,
                            lx21008: lx_uuid,
                            verfahren: lx_verfahren,
                        }
                    })
                    .collect::<Vec<_>>();

                    for f in flurstuecke {
                        match (f.verfahren, f.ax21008, f.lx21008) {
                            (Some(verfahren_uuid), Some(ax21008), Some(lx21008)) => {
                                flurstuecke_map
                                .entry(verfahren_uuid)
                                .or_insert_with(|| Vec::new())
                                .push(AxFlurstueck {
                                    uuid: f.uuid,
                                    kennzeichen: f.kennzeichen,
                                    ax21008,
                                    lx21008,
                                });
                            },
                            (_, ax21008, lx21008) => {
                                flurstuecke_ohne_verfahren.push(AxFlurstueckOhneVerfahren {
                                    uuid: f.uuid,
                                    kennzeichen: f.kennzeichen,
                                    ax21008: ax21008,
                                    lx21008: lx21008,
                                });
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("fehler: {}", e);
                }
            }

            // mapping: AX21008 (AX_Buchungsstelle) -> AX21007 (AX_Buchungsblatt)
            let query_ax21008_ax21007 = format!("SELECT UUID, AX21007B FROM {schema}.AX21008", schema = schema);

            let mut stmt = self.conn.prepare(&query_ax21008_ax21007, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_ax21008_ax21007, e)
            })?;

            let mut mapping_ax21008_ax21007 = BTreeMap::new();
            match stmt.query_as::<(String, String)>(&[]) {
                Ok(o) => {
                    for (ax21008, ax21007) in o.into_iter().filter_map(|o| o.ok()) {
                        mapping_ax21008_ax21007.insert(ax21008.clone(), ax21007.clone());
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }


            // mapping: LxBuchungsstelleBelastet zu LxBuchungsstelleBodenordnung
            let query_buchungsstelle_belastet_zu_buchungsstelle_bodenordnung = format!("
                SELECT OFID, DFID FROM {schema}.A_LX23004_LX21008B
            ", schema = schema);

            let mut stmt = self.conn.prepare(&query_buchungsstelle_belastet_zu_buchungsstelle_bodenordnung, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_buchungsstelle_belastet_zu_buchungsstelle_bodenordnung, e)
            })?;

            let mut buchungsstelle_belastet_zu_buchungsstelle_bodenordnung = BTreeMap::new();
            if let Ok(rr) = stmt.query_as::<(String, String)>(&[]) {
                for (ofid /* 23004 */, dfid /* 21008 */) in rr.into_iter().filter_map(|r| r.ok()) {
                    buchungsstelle_belastet_zu_buchungsstelle_bodenordnung
                    .entry(ofid)
                    .or_insert_with(|| Vec::new())
                    .push(dfid);
                }
            }

            // wähle alle AxBuchungsblattBezirke aus, um den Namen des BBZ zu erhalten
            let query_ax_buchungsblattbezirke = format!("SELECT LAN16, BBB, BEZ FROM {schema}.AX73010");
            
            let mut stmt = self.conn.prepare(&query_ax_buchungsblattbezirke, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_ax_buchungsblattbezirke, e)
            })?;

            let mut ax_buchungsblattbezirke_reverse_map = BTreeMap::new();
            let mut ax_buchungsblattbezirke_map = BTreeMap::new();
            if let Ok(rr) = stmt.query_as::<(usize, usize, String)>(&[]) {
                for r in rr.into_iter().filter_map(|o| o.ok()) {
                    ax_buchungsblattbezirke_map.insert((r.0.clone(), r.1.clone()), r.2.clone());
                    ax_buchungsblattbezirke_reverse_map
                        .entry(r.2)
                        .or_insert_with(|| Vec::new())
                        .push(BuchungsblattBezirk {
                            lan16: r.0,
                            bbb: r.1,
                        });
                }
            }
            
            // wähle alle AxGemarkung aus, um den Namen der Gemarkungen zu erhalten
            let query_ax_gemarkungen = format!("SELECT LAN19, GMN19, BEZ FROM {schema}.AX73007");
            
            let mut stmt = self.conn.prepare(&query_ax_gemarkungen, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_ax_gemarkungen, e)
            })?;

            let mut ax_gemarkungen_reverse_map = BTreeMap::new();
            let mut ax_gemarkungen_map = BTreeMap::new();
            if let Ok(rr) = stmt.query_as::<(usize, usize, String)>(&[]) {
                for r in rr.into_iter().filter_map(|o| o.ok()) {
                    ax_gemarkungen_map.insert((r.0.clone(), r.1.clone()), r.2.clone());
                    ax_gemarkungen_reverse_map
                        .entry(r.2)
                        .or_insert_with(|| Vec::new())
                        .push(Gemarkung {
                            lan19: r.0,
                            gmn19: r.1,
                        });
                }
            }

            // alle LX_BuchungsblattBodenordnung laden mit Verfahrens-ID und joinen mit AX_Buchungsblatt
            let query_grundbuchblattbezirke = format!("
                SELECT a.UUID, a.LX91003, a.KAN, a.NEBL, a.GBVE, a.AX21007, b.LAN16, b.BBB, b.BBN, b.BLT 
                FROM {schema}.LX21007 a
                INNER JOIN {schema}.AX21007 b ON a.AX21007 = b.UUID
            ");
        
            let mut stmt = self.conn.prepare(&query_grundbuchblattbezirke, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_grundbuchblattbezirke, e)
            })?;
        
            let mut buchungsblatt_bodenordnung_map = BTreeMap::new();
            if let Ok(rr) = stmt.query_as::<(String, String, usize, usize, usize, String, usize, usize, usize, usize)>(&[]) {

                let buchungsblatt_bodenordnung = rr
                .into_iter()
                .map(|abt2| {
                    let (
                        uuid, 
                        verfahren_uuid,
                        kan,
                        nebenbeteiligten_blatt,
                        grundbuchvergleich_durchgefuehrt,
                        ax_buchungsblatt_uuid,
                        lan16,
                        bbb,
                        bbn,
                        blt,
                    ) = abt2?;

                    let nebenbeteiligten_blatt = nebenbeteiligten_blatt == 1;
                    let grundbuchvergleich_durchgefuehrt = grundbuchvergleich_durchgefuehrt == 1;

                    let kan = match KennzeichnungAlterNeuerBestand::from_usize(kan) {
                        Some(s) => s,
                        None => { return Err(oracle::Error::InternalError(format!("Ungültige KAN für AX_BuchungsblattBodenordnung: {}", kan))); },
                    };

                    let bbb_name = ax_buchungsblattbezirke_map
                    .get(&(lan16, bbb))
                    .cloned();

                    Ok((verfahren_uuid.clone(), LxBuchungsblattBodenordnung {
                        uuid,
                        nebenbeteiligten_blatt,
                        grundbuchvergleich_durchgefuehrt,
                        ax_buchungsblatt: AxBuchungsblatt {
                            uuid: ax_buchungsblatt_uuid,
                            lan16,
                            bbb,
                            bbb_name,
                            bbn,
                            blt,
                        }
                    }))
                })
                .collect::<Result<Vec<_>, oracle::Error>>()
                .map_err(|e| {
                    format!("FEHLER in conn.query(\"{}\"): {}", query_grundbuchblattbezirke, e)
                })?;


                for (k, v) in buchungsblatt_bodenordnung {
                    buchungsblatt_bodenordnung_map
                    .entry(k)
                    .or_insert_with(|| Vec::new())
                    .push(v)
                }
            }

            // mapping: LX21008 -> AX21008
            let query_lx21008_ax21008 = format!("SELECT UUID, AX21008 FROM {schema}.LX21008", schema = schema);

            let mut stmt = self.conn.prepare(&query_lx21008_ax21008, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_lx21008_ax21008, e)
            })?;

            let mut mapping_lx21008_ax21008 = BTreeMap::new();
            let mut mapping_ax21008_lx21008 = BTreeMap::new();
            match stmt.query_as::<(String, String)>(&[]) {
                Ok(o) => {
                    for (lx21008, ax21008) in o.into_iter().filter_map(|r| r.ok()) {
                        mapping_lx21008_ax21008.insert(lx21008.clone(), ax21008.clone());
                        mapping_ax21008_lx21008.insert(ax21008, lx21008);
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }

            // alle Abt 2. Rechte
            let query_abt2 = format!("                
                SELECT a.UUID, a.BEG, a.ENDE, a.LFD, a.LX91003, c.UUID, c.BEG
                FROM {schema}.LX23001 a
                INNER JOIN {schema}.A_LX23001_LX23004  b ON a.UUID = b.OFID
                INNER JOIN {schema}.LX23004 c ON b.DFID = c.UUID
            ", schema = schema);

            let mut stmt = self.conn.prepare(&query_abt2, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_abt2, e)
            })?;

            let mut abteilung2_rechte_in_schema_map = BTreeMap::new();

            if let Ok(rr) = stmt.query_as::<(String, DateTime<Utc>, Option<DateTime<Utc>>, usize, String, String, DateTime<Utc>)>(&[]) {

                let abteilung2_rechte_in_schema = rr
                .into_iter()
                .filter_map(|o| o.ok())
                .map(|abt2| {

                    let (
                        uuid, 
                        erstellt_am,
                        ende,
                        lfd_nr, 
                        verfahren_uuid,
                        buchungsstelle_belastet_uuid,
                        lx21004_erstellt_am,
                    ) = abt2;
                    
                    //  LX_21008
                    let buchungsstelle_bodenordnung_uuids = match buchungsstelle_belastet_zu_buchungsstelle_bodenordnung.get(&buchungsstelle_belastet_uuid) {
                        Some(s) => s,
                        None => { return Err(oracle::Error::InternalError(format!("Für Recht {} konnte keine Buchungssstelle ermittelt werden", uuid))); },
                    };

                    // AX_21008 = Buchungssstelle
                    let mut buchungsstellen = Vec::new();
                    for lx21008 in buchungsstelle_bodenordnung_uuids.iter() {

                        let ax21008 = match mapping_lx21008_ax21008.get(lx21008) {
                            Some(s) => s.clone(),
                            None => { 
                                // return Err(oracle::Error::InternalError(format!("Für LX_Buchungsstelle {} konnte keine AX_Buchungsstelle ermittelt werden", lx21008))); 
                                continue;
                            },
                        };

                        // AX21007 = Buchungsblatt
                        let ax21007 = match mapping_ax21008_ax21007.get(&ax21008) {
                            Some(s) => s.clone(),
                            None => { 
                                // return Err(oracle::Error::InternalError(format!("Für Buchungsstelle {} konnte kein AX_Buchungsblatt ermittelt werden", ax21008))); 
                                continue;
                            },
                        };

                        buchungsstellen.push(AxBuchungsstelle {
                            lx21004: buchungsstelle_belastet_uuid.clone(),
                            lx21004_erstellt_am,
                            lfd_nr_grundbuch: lfd_nr,
                            lx21008: lx21008.clone(),
                            ax21008,
                            ax21007,
                        });
                    }

                    Ok((verfahren_uuid.clone(), LxAbteilung2 {
                        uuid,
                        erstellt_am,
                        ende,
                        lfd_nr,
                        buchungsstellen,
                    }))
                })
                .collect::<Result<Vec<_>, oracle::Error>>()
                .map_err(|e| {
                    format!("FEHLER in conn.query(\"{}\"): {}", query_abt2, e)
                })?;

                for (k, v) in abteilung2_rechte_in_schema {
                    abteilung2_rechte_in_schema_map
                    .entry(k)
                    .or_insert_with(|| Vec::new())
                    .push(v)
                }
            }

            // alle Abt 3. Rechte
            // lfd. Nr., textlicher Teil, RechteArt (Schuld), Aktionsstatus, Rangvermerk, eingetragen am, Verfahrens-UUID
            let query_abt3 = format!("
                SELECT a.UUID, a.BEG, a.ENDE, a.LFD, a.LX91003, c.UUID, c.BEG
                FROM {schema}.LX23002 a
                INNER JOIN {schema}.A_LX23002_LX23004  b ON a.UUID = b.OFID
                INNER JOIN {schema}.LX23004 c ON b.DFID = c.UUID
            ", schema = schema);

            let mut stmt = self.conn.prepare(&query_abt3, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query_abt3, e)
            })?;

            let mut abteilung3_rechte_in_schema_map = BTreeMap::<String, Vec<LxAbteilung3>>::new();
            if let Ok(rr) = stmt.query_as::<(String, DateTime<Utc>, Option<DateTime<Utc>>, usize, String, String, DateTime<Utc>)>(&[]) {

                let abteilung3_rechte_in_schema = rr
                .into_iter()
                .filter_map(|o| o.ok())
                .map(|abt3| {

                    let (
                        uuid,
                        erstellt_am,
                        ende,
                        lfd_nr,
                        verfahren_uuid,
                        buchungsstelle_belastet_uuid,
                        lx21004_erstellt_am,
                    ) = abt3;
                    
                    //  LX_21008
                    let buchungsstelle_bodenordnung_uuids = match buchungsstelle_belastet_zu_buchungsstelle_bodenordnung.get(&buchungsstelle_belastet_uuid) {
                        Some(s) => s,
                        None => { return Err(oracle::Error::InternalError(format!("Für Recht {} konnte keine Buchungssstelle ermittelt werden", uuid))); },
                    };

                    // AX_21008
                    let mut buchungsstellen = Vec::new();
                    for lx21008 in buchungsstelle_bodenordnung_uuids.iter() {

                        let ax21008 = match mapping_lx21008_ax21008.get(lx21008) {
                            Some(s) => s.clone(),
                            None => { 
                                // return Err(oracle::Error::InternalError(format!("Für LX_Buchungsstelle {} konnte keine AX_Buchungsstelle ermittelt werden", lx21008))); 
                                continue;
                            },
                        };

                        let ax21007 = match mapping_ax21008_ax21007.get(&ax21008) {
                            Some(s) => s.clone(),
                            None => { 
                                // return Err(oracle::Error::InternalError(format!("Für Buchungsstelle {} konnte kein AX_Buchungsblatt ermittelt werden", ax21008))); 
                                continue;
                            },
                        };

                        buchungsstellen.push(AxBuchungsstelle {
                            lx21004: buchungsstelle_belastet_uuid.clone(),
                            lx21004_erstellt_am,
                            lfd_nr_grundbuch: lfd_nr,
                            lx21008: lx21008.clone(),
                            ax21008,
                            ax21007,
                        });
                    }

                    Ok((verfahren_uuid.clone(), LxAbteilung3 {
                        uuid,
                        ende,
                        erstellt_am,
                        lfd_nr,
                        buchungsstellen,
                    }))
                })
                .collect::<Result<Vec<_>, oracle::Error>>()
                .map_err(|e| {
                    format!("FEHLER in conn.query(\"{}\"): {}", query_abt3, e)
                })?;

                for (k, v) in abteilung3_rechte_in_schema {
                    abteilung3_rechte_in_schema_map
                    .entry(k)
                    .or_insert_with(|| Vec::new())
                    .push(v)
                }
            }


            // Oracle erlaubt kein Schema-Name als Variable
            let query = format!("SELECT VNR, VKBZ, VNAM, UUID FROM {}.LX91003", schema);

            let mut stmt = self.conn.prepare(&query, &[StmtParam::FetchArraySize(10000)])
            .map_err(|e| {
                format!("FEHLER in conn.prepare(\"{}\"): {}", query, e)
            })?;

            let rr = stmt.query_as::<(usize, String, String, String)>(&[])
            .map_err(|e| {
                format!("FEHLER in conn.query(\"{}\"): {}", query, e)
            })?;

            let mut verfahren_in_tabelle = rr
            .into_iter()
            .filter_map(|r| r.ok())
            .map(|(vnr, vkbz, vnam, uuid)| {

                let buchungsblatt_bodenordnung = buchungsblatt_bodenordnung_map.get(&uuid).cloned().unwrap_or_default();
                let a2 = abteilung2_rechte_in_schema_map.get(&uuid).cloned().unwrap_or_default();
                let a3 = abteilung3_rechte_in_schema_map.get(&uuid).cloned().unwrap_or_default();

                VerfahrenGeladen {

                    ui: UiState::default(),
                    nummer: vnr,
                    name: if vnam.is_empty() { vkbz } else { vnam },
                    dhk_verbindung: schema.clone(),
                    uuid: uuid.clone(),

                    flurstuecke: flurstuecke_map.get(&uuid).cloned().unwrap_or_default(),
                    gemarkungen: ax_gemarkungen_reverse_map.clone(),
                    personen: lx_person_map.get(&uuid).cloned().unwrap_or_default(),
                    personen_rollen: lx_personen_rollen_map.get(&uuid).cloned().unwrap_or_default(),
                    buchungsblatt_bodenordnung: buchungsblatt_bodenordnung,
                    buchungsblattbezirke: ax_buchungsblattbezirke_reverse_map.clone(),
                    ordnungsnummern: ordnungsnummer_map.get(&uuid).cloned().unwrap_or_default(),

                    abt2_rechte: a2,
                    abt3_rechte: a3,

                    auftragsstatus: None,
                    lefis_geladen: None,
                }
            })
            .collect::<Vec<_>>();

            verfahren.append(&mut verfahren_in_tabelle);
        }

        flurstuecke_ohne_verfahren.sort();
        flurstuecke_ohne_verfahren.dedup();
        flurstuecke_ohne_verfahren.sort_by(|a, b| a.kennzeichen.cmp(&b.kennzeichen));

        verfahren.sort_by(|a, b| a.nummer.cmp(&b.nummer));

        Ok(GeladeneVerfahren {
            verfahren,
            flurstuecke_ohne_verfahren,
        })
    }
}


extern "C" fn render(data: &mut RefAny, _: &mut LayoutCallbackInfo) -> StyledDom {
    let data_clone = data.clone();
    let data = match data.downcast_ref::<AppData>() {
        Some(s) => s,
        None => return StyledDom::default(),
    };

    crate::ui::render(data_clone, &data)
    .style(Css::empty())
}

fn main() {

    let mut konnte_verfahren_nicht_laden_startup = false;

    let (konfiguration, db) = match LefisUploadKonfiguration::neu_laden() {
        Ok(o) => {
            let lefis_info = match LefisInfo::new(&o.lefis) {
                Ok(o) => Some(o),
                Err(e) => {
                    konnte_verfahren_nicht_laden_startup = true;
                    None
                },
            };

            (o, lefis_info)
        },
        Err(e) => { 
            konnte_verfahren_nicht_laden_startup = true;
            (LefisUploadKonfiguration::default(), None) 
        },
    };

    let dhk_verbindung = match db.as_ref() {
        Some(s) => match DhkVerbindung::new(&s.oracle) {
            Ok(o) => Some(o),
            Err(e) => {
                konnte_verfahren_nicht_laden_startup = true;
                None
            }
        },
        None => None,
    };

    let geladene_verfahren = if konnte_verfahren_nicht_laden_startup {
        GeladeneVerfahren::default()
    } else {
        dhk_verbindung.as_ref().and_then(|dhk| match dhk.lade_verfahren() {
            Ok(o) => Some(o),
            Err(e) => {
                println!("Fehler beim Laden der Verfahren: {}", e);
                MsgBox::error(format!("Fehler beim Laden der Verfahren: {}", e).into());
                None
            }
        }).unwrap_or_default()
    };

    let mut app = App::new(RefAny::new(AppData {
        verfahren_filter: None,
        konfiguration,
        lefis_info: db,
        dhk_verbindung,
        geladene_verfahren,
        ausgewaehltes_verfahren: None,
    }), AppConfig::new(LayoutSolver::Default));

    if konnte_verfahren_nicht_laden_startup {
        app.add_window(crate::ui::konfiguration_fenster());
    }

    let mut root = WindowCreateOptions::new(render);
    root.state.title = "LefisUpload".into();
    root.state.flags.frame = WindowFrame::Maximized;
    app.run(root);
}
