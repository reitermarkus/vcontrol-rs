VITOSOFT_EXE = 'Vitosoft300SID1_Setup.exe'

# https://github.com/sarnau/InsideViessmannVitosoft/blob/main/VitosoftSoftware.md
file VITOSOFT_EXE do |t|
  endpoint = 'https://update-vitosoft.viessmann.com/vrimaster/VRIMasterWebService.asmx'


  out, err, status = Open3.capture3('curl', '-sSfL', endpoint,
    '--header', 'Content-Type: text/xml; charset=utf-8',
    '--header', 'SOAPAction: "http://www.e-controlnet.de/services/VRIMasterWebService/CheckSoftwareVersion"',
    '--data', <<~XML)
      <?xml version="1.0" encoding="utf-8"?>
        <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
          <soap:Body><CheckSoftwareVersion xmlns="http://www.e-controlnet.de/services/VRIMasterWebService">
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
    XML

  xml = Nokogiri::XML(out)
  xml.remove_namespaces!

  version = xml.xpath('/Envelope/Body/CheckSoftwareVersionResponse/CheckSoftwareVersionResult/SoftwareVersion/MasterCurrentSoftwareVersion').text


  out, err, status = Open3.capture3('curl', '-sSfL', endpoint,
    '--header', 'Content-Type: text/xml; charset=utf-8',
    '--data', <<~XML)
      <?xml version="1.0" encoding="utf-8"?>
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
    XML

  xml = Nokogiri::XML(out)
  xml.remove_namespaces!

  token = xml.xpath('/Envelope/Body/RequestDownloadResponse/RequestDownloadResult/Token/Value').text

  out, err, status = Open3.capture3('curl', '-sSfL', endpoint,
    '--header', 'Content-Type: text/xml; charset=utf-8',
    '--data', <<~XML)
      <?xml version="1.0" encoding="utf-8"?>
      <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
        <soap:Body>
          <DownloadSoftware xmlns="http://www.e-controlnet.de/services/VRIMasterWebService">
            <token>
              <Value>#{token}</Value>
              <CreationTime>2021-01-01T12:00:00.0000000+02:00</CreationTime>
              <ValidTime>2021-01-02T12:00:00.0000000+02:00</ValidTime>
            </token>
            <softwareVersion>
              <ClientRequestSoftwareVersion>#{version}</ClientRequestSoftwareVersion>
            </softwareVersion>
          </DownloadSoftware>
        </soap:Body>
      </soap:Envelope>
    XML

  xml = Nokogiri::XML(out)
  xml.remove_namespaces!

  url = xml.xpath('/Envelope/Body/DownloadSoftwareResponse/DownloadSoftwareResult/URL').text

  sh 'curl', '-sSfL', url, '-o', t.name
  touch t.name
end
