use std::env;

use vcontrol::{Device, Optolink, VControl, device::ECOTRONIC, Value};

fn main() {
  env_logger::init();

  let optolink_port = env::args().nth(1).expect("no serial port specified");
  let optolink = if optolink_port.contains(':') {
    Optolink::connect(optolink_port)
  } else {
    Optolink::open(optolink_port)
  }.unwrap();

  let mut vcontrol = VControl::<ECOTRONIC>::connect(optolink).unwrap();

  let map = ECOTRONIC::map();

  dbg!(vcontrol.get("NRF_Uhrzeit")).unwrap();
  dbg!(vcontrol.get("Ecotronic_DA_Absperrschieber").unwrap());
  dbg!(vcontrol.get("Ecotronic_Abgas_Minimaltemperatur").unwrap());
  dbg!(vcontrol.get("Ecotronic_LAN_Proxy_Passwort").unwrap());
  dbg!(vcontrol.get("KennungCodierkarte").unwrap());
  dbg!(vcontrol.get("Ecotronic_Abgastemperatur").unwrap());
  dbg!(vcontrol.get("NRF_ResetUrsache").unwrap());
  dbg!(vcontrol.get("Ecotronic_Schaltzeiten_FT").unwrap());

  // dbg!(vcontrol.get("SC100_KesselsollEffektiv")).unwrap();
  // dbg!(vcontrol.get("SC100_GefoerdertesMaterialPellet")).unwrap();
  // dbg!(vcontrol.get("NRF_Brennstoffverbrauch_Bedien")).unwrap();
  // dbg!(vcontrol.get("NRF_K2F_KonfiKorrfaktorPelletverbrauch")).unwrap();

  let mut keys = map.keys().collect::<Vec<_>>();
  keys.sort();

  for key in keys {
    eprintln!("{}:", key);

    let res = vcontrol.get(key);

    match res {
      Ok(value) => {
        if !matches!(value, Value::Empty) {
          eprintln!("  {:?}", value);
        }
      },
      Err(err) => {
        eprintln!("{} error: {:?}", key, err);
      },
    }

    // sleep(Duration::from_secs(1));
  }

  // Missing:
  // Abgasetemperatur


  let known_keys = [
    "AktorentestVBC550",
    "Ausgang52_Mischerposition_M2",
    "BedienAnzeigetext_FBM1",
    "BedienAnzeigetext_FBM2",
    "BedienparameterA1M1FunktionReset",
    "BedienparameterM2FunktionReset",
    "BedienteilBA_GWGA1",
    "BrennerstartsBedien",
    "DIPSchalterKAE0_10V_Bereich",
    "DIPSchalterKAE0_10V_Pumpen",
    "DIPSchalterKAE0_10V_PumpenKR",
    "DigitalAusgang_29",
    "ExternSollwert0_10V",
    "Fehlerhistorie0",
    "HK_Mischerposition_akt_M1",
    "HK_Mischerposition_akt_M2",
    "HK_Sparschaltung_AbsSommerM2",
    "HK_Sparschaltung_MischerM2",
    "HK_Sparschaltung_PumpenstillstandM2",
    "HK_Sparschaltung_RaumSommerM2",
    "K36_KonfiVTMinKaskade",
    "K54_Solar",
    "K54_Solar_Info",
    "K73_KonfiZpumpeIntervallFreigabe",
    "K76_KonfiKommunikationsmodul_D",
    "K7F_KonfiEinfamilienhaus",
    "K81_KonfiFunktionUhr",
    "K81_Konfi_FunktionUhr_D",
    "K90_KonfiZeitkonstAussentemperatur",
    "K98_KonfiVI_Anlagennummer",
    "K98_KonfiVI_Anlagennummer_D",
    "K9D_KonfiKennungKAE0_10V",
    "KA0_FernbedienungM2",
    "KA0_KennungFernbedienungA1M1",
    "KA0_KonfiKennungFernbedienungA1M1",
    "KA0_KonfiKennungFernbedienungM2",
    "KA2_KonfiSpeichervorrangA1M1",
    "KA2_KonfiSpeichervorrangM2",
    "KA3_KonfiFrostgrenzeA1M1",
    "KA3_KonfiFrostgrenzeM2",
    "KA4_KonfiKeineFrostschutzfunktionA1M1",
    "KA4_KonfiKeineFrostschutzfunktionM2",
    "KA5_KonfSommersparSchaltschwelleA1M1",
    "KA5_KonfSommersparSchaltschwelleM2",
    "KA6_KonfiAbsolutSommersparA1M1",
    "KA6_KonfiAbsolutSommersparM2",
    "KA7_KonfiMischersparfunktionM1",
    "KA7_KonfiMischersparfunktionM2",
    "KB0_KonfiRaumaufschaltungA1M1",
    "KB0_KonfiRaumaufschaltungM2",
    "KB2_KonfiRaumeinflussA1M1",
    "KB2_KonfiRaumeinflussM2",
    "KC5_KonfiMinimalbegrenzungA1M1",
    "KC5_KonfiMinimalbegrenzungM2",
    "KC8_KonfiBegrenzungRaumeinflussA1M1",
    "KC8_KonfiBegrenzungRaumeinflussM2",
    "KE2_KonfiRT_KorrekturFernbedienungA1M1",
    "KE2_KonfiRT_KorrekturFernbedienungM2",
    "KF1_KonfiTemperaturprogrammM1",
    "KF1_KonfiTemperaturprogrammM2",
    "KF2_KonfiPartyzeitA1M1",
    "KF2_KonfiPartyzeitM2",
    "KennungCodierkarte",
    "NRF_Bedienanzeigetext_BDE",

    "NRF_Brennstoffverbrauch_Bedien",
    "NRF_DigitalAusgang_KP",
    "NRF_DigitalAusgang_SLP",
    "NRF_DigitalAusgang_SSM",
    "NRF_DigitalAusgang_ZP",
    "NRF_FM_WartungBZ",
    "NRF_FM_WartungZeitinterwall",
    "NRF_HK_AktuelleBetriebsartM2",
    "NRF_HK_Frostgefahr_aktivA1M1",
    "NRF_HK_Frostgefahr_aktivM2",
    "NRF_K06_KonfiKTMax",
    "NRF_K17_KonfiFoerdertechnik",
    "NRF_K18_KonfiEinschubSaugen30",
    "NRF_K1A_KonfiAustragmotorTakt",
    "NRF_K1B_KonfiEinschubzeit",
    "NRF_K1C_KonfiEinschubSchnecke30",
    "NRF_K1D_KonfiEinschubSchnecke100",
    "NRF_K26_KonfiMeldungLagerraumnachfuellen",
    "NRF_K27_KonfiAustragmotorTaktSchnecke",
    "NRF_K30_KonfiEinschaltpunktWABunten",
    "NRF_K31_KonfiAbschaltpunktWABoben",
    "NRF_K32_KonfiUeberwachungsintegralWAB",
    "NRF_K39_KonfiZuschaltintegralschwelleZK",
    "NRF_K3A_KonfiAbschaltintegralschwelleZK",
    "NRF_K3B_KonfiSchalthystereseZK",
    "NRF_K3D_KonfiEinschaltpunktUV",
    "NRF_K3E_KonfiAusschaltpunktUV",
    "NRF_K3F_KonfiSchalthystereseUV",
    "NRF_K54_Solar_Info_Extern",

    "NRF_K66_KonfiVolumenstrom",
    "NRF_K67_KonfiWW_Solltemperatur3",
    "NRF_K68_KonfiKollektormaximaltemperatur",
    "NRF_K6C_KonfiSpeichermaximaltemperatur",
    "NRF_K6D_KonfiFrostschutztemperatur",
    "NRF_K6E_KonfiKollektoranfahrtempMAX",
    "NRF_K6F_KonfiHystKollektoranfahrtemp",
    "NRF_K76_KonfiKommunikationsmodul",
    "NRF_K80_KonfiFehlerDelay",

    "NRF_KB5_KonfiRaumSommersparA1M1",
    "NRF_KB5_KonfiRaumSommersparM2",
    "NRF_KBC_KonfiErzwungeneWaermeaufnahmeA1M1",
    "NRF_KBC_KonfiErzwungeneWaermeaufnahmeM2",
    "NRF_KC3_KonfiLaufzeitHKMischerA1M1",
    "NRF_KC3_KonfiLaufzeitHKMischerM2",
    "NRF_KC6_KonfiVTMaxA1M1",
    "NRF_KC6_KonfiVTMaxM2",
    "NRF_Kaskade_VTSollwert",
    "NRF_Schaltzeiten_FT",
    "NRF_Schaltzeiten_M1",
    "NRF_Schaltzeiten_M1_ZP",
    "NRF_Sparschaltung_AbsSommerspar",

    "NRF_Sparschaltung_BBHFunktion",
    "NRF_Sparschaltung_BBH_HKmischer",
    "NRF_Sparschaltung_BBH_WechselaufTrue",
    "NRF_Sparschaltung_Pumpenstillstand",
    "NRF_Sparschaltung_RaumSommer",
    "NRF_Sparschaltung_rfu",
    "NRF_SystemIdent_SW1",
    "NRF_SystemIdent_SW2",
    "NRF_TemperaturFehler_ATS",
    "NRF_TemperaturFehler_PTSO",
    "NRF_TemperaturFehler_PTSU",
    "NRF_TemperaturFehler_REF",
    "NRF_TemperaturFehler_STS",
    "NRF_TemperaturFehler_STSSOL",
    "NRF_TemperaturFehler_VTSM1",
    "NRF_TemperaturFehler_VTSM2",
    "NRF_Temperaturanstieg_PTSU", // Puffer 3
    "NRF_Temperaturanstieg_STS",
    "NRF_Temperaturanstieg_STSSOL", // Puffer Soll
    "NRF_Temperaturanstieg_VTSM1",
    "NRF_Temperaturanstieg_VTSM2", // Puffer Mittelwert
    "NRF_TiefpassTemperaturwert_ATS",
    "NRF_TiefpassTemperaturwert_PTSO", // Puffer 1 (oben)
    "NRF_TiefpassTemperaturwert_PTSU", // Puffer 2
    "NRF_TiefpassTemperaturwert_STS",
    "NRF_TiefpassTemperaturwert_STSSOL",
    "NRF_Uhrzeit",
    "NRF_WW_SolltemperaturAktuell",

    "SC100_DrehzahlIst",
    "SC100_DrehzahlSoll",
    "SC100_Einschubtemperatur",
    "SC100_EinschubwertIst",
    "SC100_EinschubwertSoll",
    "SC100_Flammtemperatur",
    "SC100_KesselIsttemperatur", // Kesseltemperatur
    "SC100_KesselLeistung",
    "SC100_KesselsollEffektiv",
    "SC100_Lambdasonde",
    "SC100_PositionPrimaerluftklappe",
    "SC100_PositionSekundaerluftklappe",
    "SC100_Prozessflags",
    "SC100_Prozessstatus",
    "SC100_digitale_Ausgaenge1_b1",
    "SC100_digitale_Ausgaenge1_b2",
    "SC100_digitale_Ausgaenge1_b3",
    "SC100_digitale_Ausgaenge1_b4",
    "SC100_digitale_Ausgaenge1_b5",
    "SC100_digitale_Ausgaenge1_b6",
    "SC100_digitale_Ausgaenge1_b7",
    "SC100_digitale_Ausgaenge2_b1",
    "SC100_digitale_Ausgaenge2_b2",
    "SC100_digitale_Ausgaenge2_b3",
    "SC100_digitale_Ausgaenge2_b4",
    "SC100_digitale_Eingaenge1_b1",
    "SC100_digitale_Eingaenge1_b2",
    "SC100_digitale_Eingaenge1_b3",
    "SC100_digitale_Eingaenge1_b4",
    "SC100_digitale_Eingaenge1_b5",
    "SC100_digitale_Eingaenge1_b6",
    "SC100_digitale_Eingaenge1_b7",
    "SC100_digitale_Eingaenge2_b1",
    "SC100_digitale_Eingaenge2_b2",
    "SC100_digitale_Eingaenge2_b3",
    "SC100_digitale_Eingaenge2_b4",
    "SC100_digitale_Eingaenge2_b5",
    "SolarNachlade",
    "SolarPumpe",
    "SolarStunden",
    "SolarWaerme",
    "Solarkollektortemperatur",
    "SystemIdent_GG",
    "SystemIdent_GK",
    "SystemIdent_HX",
    "Temperatur_2_M1",
    "Temperatur_2_M2",
    "VT_SolltemperaturA1M1",
    "VT_SolltemperaturM2",
    "VorlauftemperaturM1",
    "WW_Status_NR2",
    "circulation_pump_status",
    "date_time",
    "device_group",
    "dhw_circulation_pump_switching_times_mode_a1m1",
    "dhw_circulation_pump_switching_times_mode_m2",
    "dhw_switching_times_mode_a1m1",
    "dhw_switching_times_mode_m2",
    "dhw_temperature_desired",
    "dhw_temperature_desired_effective",
    "ecnsysDeviceIdent",
    "ecnsysErrorBuffer",
    "ecnsysEventType~BHKWError",
    "ecnsysEventType~BHKWErrorIndex",
    "ecnsysLONCommunicationModul",
    "ecnsysLONDeviceID",
    "ecnsysVitocomAnlagennummer",
    "economy_mode_a1m1",
    "economy_mode_m2",
    "flow_temperature_actual_m2",
    "frost_risk_a1m1",
    "frost_risk_m2",
    "heating_circuit_operating_mode_effective_a1m1",
    "heating_circuit_operating_mode_effective_m2",
    "heating_circuit_switching_times_a1m1",
    "heating_circuit_switching_times_m2",
    "heating_curve_level_a1m1",
    "heating_curve_level_m2",
    "heating_curve_slope_a1m1",
    "heating_curve_slope_m2",
    "holiday_departure_date_a1m1",
    "holiday_departure_date_m2",
    "holiday_program_a1m1",
    "holiday_program_m2",
    "holiday_return_date_a1m1",
    "holiday_return_date_m2",
    "nviAlarm_Anlagennummer",
    "nviAlarm_Fehlermanager",
    "nviAlarm_Node1",
    "nviAlarm_Veraenderung",
    "nviBoCState_HarteSperre_PM_Boiler1",
    "nviBoCState_HarteSperre_PM_Boiler2",
    "nviBoCState_HarteSperre_PM_Boiler3",
    "nviBoCState_HarteSperre_PM_Boiler4",
    "nviBoCState_Kesselfehler_PM_Boiler1",
    "nviBoCState_Kesselfehler_PM_Boiler2",
    "nviBoCState_Kesselfehler_PM_Boiler3",
    "nviBoCState_Kesselfehler_PM_Boiler4",
    "nviBoCState_LF_Kritisch_PM_Boiler1",
    "nviBoCState_LF_Kritisch_PM_Boiler2",
    "nviBoCState_LF_Kritisch_PM_Boiler3",
    "nviBoCState_LF_Kritisch_PM_Boiler4",
    "nviBoCState_SP_PM_Boiler1",
    "nviBoCState_SP_PM_Boiler2",
    "nviBoCState_SP_PM_Boiler3",
    "nviBoCState_SP_PM_Boiler4",
    "nviBoCState_WeicheSperre_PM_Boiler1",
    "nviBoCState_WeicheSperre_PM_Boiler2",
    "nviBoCState_WeicheSperre_PM_Boiler3",
    "nviBoCState_WeicheSperre_PM_Boiler4",
    "nviConsumerDmd_Attribute1_CFDM",
    "nviConsumerDmd_Attribute2_CFDM",
    "nviProdCState_AbschaltBetrieb_LFDM",
    "nviProdCState_Fehler_LFDM",
    "nviProdCState_HarteSperre_LFDM",
    "nviProdCState_LF_Kritisch_LFDM",
    "nviProdCState_NormalBetrieb_LFDM",
    "nviProdCState_NurWW_LFDM",
    "nviProdCState_SP_LFDM",
    "nviProdCState_Speicher_LFDM",
    "nviProdCState_WeicheSperre_LFDM",
    "nviProdCState_ZentralFerien_LFDM",
    "nviProdCState_ZentralSpeicher_LFDM",
    "nviProdCState_Zentralbed_LFDM",
    "nvoAlarm_Anlagennummer",
    "cnvoAlarm_Fehlermanager",
    "nvoAlarm_Fehlermanager_Teilnehmerliste",

  ];

  // NRF_Brennstoffverbrauch_Bedien






  // dbg!(vcontrol.get("date_time")).unwrap();
  // dbg!(vcontrol.get("system_error_history_0_time")).unwrap();
  // dbg!(vcontrol.get("system_error_history_0")).unwrap();
  // dbg!(vcontrol.get("system_error_history_9")).unwrap();
  //
  // dbg!(vcontrol.get("ecnsysDeviceBoilerSerialNumber")).unwrap();
  // dbg!(vcontrol.get("ecnsysControllerSerialNumber")).unwrap();
  //
  //
  // dbg!(vcontrol.get("Sachnummer_foreign")).unwrap();
  // dbg!(vcontrol.get("Sachnummer_LON")).unwrap();
  // dbg!(vcontrol.get("ecnsysEventType~VCOMLanLinkID")).unwrap();
  //
  // dbg!(vcontrol.get("SachnummerGLP")).unwrap();
  // dbg!(vcontrol.get("NRF_Sachnummer")).unwrap();
  //
  // dbg!(vcontrol.get("nviBoCState_ReturnTSet_PM_Boiler1")).unwrap();
  //
  // dbg!(vcontrol.get("ecnsysDeviceIdent")).unwrap();
  // dbg!(vcontrol.get("ecnsysDeviceIdentF0")).unwrap();
  //
  // dbg!(vcontrol.get("SystemIdent_GKlasse")).unwrap();
  //
  // dbg!(vcontrol.get("SystemIdent_GG")).unwrap();
  // dbg!(vcontrol.get("SystemIdent_GK")).unwrap();
  // dbg!(vcontrol.get("SystemIdent_HX")).unwrap();
  // dbg!(vcontrol.get("device_group")).unwrap();
}
