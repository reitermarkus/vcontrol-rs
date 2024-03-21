use std::{
  io::BufReader,
  process::{Command, Stdio},
};

use codegen::files;

// https://github.com/sarnau/InsideViessmannVitosoft/blob/main/VitosoftSoftware.md
const ENDPOINT: &str = "https://update-vitosoft.viessmann.com/vrimaster/VRIMasterWebService.asmx";

fn main() -> anyhow::Result<()> {
  let mut child = Command::new("curl")
    .arg("-sSfL").arg(ENDPOINT)
    .arg("--header").arg("Content-Type: text/xml; charset=utf-8")
    .arg("--header").arg(r#"SOAPAction: "http://www.e-controlnet.de/services/VRIMasterWebService/CheckSoftwareVersion""#)
    .arg("--data").arg(r#"<?xml version="1.0" encoding="utf-8"?>
      <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
        <soap:Body>
          <CheckSoftwareVersion xmlns="http://www.e-controlnet.de/services/VRIMasterWebService">
            <client>
              <CustomerId>90 Tage Testlizenz</CustomerId>
              <MaintenanceContractEndTime>0001-01-01T00:00:00</MaintenanceContractEndTime>
            </client>
            <licenceInfo>
              <LicenceId>1063</LicenceId>
              <LicenceHash>B25913BD9D042609498C93AC6DA797D8</LicenceHash>
              <CustMajorVersion />
              <CID>008785F19C4F1A485EAFF715026860E2</CID>
            </licenceInfo>
            <softwareVersion>
              <ClientSoftwareVersion>6.1.0.2</ClientSoftwareVersion>
            </softwareVersion>
          </CheckSoftwareVersion>
        </soap:Body>
      </soap:Envelope>
    "#)
    .stdout(Stdio::piped())
    .spawn()?;
  let xml: serde_json::Value = quick_xml::de::from_reader(BufReader::new(child.stdout.take().unwrap()))?;
  child.wait()?;
  let version=xml["Body"]["CheckSoftwareVersionResponse"]["CheckSoftwareVersionResult"]["SoftwareVersion"]["MasterCurrentSoftwareVersion"]
      ["$text"].as_str().unwrap();

  let mut child = Command::new("curl")
    .arg("-sSfL").arg(ENDPOINT)
    .arg("--header").arg("Content-Type: text/xml; charset=utf-8")
    .arg("--data").arg(r#"<?xml version="1.0" encoding="utf-8"?>
      <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
        <soap:Body>
          <RequestDownload xmlns="http://www.e-controlnet.de/services/VRIMasterWebService">
            <client>
              <CustomerId>90 Tage Testlizenz</CustomerId>
              <MaintenanceContractEndTime>0001-01-01T00:00:00</MaintenanceContractEndTime>
            </client>
            <licenceInfo>
              <LicenceId>1063</LicenceId>
              <LicenceHash>B25913BD9D042609498C93AC6DA797D8</LicenceHash>
              <CustMajorVersion />
              <CID>008785F19C4F1A485EAFF715026860E2</CID>
            </licenceInfo>
            <downloadType>Software</downloadType>
          </RequestDownload>
        </soap:Body>
      </soap:Envelope>
    "#)
    .stdout(Stdio::piped())
    .spawn()?;
  let xml: serde_json::Value = quick_xml::de::from_reader(BufReader::new(child.stdout.take().unwrap()))?;
  let token =
    xml["Body"]["RequestDownloadResponse"]["RequestDownloadResult"]["Token"]["Value"]["$text"].as_str().unwrap();

  let mut child = Command::new("curl")
      .arg("-sSfL").arg(ENDPOINT)
      .arg("--header").arg("Content-Type: text/xml; charset=utf-8")
      .arg("--data").arg(format!(r#"<?xml version="1.0" encoding="utf-8"?>
        <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
          <soap:Body>
            <DownloadSoftware xmlns="http://www.e-controlnet.de/services/VRIMasterWebService">
              <token>
                <Value>{token}</Value>
                <CreationTime>2021-01-01T12:00:00.0000000+02:00</CreationTime>
                <ValidTime>2021-01-02T12:00:00.0000000+02:00</ValidTime>
              </token>
              <softwareVersion>
                <ClientRequestSoftwareVersion>{version}</ClientRequestSoftwareVersion>
              </softwareVersion>
            </DownloadSoftware>
          </soap:Body>
        </soap:Envelope>
      "#))
      .stdout(Stdio::piped())
      .spawn()?;
  let xml: serde_json::Value = quick_xml::de::from_reader(BufReader::new(child.stdout.take().unwrap()))?;
  let url = xml["Body"]["DownloadSoftwareResponse"]["DownloadSoftwareResult"]["URL"]["$text"].as_str().unwrap();

  Command::new("curl")
    .arg("-SfL")
    .arg("--continue")
    .arg("-")
    .arg(&url)
    .arg("--output")
    .arg("vitosoft.exe")
    .spawn()?
    .wait()?;

  Command::new("7z").arg("x").arg("vitosoft.exe").args(files::ALL_FILES).arg("-y").spawn()?.wait()?;

  Ok(())
}
