VITOSOFT_DIR = 'src'
DATAPOINT_DEFINITION_VERSION_XML                  = "#{VITOSOFT_DIR}/ecnVersion.xml"
DATAPOINT_DEFINITIONS_XML                         = "#{VITOSOFT_DIR}/DPDefinitions.xml"
DATAPOINT_TYPES_XML                               = "#{VITOSOFT_DIR}/ecnDataPointType.xml"
EVENT_TYPES_XML                                   = "#{VITOSOFT_DIR}/ecnEventType.xml"
SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_XML          = "#{VITOSOFT_DIR}/sysDeviceIdent.xml"
SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_XML = "#{VITOSOFT_DIR}/sysDeviceIdentExt.xml"
SYSTEM_EVENT_TYPES_XML                            = "#{VITOSOFT_DIR}/sysEventType.xml"
TEXT_RESOURCES_DIR     = "#{VITOSOFT_DIR}"

desc 'convert XML files to raw YAML files'
task :raw => [
  DATAPOINT_DEFINITION_VERSION_RAW,
  EVENT_TYPES_RAW,
  SYSTEM_EVENT_TYPES_RAW,
  SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_RAW,
  SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_RAW,
  DATAPOINT_TYPES_RAW,
  DATAPOINT_DEFINITIONS_RAW,
  TRANSLATIONS_RAW,
]

file DATAPOINT_DEFINITION_VERSION_RAW => DATAPOINT_DEFINITION_VERSION_XML do |t|
  doc = Nokogiri::XML::Document.parse(File.open(t.source))
  doc.remove_namespaces!

  version = doc.at('/IEDataSet/Version/DataPointDefinitionVersion').text

  File.write t.name, version.to_yaml
end

# Only one company ID is used.
def assert_company_id(text)
  raise if Integer(text) != 24
end

def parse_bool(text)
  case text&.strip
  when nil, ''
    nil
  when 'true'
    true
  when 'false'
    false
  else
    raise
  end
end

def parse_byte_array(text)
  text.empty? ? nil : text.delete_prefix('0x').each_char.each_slice(2).map { |c| Integer(c.join, 16) }
end

def parse_default_value(text)
  case text
  when /\A0x\h{2}+\Z/
    parse_byte_array(text)
  when /\A\-?(0|[1-9]\d*)\Z/
    Integer(text)
  when /\A\-?\d+[,.]\d+\Z/
    Float(text.sub(',', '.'))
  when /\A\d{2}.\d{2}.\d{4}\Z/
    text.split('.').reverse.join('-')
  when /\A\d{2}.\d{2}.\d{4} \d{2}:\d{2}:\d{2}\Z/
    date, time = text.split(' ')
    date = date.split('.').reverse.join('-')
    [date, time].join('T')
  when '', '--', 'TBD'
    nil
  else
    text
  end
end

def event_types(path)
  reader = Nokogiri::XML::Reader(File.open(path))

  reader.map { |node|
    next unless node.name == 'EventType'

    fragment = Nokogiri::XML.fragment(node.inner_xml)
    next if fragment.children.empty?

    event_type = fragment.children.map { |n|
      value = case name = n.name.underscore
      when 'id'
        strip_address(n.text.strip)
      when 'active'
        parse_bool(n.text)
      when 'address'
        n.text.empty? ? nil : Integer(n.text, 16) rescue Float(n.text)
      when 'alz'
        name = 'default_value'
        parse_default_value(n.text.strip)
      when /^(block|byte|bit)_(length|position|factor)$/, 'mapping_type', 'rpc_handler', 'priority'
        Integer(n.text)
      when /^conversion_(factor|offset)$/
        Float(n.text)
      when /^((lower|upper)_border|stepping)$/
        Float(n.text)
      when 'access_mode'
        n.text.empty? ? nil : n.text.underscore
      when /^fc_(read|write)$/
        {
          ''                           => nil,
          'undefined'                  => nil,
          'Virtual_MarktManager_READ'  => 'virtual_market_manager_read',
          'Virtual_MarktManager_WRITE' => 'virtual_market_manager_write',
        }.fetch(n.text, n.text.underscore)
      when 'option_list'
        n.text.split(';')
      when 'value_list'
        n.text.split(';').map { |v| v.split('=', 2) }.map { |(k, v)| [k.to_i, clean_enum_text(k, v)] }.to_h
      when /^prefix_(read|write)$/
        n.text.empty? ? nil : n.text.delete_prefix('0x').each_char.each_slice(2).map { |c| Integer(c.join, 16) }
      else
        v = n.text.strip
        v.empty? ? nil : v
      end

      [name, value]
    }.compact.to_h

    [
      event_type.fetch('id'),
      event_type,
    ]
  }.compact.to_h
end

file EVENT_TYPES_RAW => EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file SYSTEM_EVENT_TYPES_RAW => SYSTEM_EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_RAW => SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_RAW => SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file DATAPOINT_TYPES_RAW => DATAPOINT_TYPES_XML do |t|
  reader = Nokogiri::XML::Reader(File.open(t.source))
  datapoint_types = reader.map { |node|
    next unless node.name == 'DataPointType'

    fragment = Nokogiri::XML.fragment(node.inner_xml)
    next if fragment.children.empty?

    datapoint_type = fragment.children.map { |n|
      value = case name = n.name.underscore
      when 'controller_type', 'error_type', 'event_optimisation'
        Integer(n.text)
      when 'options'
        {
          'undefined' => nil,
        }.fetch(n.text, n.text)
      else
        v = n.text.strip
        v.empty? ? nil : v
      end

      [name, value]
    }.compact.to_h

    [
      datapoint_type.fetch('id'),
      datapoint_type,
    ]
  }.compact.to_h

  File.write t.name, datapoint_types.sort_by_key.to_yaml
end

file DATAPOINT_DEFINITIONS_RAW => DATAPOINT_DEFINITIONS_XML do |t|
  datapoint_definitions = {}
  event_type_definitions = {}

  reader = Nokogiri::XML::Reader(File.open(t.source))
  reader.each do |node|
    case node.name
    when 'ecnDatapointType'
      fragment = Nokogiri::XML.fragment(node.inner_xml)
      next if fragment.children.empty?

      datapoint_type = fragment.children.map do |n|
        value = case name = n.name.underscore
        when 'id', 'event_type_id', 'status_event_type_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          next
        else
          v = n.text.strip
          v.empty? ? nil : v
        end

        [name, value]
      end.compact.to_h

      datapoint_definitions[datapoint_type.fetch('id')] = datapoint_type
    when 'ecnDataPointTypeEventTypeLink'
      fragment = Nokogiri::XML.fragment(node.inner_xml)
      next if fragment.children.empty?

      link = fragment.children.map do |n|
        value = case name = n.name.underscore
        when 'data_point_type_id', 'event_type_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          next
        else
          v = n.text.strip
          v.empty? ? nil : v
        end

        [name, value]
      end.compact.to_h

      data_point_type = datapoint_definitions.fetch(link.fetch('data_point_type_id'))
      data_point_type['event_types'] ||= []
      data_point_type['event_types'].push(link.fetch('event_type_id'))
      data_point_type['event_types'].sort!
    when 'ecnEventType'
      fragment = Nokogiri::XML.fragment(node.inner_xml)
      next if fragment.children.empty?

      event_type = fragment.children.map do |n|
        value = case name = n.name.underscore
        when 'id', 'priority', 'config_set_id', 'config_set_parameter_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          next
        when 'enum_type', 'filtercriterion', 'reportingcriterion'
          parse_bool(n.text)
        when 'address'
          strip_address(n.text.strip)
        when 'conversion'
          n.text.strip
        when 'default_value'
          parse_default_value(n.text.strip)
        when 'type'
          name = 'access_mode'
          case Integer(n.text.strip)
          when 1
            'read'
          when 2
            'write'
          when 3
            'read_write'
          else
            raise
          end
        else
          v = n.text.strip
          v.empty? ? nil : v
        end

        [name, value]
      end.compact.to_h

      event_type_definitions[event_type.fetch('id')] = event_type
    end
  end

  definitions = {
    'datapoints' => datapoint_definitions,
    'event_types' => event_type_definitions,
  }

  File.write t.name, definitions.sort_by_key.to_yaml
end

file TRANSLATIONS_RAW => TEXT_RESOURCES_DIR.to_s do |t|
  languages = {}
  translations = {}

  Pathname(t.source).glob('Textresource_*.xml').each do |text_resource|
    reader = Nokogiri::XML::Reader(text_resource.open)

    reader.each do |node|
      case node.name
      when 'Culture'
        id = node.attribute('Id')
        name = node.attribute('Name')

        languages[id] ||= name
      when 'TextResource'
        language_id = node.attribute('CultureId')
        label = node.attribute('Label')
        value = node.attribute('Value').strip.gsub('##ecnnewline##', "\n").gsub('##ecntab##', "\t").gsub('##ecnsemicolon##', ';').gsub('##nl##', "\n")

        if /~(?<index>\d+)$/ =~ label
          value = clean_enum_text(index, value)
        end

        translations[label] ||= {}
        translations[label][languages.fetch(language_id)] = value
      end
    end
  end

  File.write t.name, translations.sort_by_key.to_yaml
end
