require 'backports/2.7.0/enumerable/filter_map'

desc 'create cleaned versions for raw YAML files'
task :cleaned => [
  SYSTEM_EVENT_TYPES_CLEANED,
  DATAPOINT_DEFINITIONS_CLEANED,
  DEVICES_CLEANED,
  TRANSLATIONS_CLEANED,
]

EVENT_TYPE_NAME_FIXES = {
  '@@WW_Temperatur_Mitte_ab_Bit_0' => 'WW_Temperatur_Mitte_ab_Bit_0',
  '@@WW_Temperatur_Mitte_ab_Bit_4' => 'WW_Temperatur_Mitte_ab_Bit_4',
  '@@viessmann.eventvaluetype.name.WPR3_Split.KC0_Main_mode_variant' => '@@viessmann.eventtype.name.WPR3_Split.KC0_Main_mode_variant',
}

EMPTY_VALUE_TRANSLATION = 'viessmann-ess.eventvaluetype.ModulBetriebsart~3'

TRANSLATION_FIXES = {
  'viessmann-ess.eventvaluetype.AnwahlDrsosselklappe~0'                   => 'viessmann-ess.eventvaluetype.AnwahlDrosselklappe~0',
  'viessmann-ess.eventvaluetype.AnwahlDrsosselklappe~1'                   => 'viessmann-ess.eventvaluetype.AnwahlDrosselklappe~1',
  'viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~0'               => 'viessmann.eventvaluetype.WPR3_SGReady_Funktionen~0',
  'viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~1'               => 'viessmann.eventvaluetype.WPR3_SGReady_Funktionen~1',
  'viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~2'               => 'viessmann.eventvaluetype.WPR3_SGReady_Funktionen~2',
  'viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~3'               => 'viessmann.eventvaluetype.WPR3_SGReady_Funktionen~3',
  'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~0' => 'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~0',
  'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~2' => 'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~2',
  'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~3' => 'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~3',
  'viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~0'            => 'viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~0',
  'viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~2'            => 'viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~2',
  'viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~3'            => 'viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~3',
  'viessmann.eventvaluetype.K45_Flagtoindicateoper/shortLWT~2'            => 'viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~2',
  'viessmann.eventvaluetype.K45_Flagtoindicateoper/shortLWT~3'            => 'viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~3',
}

VALUE_LIST_FIXES = {
  'viessmann.eventvaluetype.name.HO2B_Geraetetyp~8'                          => 'viessmann.eventvaluetype.HO2B_Geraetetyp~8',
  'viessmann.eventvaluetype..SC100_SoftwareIndex~14'                         => 'viessmann.eventvaluetype.SC100_SoftwareIndex~14',
  'viessmann.eventvaluetype.name.SR13_FktDrehzahlPumpe~3'                    => 'viessmann.eventvaluetype.SR13_FktDrehzahlPumpe~3',
  'viessmann.eventvaluetype.Vitotwin_Fuehlereingang~15'                      => 'viessmann.eventvaluetype.Vitotwin_Fuehlereingang~3', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~0'    => 'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~0',
  'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper/shortLWT~3'    => 'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~3',
  'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper/shortLWT~2'    => 'viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~2',
  'viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~0'               => 'viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~0',
  'viessmann.eventvaluetype.K45_Flagtoindicateopen_shortIRT~2'               => 'viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~2',
  'viessmann.eventvaluetype.K45_Flagtoindicateopen_shortIRT~3'               => 'viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~3',
  'viessmann.eventvaluetype.name.K4F_Protectionreason_0~4'                   => 'viessmann.eventvaluetype.K4F_Protectionreason_0~4',
  'viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~5'              => 'viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~4', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.KC4_Main_mode_variant_diagnostics~2'  => 'viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~2', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.KC4_Main_modevariant_diagnostics~0'   => 'viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~0', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.KC4_Main_mode_variant_diagnostics~16' => 'viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~3', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.KC4_Main_mode_variant_diagnostics~1'  => 'viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~1', # Translation does not exist.
  'viessmann.eventvaluetype.name.WPR3_Split.KC5_Protection_code~3'           => 'viessmann.eventvaluetype.WPR3_Split.KC5_Protection_code~3',
  'viessmann.eventvaluetype.WPR3_Split.KE1_IFeel_mode_status~1'              => 'viessmann.eventvaluetype.WPR3_Split.KE1_IFeel_mode_status~2', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.KE4_Reset_NLOAD~1'                    => 'viessmann.eventvaluetype.name.WPR3_Split.KE4_Reset_NLOAD~1',
  'viessmann.eventvaluetype.name.WPR3_Split.KEF_On_Off_Status~1'             => 'viessmann.eventvaluetype.WPR3_Split.KEF_On_Off_Status~1',
  'viessmann.eventvaluetype.WPR3_Split.KF1_self_test_jumper_1~1'             => 'viessmann.eventtype.name.WPR3_Split.KF1_self_test_jumper_1~1',
  'viessmann.eventvaluetype.WPR3_Split.KF10_jumper_10~0'                     => 'viessmann.eventvaluetype.WPR3_Split.KF2_jumper_10~0', # Translation does not exist.
  'viessmann.eventvaluetype.WPR3_Split.KF10_jumper_10~1'                     => 'viessmann.eventvaluetype.WPR3_Split.KF2_jumper_10~1', # Translation does not exist.
  'viessmann-ess.eventvaluetype.nciNetConfig~0'                              => 'viessmann.eventvaluetype.nciNetConfig~0',
  'viessmann-ess.eventvaluetype.nciNetConfig~1'                              => 'viessmann.eventvaluetype.nciNetConfig~1',
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~0'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~1'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~2'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~4'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~5'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~6'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~7'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~8'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~9'                      => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~10'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~11'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~12'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~13'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~14'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~15'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-1'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-2'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-3'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-4'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-5'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-6'                     => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-10'                    => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-11'                    => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-12'                    => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-13'                    => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~-1'                 => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~0'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~1'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~2'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~3'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~4'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~5'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~8'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~7'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~9'                  => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~10'                 => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
  'viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~11'                 => EMPTY_VALUE_TRANSLATION, # Translation does not exist.
}

def map_unit(unit)
  case unit.delete_prefix('ecnUnit.')
  when 'Minuten'
    'min'
  when 'Grad C'
    '°C'
  when 'Prozent'
    '%'
  when 'K'
    'K'
  when 'Sekunden', 'Sek.'
    's'
  when 'Stunden'
    'h'
  when 'Prozent pro K'
    '%/K'
  when 'Bar'
    'bar'
  when 'Ohm'
    'Ω'
  when 'K Sec'
    'K/s'
  when 'K Min'
    'K/min'
  when 'K pro h'
    'K/h'
  when 'Monate'
    'mo'
  when 'kW', 'KW', 'kW_10'
    'kW'
  when 'MWh'
    'MWh'
  when 'kWh'
    'kWh'
  when 'l pro min'
    'l/min'
  when 'l pro h'
    'l/h'
  when 'm3 pro h', 'cbm pro h'
    'm³/h'
  when 'm3'
    'm³'
  when 'kWh pro m3'
    'kWh/m³'
  when 'Tage'
    'd'
  when 'Liter'
    'l'
  when 'kg'
    'kg'
  when 'rps'
    'rev/s'
  when 'rps pro s'
    'rev/s²'
  when 'U pro min'
    'rev/min'
  when 'Grad C pro Min'
    '°C/min'
  when 'Tonnen'
    't'
  when 'mBar'
    'mbar'
  when 'dBm'
    'dBm'
  when 'Bar (absolut)'
    'bara'
  when 'ct_pro_kwh'
    'c/kWh'
  when 'g_pro_sec'
    'g/s'
  when 'kg_pro_h'
    'kg/h'
  when 'h'
    'h'
  when 'V'
    'V'
  when 'mV'
    'mV'
  when 'A'
    'A'
  when 'Hz'
    'Hz'
  when 'W'
    'W'
  when 'Wh'
    'Wh'
  when 'VA'
    'VA'
  when 'VAr'
    'VAr'
  when 'Ah'
    'Ah'
  when 'kJ'
    'kJ'
  when 'MJ'
    'MJ'
  when 'GJ'
    'GJ'
  when 'ppm'
    'ppm'
  when 'Minus', 'Pts', 'Pkt', 'sech'
    nil
  else
    raise "Unknown unit: #{unit}"
  end
end

def map_event_type_name(name)
  name = EVENT_TYPE_NAME_FIXES.fetch(name, name)

  name.sub(/\A@@viessmann(-ess)?\.eventtype\.name\./, '')
end

file SYSTEM_EVENT_TYPES_CLEANED => [SYSTEM_EVENT_TYPES_RAW, TRANSLATIONS_RAW] do |t|
  system_event_types_raw, translations_raw = t.sources.map { |source| load_yaml(source) }

  system_event_types = system_event_types_raw.filter_map { |system_event_type_id, system_event_type|
    next unless event_type_supported?(system_event_type_id, system_event_type)

    case system_event_type_id
    when 'ecnsysEventType~ErrorIndex'
      system_event_type['value_type'] = 'ErrorIndex'
    when /\AecnsysFehlerhistorie\d+\Z/
      system_event_type['value_type'] = 'Error'
    end

    system_event_type['value_list']&.transform_values! { |v| v.delete_prefix('@@') }

    [system_event_type_id, system_event_type]
  }.to_h

  File.write t.name, system_event_types.to_yaml
end

file DATAPOINT_DEFINITIONS_CLEANED => DATAPOINT_DEFINITIONS_RAW do |t|
  datapoint_definitions_raw = load_yaml(t.source)

  datapoints = datapoint_definitions_raw.fetch('datapoints')
  event_types = datapoint_definitions_raw.fetch('event_types')
  event_value_types = datapoint_definitions_raw.fetch('event_value_types')
  table_extensions = datapoint_definitions_raw.fetch('table_extensions')
  table_extension_values = datapoint_definitions_raw.fetch('table_extension_values')

  table_extension_values.each do |_, v|
    table_extension = table_extensions.fetch(v.fetch('ref_id'))

    pk = table_extension.fetch('pk_fields').zip(v.fetch('pk_value')).to_h
    id = pk.fetch('id')

    table_name = table_extension.fetch('table_name')
    field_name = table_extension.fetch('field_name')
    value = v.fetch('internal_value')

    case table_name
    when 'ecnDatapointType'
      next unless datapoint = datapoints[id]

      value = case field_name
      when 'identification', 'identification_extension', 'identification_extension_till'
        case value
        when /\A\h{4}\Z/i
          Integer(value, 16)
        else
          nil
        end
      when 'f0', 'f0_till'
        [value].pack('n').unpack('n').first
      when 'options'
        if value == 'undefined'
          nil
        else
          value.underscore
        end
      else
        value
      end

      datapoint[field_name] = value unless value.nil?
    when 'ecnEventType'
      next unless event_type = event_types[id]

      value = case field_name
      when 'address'
        case value
        when /\A0x\h+\Z/
          Integer(value.delete_prefix('0x'), 16)
        else
          nil
        end
      when /^fc_(read|write)$/
        parse_function(value)
      when 'option'
        field_name = 'option_list'
        parse_option_list(value)
      else
        value
      end

      event_type[field_name] = value unless value.nil?
    when 'ecnEventTypeGroup'
      next
    else
      raise "Unknown table: #{table_name}"
    end
  end

  datapoints = datapoints.filter_map { |k, v|
    datapoint_type_id = v.delete('address')

    # Remove devices without identification number.
    next unless v.key?('identification')

    # Remove unsupported devices.
    next if datapoint_type_id.start_with?('BESS')
    next if datapoint_type_id.start_with?('@@BatteryEnergyStorageSystem.')

    v['event_types'] = v.fetch('event_types').filter_map { |id|
      event_type = event_types.fetch(id)
      event_type_id = map_event_type_name(event_type.fetch('name'))
      next unless event_type_supported?(event_type_id, event_type)
      event_type_id
    }
    [datapoint_type_id, v]
  }.to_h

  event_value_types = event_value_types.map { |k, v|
    if unit = v.delete('unit')
      v['unit'] = map_unit(unit)
    end

    v = case data_type = v.fetch('data_type')
    when 'DateTime'
      { 'value_type' => 'DateTime' }
    when 'Binary'
      case v.fetch('name')
      when 'ecnsysEventType~ErrorIndex'
        { 'value_type' => 'ErrorIndex' }
      when 'ecnsysEventType~Error'
        { 'value_type' => 'Error' }
      when 'Mapping~Schaltzeiten'
        { 'value_type' => 'CircuitTimes' }
      else
        { 'value_type' => 'ByteArray' }
      end
    when 'VarChar', 'NText'
      if v.key?('enum_address_value')
        enum_replace_value = v.fetch('enum_replace_value').delete_prefix('@@')
        enum_replace_value = TRANSLATION_FIXES.fetch(enum_replace_value, enum_replace_value)
        value_list = VALUE_LIST_FIXES.fetch(enum_replace_value, enum_replace_value)

        {
          'value_list' => {
            v.fetch('enum_address_value') => value_list
          }
        }
      else
        { 'value_type' => 'String' }
      end
    when 'Int', 'Float', 'Bit'
      data_type = 'Double' if data_type == 'Float'

      {
        'value_type' => data_type,
        'lower_border' => v.delete('lower_border'),
        'upper_border' => v.delete('upper_border'),
        'stepping' => v.delete('stepping'),
        'unit' => v.delete('unit'),
      }.compact
    else
      raise
    end

    [k, v]
  }.to_h

  event_types = event_types.filter_map { |id, event_type|
    event_type_id = map_event_type_name(event_type.fetch('name'))

    # Remove unneeded/unsupported event types.
    next unless event_type_supported?(event_type_id, event_type)

    value_types = event_type.delete('value_types')&.reduce({}) { |h, value_type|
      h.deep_merge!(event_value_types.fetch(value_type).deep_dup)
    }

    event_type.merge!(value_types) if value_types

    event_type.delete('conversion_factor') if event_type['conversion_factor'] == 0.0
    event_type.delete('conversion_offset') if event_type['conversion_offset'] == 0.0

    [event_type_id, event_type]
  }.to_h

  datapoint_definitions = {
    'datapoints' => datapoints,
    'event_types' => event_types,
  }

  File.write t.name, datapoint_definitions.to_yaml
end

DUMMY_EVENT_TYPES = ['GWG_Kennung', 'ecnStatusEventType']

def event_type_supported?(type_id, type)
  return false if type_id.start_with?('Node_')
  return false if type_id.start_with?('nciNet')

  fc_read = type['fc_read']
  fc_write = type['fc_write']

  fc_read == 'virtual_read' || fc_write == 'virtual_write'
end

file DEVICES_CLEANED => [DATAPOINT_DEFINITIONS_CLEANED, SYSTEM_EVENT_TYPES_CLEANED] do |t|
  datapoint_definitions, system_event_types = t.sources.map { |source| load_yaml(source) }

  datapoints = datapoint_definitions.fetch('datapoints')
  event_types = datapoint_definitions.fetch('event_types')

  devices = datapoints.filter_map { |datapoint_type_id, v|
    v['event_types'] += system_event_types.keys

    # Remove devices without any supported event types.
    next if (v['event_types'] - system_event_types.keys - DUMMY_EVENT_TYPES).empty?

    [datapoint_type_id, v]
  }.to_h

  File.write t.name, devices.to_yaml
end

file TRANSLATIONS_CLEANED => [DATAPOINT_DEFINITIONS_RAW, TRANSLATIONS_RAW, REVERSE_TRANSLATIONS_RAW] do |t|
  datapoint_definitions_raw, translations_raw, reverse_translations_raw = t.sources.map { |source| load_yaml(source) }

  translations_cleaned = translations_raw.reduce({}) { |h, (k, v)|
    h[TRANSLATION_FIXES.fetch(k, k)] = v.fetch('en')
    h
  }

  datapoint_definitions_raw.fetch('event_value_types').each do |_, event_value_type|
    add_missing_enum_replace_value_translations(event_value_type, translations_cleaned, reverse_translations: reverse_translations_raw)
  end

  File.write t.name, translations_cleaned.to_yaml
end
