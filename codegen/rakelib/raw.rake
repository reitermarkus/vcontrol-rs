require 'open3'
require 'tempfile'
require 'base64'
require 'stringio'
require 'parallel'
require 'deepsort'
require 'backports/2.7.0/enumerable/filter_map'

require 'pycall/import'
include PyCall::Import
pyimport :io

VITOSOFT_DIR = 'src'
DATAPOINT_DEFINITIONS_XML                         = "#{VITOSOFT_DIR}/DPDefinitions.xml"
SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_XML          = "#{VITOSOFT_DIR}/sysDeviceIdent.xml"
SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_XML = "#{VITOSOFT_DIR}/sysDeviceIdentExt.xml"
SYSTEM_EVENT_TYPES_XML                            = "#{VITOSOFT_DIR}/sysEventType.xml"
TEXT_RESOURCES_DIR                                = "#{VITOSOFT_DIR}"
REVERSE_TRANSLATIONS_RAW                          = 'reverse_translations.raw.yml'

desc 'download program for decoding .NET Remoting Binary Format data'
file 'nrbf.py' do |t|
  sh 'pip3', 'install', 'namedlist'
  sh 'curl', '-sSfL', 'https://github.com/gurnec/Undo_FFG/raw/HEAD/nrbf.py', '-o', t.name
  chmod '+x', t.name
end

task :import_nrbf => 'nrbf.py' do
  PyCall.sys.path.append Dir.pwd
  pyimport :nrbf
end

NRBF_CACHE = Pathname('nrbf-cache.json')

at_exit do
  next unless defined?(@dotnet_decode_cache)

  NRBF_CACHE.write JSON.pretty_generate(@dotnet_decode_cache)
end

def dotnet_decode(base64_string)
  @dotnet_decode_cache ||= NRBF_CACHE.exist? ? JSON.parse(NRBF_CACHE.read) : {}

  if @dotnet_decode_cache.key?(base64_string)
    return @dotnet_decode_cache[base64_string]
  end

  Rake::Task[:import_nrbf].invoke

  binary_string = Base64.strict_decode64(base64_string)
  bytes = io.BytesIO.new(binary_string)
  value = nrbf.read_stream(bytes)

  @dotnet_decode_cache[base64_string] = if value.respond_to?(:__class__)
    case class_name = value.__class__.__name__
    when 'System_Boolean', 'System_Int32', 'System_Double'
      value.m_value
    else
      raise "Uknown class: #{class_name}"
    end
  else
    value
  end
end

desc 'convert XML files to raw YAML files'
multitask :raw => [
  SYSTEM_EVENT_TYPES_RAW,
  SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_RAW,
  SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_RAW,
  DATAPOINT_DEFINITIONS_RAW,
  TRANSLATIONS_RAW,
]

# Remove enum index from text representation if present.
def clean_enum_text(translation_id, index, text)
  index = if match = translation_id&.match(/^(?<name>.*)~(?<index>\d+)$/)
    if match[:name] == 'viessmann.eventvaluetype.K73_KonfiZpumpeIntervallFreigabe'
      # False positive: <index> per hour
      nil
    else
      match[:index]
    end
  else
    index
  end

  if index
    index = Regexp.escape(index)
    text = text.sub(/^(?:#{index}(?::)?\s+|\(0#{index}\))([^\s]+)/, '\1')
  end

  text.strip
end

def value_if_non_empty(node)
  v = node.text.strip
  v.empty? ? nil : v
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

def parse_value_list(text)
  text.split(';').map { |v| v.split('=', 2) }.map { |(k, v)| [Integer(k), clean_enum_text(nil, k, v)] }.to_h
end

def parse_options_value(text)
  text.split(';').map { |v| v.split('=', 2) }.map { |(k, v)| [k, v] }.to_h
end

def parse_option_list(text)
  text.split(';').map(&:underscore)
end

def parse_byte_array(text)
  text.empty? ? nil : text.delete_prefix('0x').each_char.each_slice(2).map { |c| Integer(c.join, 16) }
end

def parse_value(text)
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

def parse_function(text)
  {
    ''                           => nil,
    'undefined'                  => nil,
    'Virtual_MarktManager_READ'  => 'virtual_market_manager_read',
    'Virtual_MarktManager_WRITE' => 'virtual_market_manager_write',
  }.fetch(text, text.underscore)
end

def parse_translation_text(text)
  text.strip
      .gsub(/\s+/, ' ')
      .gsub('##ecnnewline##', "\n")
      .gsub('##ecntab##', "\t")
      .gsub('##ecnsemicolon##', ';')
      .gsub('##nl##', "\n")
      .lines.map(&:strip).join("\n")
end

# Simplify all whitespace, since some translations differ only by whitespace.
def simplify_translation_text(text)
  text.gsub(/\s+/, ' ').strip
end

def parse_description(text, reverse_translations:)
  case v = text.strip
  when ''
    nil
  when /^@@/
    v
  else
    k = simplify_translation_text(v)
    "@@#{reverse_translations.fetch(k)}"
  end
end

def parse_conversion(text)
  @conversion_cache ||= {}

  if conversion = @conversion_cache.key?(text)
    return @conversion_cache.fetch(text)
  end

  conversion = case text
  when 'NoConversion', '', nil, 'GWG_2010_Kennung~0x00F9'
    nil
  else
    text.sub(/(\A|[a-z])Mult([A-Z]|\Z)/, '\1Mul\2')
        .sub(/(\A|[a-z])MBus([A-Z]|\Z)/, '\1Mbus\2')
        .sub(/(\A|[a-z])2([A-Z]|\Z)/, '\1To\2')
        .underscore
  end

  @conversion_cache[text] = conversion
end

def event_types(path, reverse_translations: {})
  document = Nokogiri::XML.parse(File.read(path))
  document.remove_namespaces!

  types = document.xpath('.//EventTypes/EventType').filter_map { |fragment|
    next if fragment.children.empty?

    event_type = fragment.children.filter_map { |n|
      value = case name = n.name.underscore
      when 'id'
        strip_address(n.text.strip)
      when 'active'
        parse_bool(n.text)
      when 'address'
        n.text.empty? ? nil : Integer(n.text, 16) rescue Float(n.text)
      when 'alz'
        name = 'default_value'
        parse_value(n.text.strip)
      when /^(block|byte|bit)_(length|position|factor)$/, 'mapping_type', 'rpc_handler', 'priority'
        Integer(n.text)
      when 'conversion'
        parse_conversion(n.text)
      when /^conversion_(factor|offset)$/
        Float(n.text)
      when /^((lower|upper)_border|stepping)$/
        Float(n.text)
      when 'access_mode'
        n.text.empty? ? nil : n.text.underscore
      when 'description'
        parse_description(n.text, reverse_translations: reverse_translations)
      when 'data_type'
        case value = n.text.underscore
        when 'readonly'
          name = 'access_mode'
          'read'
        when 'dropdown'
          next
        else
          raise "Unknown `data_type` value: #{value}"
        end
      when /^fc_(read|write)$/
        parse_function(n.text)
      when 'option_list'
        parse_option_list(n.text)
      when 'value_list'
        parse_value_list(n.text).transform_values { |v|
          parse_description(v, reverse_translations: reverse_translations)
        }
      when /^prefix_(read|write)$/
        n.text.empty? ? nil : n.text.delete_prefix('0x').each_char.each_slice(2).map { |c| Integer(c.join, 16) }
      else
        value_if_non_empty(n)
      end

      next if value.nil?

      [name, value]
    }.to_h

    [
      event_type.delete('id'),
      event_type,
    ]
  }.to_h

  types
end

file SYSTEM_EVENT_TYPES_RAW => [SYSTEM_EVENT_TYPES_XML, REVERSE_TRANSLATIONS_RAW] do |t|
  system_event_types_xml, reverse_translations_raw = t.sources
  reverse_translations = load_yaml(reverse_translations_raw)
  File.write t.name, event_types(system_event_types_xml, reverse_translations: reverse_translations).to_yaml
end

file SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_RAW => SYSTEM_DEVICE_IDENTIFIER_EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_RAW => SYSTEM_DEVICE_IDENTIFIER_EXTENDED_EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file DATAPOINT_DEFINITIONS_RAW => [DATAPOINT_DEFINITIONS_XML, REVERSE_TRANSLATIONS_RAW] do |t|
  datapoint_definitions_raw, reverse_translations_raw = t.sources

  reverse_translations = load_yaml(reverse_translations_raw)
  document = Nokogiri::XML.parse(File.read(datapoint_definitions_raw))
  document.remove_namespaces!

  dataset = document.at_xpath('.//ImportExportDataHolder/ECNDataSet/diffgram/ECNDataSet')

  definitions = Parallel.map({
    ['ecnVersion', 'versions'] => ->(fragment) {
      next if fragment.children.empty?

      version = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        when 'name'
          n.text.strip.underscore
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      { version.delete('name') => version.fetch('value') }
    },
    ['ecnDatapointType', 'datapoints'] => ->(fragment) {
      next if fragment.children.empty?

      datapoint_type = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'id', 'event_type_id', 'status_event_type_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        when 'description'
          # `name` contains translation ID which is mostly the same as the description.
          nil
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      { datapoint_type.delete('id') => datapoint_type }
    },
    ['ecnDataPointTypeEventTypeLink', 'datapoint_type_event_type_links'] => ->(fragment) {
      next if fragment.children.empty?

      link = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'data_point_type_id', 'event_type_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      { fragment['id'] => link }
    },
    ['ecnEventType', 'event_types'] => ->(fragment) {
      next if fragment.children.empty?

      event_type = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'id', 'priority', 'config_set_id', 'config_set_parameter_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        when 'enum_type', 'filtercriterion', 'reportingcriterion'
          name = name.sub(/(criterion)/, '_\1')
          parse_bool(n.text)
        when 'address'
          strip_address(n.text.strip)
        when 'conversion'
          parse_conversion(n.text)
        when 'default_value'
          parse_value(n.text.strip)
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
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      { event_type.delete('id') => event_type }
    },
    ['ecnEventValueType', 'event_value_types'] => ->(fragment) {
      next if fragment.children.empty?

      event_value_type = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'id', 'enum_address_value', 'status_type_id', 'value_precision', 'length'
          Integer(n.text.strip)
        when 'lower_border', 'upper_border', 'stepping'
          Float(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        when 'description'
          # `name` contains translation ID which is mostly the same as the description.
          nil
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      if event_value_type.key?('enum_replace_value')
        # `name` is mostly useless for enum values.
        event_value_type.delete('name')
      end

      { event_value_type.delete('id') => event_value_type }
    },
    ['ecnEventTypeEventValueTypeLink', 'event_type_event_value_type_links'] => ->(fragment) {
      next if fragment.children.empty?

      link = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'event_type_id', 'event_value_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      { fragment['id'] => link }
    },
    ['ecnTableExtension', 'table_extensions'] => ->(fragment) {
      next if fragment.children.empty?

      table_extension = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'id', 'internal_data_type'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        when 'pk_fields'
          n.text.split(';').map { |f| f.underscore }
        when 'internal_default_value'
          dotnet_decode(n.text)
        when 'options_value'
          parse_options_value(n.text)
        when 'label'
          # Redundant information already contained in `field_name`.
          nil
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      table_name = table_extension.fetch('table_name')
      table_extension['field_name'] = table_extension.delete('field_name').delete_prefix("label.tableextension.#{table_name}.").underscore

      { table_extension.delete('id') => table_extension }
    },
    ['ecnTableExtensionValue', 'table_extension_values'] => ->(fragment) {
      next if fragment.children.empty?

      table_extension_value = fragment.children.filter_map { |n|
        value = case name = n.name.underscore
        when 'id', 'ref_id'
          Integer(n.text.strip)
        when 'company_id'
          assert_company_id(n.text.strip)
          nil
        when 'pk_value'
          n.text.split(';').map { |v| Integer(v) }
        when 'internal_value'
          dotnet_decode(n.text)
        else
          value_if_non_empty(n)
        end

        [name, value] unless value.nil?
      }.to_h

      { table_extension_value.delete('id') => table_extension_value }
    },
  }) { |(tag, key), parse_fragment|
    {
      key => (dataset > tag).reduce({}) { |h, fragment|
        h.merge!(parse_fragment.call(fragment))
      }
    }
  }.reduce({}) { |h, v| h.merge!(v) }

  definitions.delete('datapoint_type_event_type_links').each do |_, link|
    data_point_type = definitions.fetch('datapoints').fetch(link.fetch('data_point_type_id'))
    data_point_type['event_types'] ||= []
    data_point_type['event_types'].push(link.fetch('event_type_id'))
  end

  definitions.delete('event_type_event_value_type_links').each do |_, link|
    event_type = definitions.fetch('event_types').fetch(link.fetch('event_type_id'))
    event_type['value_types'] ||= []
    event_type['value_types'].push(link.fetch('event_value_id'))
  end

  File.write t.name, definitions.to_yaml
end

file TRANSLATIONS_RAW => TEXT_RESOURCES_DIR.to_s do |t|
  text_resources = Pathname(t.source).glob('Textresource_*.xml')

  translations = Parallel.map(text_resources) { |text_resource|
    document = Nokogiri::XML.parse(text_resource.read)
    document.remove_namespaces!

    document = document.at_xpath('.//DocumentElement')
    cultures = document.xpath('.//Cultures/Culture')
    translations = document.xpath('.//TextResources/TextResource')

    languages = cultures.reduce({}) { |h, node|
      id = node.attribute('Id').text
      name = node.attribute('Name').text

      h[id] = name
      h
    }

    translations.reduce({}) { |h, node|
      language_id = node.attribute('CultureId').text
      translation_id = node.attribute('Label').text
      value = parse_translation_text(node.attribute('Value').text)

      value = clean_enum_text(translation_id, nil, value)
      h[translation_id] = { languages.fetch(language_id) => value }
      h
    }
  }.reduce({}) { |h, translations|
    h.deep_merge!(translations)
  }

  File.write t.name, translations.to_yaml
end

file REVERSE_TRANSLATIONS_RAW => TRANSLATIONS_RAW do |t|
  translations_raw = load_yaml(t.source)

  reverse_translations_raw = translations_raw.filter_map { |k, v|
    text = simplify_translation_text(v.fetch('de'))
    next if text.empty?
    [text, k]
  }.to_h

  File.write t.name, reverse_translations_raw.to_yaml
end
