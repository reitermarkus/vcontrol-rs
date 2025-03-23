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
  EVENT_TYPE_NAME_FIXES.fetch(name, name)
    .sub(/\A@@viessmann(-ess)?\.eventtype\.name\.(viessmann\.eventtype\.name\.)?/, '')
end

file SYSTEM_EVENT_TYPES_CLEANED => [SYSTEM_EVENT_TYPES_RAW, TRANSLATIONS_RAW, DATAPOINT_DEFINITIONS_CLEANED] do |t|
  system_event_types_raw, translations_raw, datapoint_definitions_cleaned = t.sources.map { |source| load_json(source) }

  event_type_ids = datapoint_definitions_cleaned.fetch('event_types').map { |_, v| v.fetch('type_id') }

  system_event_types = system_event_types_raw.reduce({}) { |h, (event_type_id, event_type)|
    next h unless event_type_supported?(event_type_id, event_type)
    event_type['type_id'] = event_type_id

    next h if event_type_ids.include?(event_type_id)

    case event_type_id
    when 'ecnsysDeviceIdent'
      event_type['value_type'] = 'DeviceId'
    when 'ecnsysDeviceIdentF0'
      event_type['value_type'] = 'DeviceIdF0'
    when 'ecnsysErrorBuffer', /\AecnsysFehlerhistorie\d+\Z/, 'ecnsysControllerSerialNumber', 'ecnsysDeviceBoilerSerialNumber'
      next h
    end

    event_type['value_list']&.transform_values! { |v| v.delete_prefix('@@') }

    h[event_type_id] = clean_event_type(event_type)
    h
  }

  save_json(t.name, system_event_types)
end

file DATAPOINT_DEFINITIONS_CLEANED => DATAPOINT_DEFINITIONS_RAW do |t|
  datapoint_definitions_raw = load_json(t.source)

  datapoints = datapoint_definitions_raw.fetch('datapoints').transform_keys { Integer(_1) }
  event_types = datapoint_definitions_raw.fetch('event_types').transform_keys { Integer(_1) }
  event_value_types = datapoint_definitions_raw.fetch('event_value_types').transform_keys { Integer(_1) }
  event_type_groups = datapoint_definitions_raw.fetch('event_type_groups').transform_keys { Integer(_1) }
  table_extensions = datapoint_definitions_raw.fetch('table_extensions').transform_keys { Integer(_1) }
  table_extension_values = datapoint_definitions_raw.fetch('table_extension_values').transform_keys { Integer(_1) }

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
        value
      when /^fc_(read|write)$/
        parse_function(value)
      when 'option'
        field_name = 'option_list'
        parse_option_list(value)
      when 'mapping_type'
        mapping_types = table_extension.fetch('options_value').transform_keys { Integer(_1) }
        mapping_type = mapping_types.fetch(value)
        mapping_type unless mapping_type == 'NoMap'
      else
        value
      end

      event_type[field_name] = value unless value.nil?
    when 'ecnEventTypeGroup'
      next unless event_type_group = event_type_groups[id]

      event_type_group[field_name] = value
    else
      raise "Unknown table: #{table_name}"
    end
  end

  datapoints = datapoints.filter_map { |k, v|
    datapoint_type_id = v.delete('address')

    # Remove devices without identification number.
    next unless v.key?('identification')

    # Remove unsupported devices.
    next if datapoint_type_id.start_with?('@@BatteryEnergyStorageSystem.')
    next if datapoint_type_id.start_with?('BESS')
    next if datapoint_type_id.start_with?('DEKATEL')
    next if datapoint_type_id.start_with?('OpenTherm')
    next if datapoint_type_id.start_with?('Vitocom')
    next if datapoint_type_id.start_with?('Vitogate')
    next if datapoint_type_id.start_with?('Vitowin')

    v['event_types'] = v.fetch('event_types').filter_map { |id|
      event_type = event_types.fetch(id)
      event_type_id = map_event_type_name(event_type.fetch('name'))
      next unless event_type_supported?(event_type_id, event_type)
      id
    }
    [datapoint_type_id, v]
  }.sort_by { |key, | key.bytes }.to_h

  event_value_types = event_value_types.reduce({}) { |h, (k, v)|
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
      when 'ecnsysEventType~Error', /\A@@viessmann.eventvaluetype.name.FehlerHisFA[0-9]+\Z/
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

    h[k] = v
    h
  }

  event_types = event_types.reduce({}) { |h, (id, event_type)|
    event_type_id = map_event_type_name(event_type.fetch('name'))
    next h unless event_type_supported?(event_type_id, event_type)
    event_type['type_id'] = event_type_id

    value_types = event_type.delete('value_types')&.reduce({}) { |h, value_type|
      h.deep_merge!(event_value_types.fetch(value_type).deep_dup)
    }

    event_type.merge!(value_types) if value_types

    h[id] = clean_event_type(event_type)
    h
  }

  datapoint_definitions = {
    'datapoints' => datapoints,
    'event_types' => event_types,
    'event_type_groups' => event_type_groups,
  }

  save_json(t.name, datapoint_definitions)
end

def event_type_supported?(type_id, type)
  return false if type_id.start_with?('Node_')
  return false if type_id.start_with?('nciNet')
  return false if type_id.start_with?('Ecotronic_LAN')
  return false if type_id.match?(/\AHO2B_((Dynamic)?IP|LAN|Proxy)/)
  return false if type_id.start_with?('ecnsysEventType~VCOMLan')
  return false if type_id.start_with?('ecnsysLON')
  return false if type_id.start_with?('ecnsysVitocom')
  return false if type_id.start_with?('vcLan')
  return false if type_id.start_with?('vcNotfax')
  return false if type_id.start_with?('vlogVSNotfax')

  return false unless type.key?('address')

  fc_read = type['fc_read']
  fc_write = type['fc_write']

  fc_read == 'virtual_read' || fc_write == 'virtual_write'
end

def clean_event_type(event_type)
  if (block_factor = event_type['block_factor'])
    if block_factor.zero?
      event_type.delete('block_factor')
    elsif event_type['value_type'] == 'CircuitTimes'
      if block_factor == 7
        event_type.delete('block_factor')
      elsif block_factor != 56
        raise "Unsupported block factor #{block_factor} for CircuitTimes: #{event_type}"
      end
    else
      block_length = event_type['block_length']

      unless (block_length % block_factor).zero?
        raise "Block length #{block_length} not divisible by block factor #{block_factor}: #{event_type}"
      end
    end
  end

  event_type.delete('bit_length') if event_type['bit_length']&.zero?
  event_type.delete('conversion_factor') if event_type['conversion_factor']&.zero?
  event_type.delete('conversion_offset') if event_type['conversion_offset']&.zero?

  event_type
end

file DEVICES_CLEANED => [DATAPOINT_DEFINITIONS_CLEANED, SYSTEM_EVENT_TYPES_CLEANED] do |t|
  datapoint_definitions, system_event_types = t.sources.map { |source| load_json(source) }

  datapoints = datapoint_definitions.fetch('datapoints')
  event_types = datapoint_definitions.fetch('event_types').transform_keys { Integer(_1) }

  devices = datapoints.filter_map { |datapoint_type_id, v|
    # Remove devices without any supported event types.
    next if v['event_types'].reject { |id|
      type_id = event_types.fetch(id).fetch('type_id')
      type_id.match?(/\AecnsysEventType~Error(Index)?\Z/)
    }.empty?

    [datapoint_type_id, v]
  }.sort_by { |key, | key.bytes }.to_h

  save_json(t.name, devices)
end

file TRANSLATIONS_CLEANED => [DATAPOINT_DEFINITIONS_RAW, TRANSLATIONS_RAW, REVERSE_TRANSLATIONS_RAW] do |t|
  datapoint_definitions_raw, translations_raw, reverse_translations_raw = t.sources.map { |source| load_json(source) }

  translations_cleaned = translations_raw.reduce({}) { |h, (k, v)|
    h[TRANSLATION_FIXES.fetch(k, k)] = v.fetch('en')
    h
  }

  datapoint_definitions_raw.fetch('event_value_types').each do |_, event_value_type|
    add_missing_enum_replace_value_translations(event_value_type, translations_cleaned, reverse_translations: reverse_translations_raw)
  end

  translations_cleaned = translations_cleaned.sort_by { |key,| key.bytes }.to_h

  save_json(t.name, translations_cleaned)
end
