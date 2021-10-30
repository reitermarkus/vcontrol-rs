desc 'create cleaned versions for raw YAML files'
task :cleaned => [
  EVENT_TYPES,
  SYSTEM_EVENT_TYPES,
  DATAPOINT_DEFINITIONS,
  DATAPOINT_TYPES,
  DEVICES,
]

file EVENT_TYPES => [DATAPOINT_DEFINITIONS, EVENT_TYPES_RAW] do |t|
  datapoint_definitions, event_types_raw =
    t.sources.map { |source| load_yaml(source) }

  event_types = {}

  datapoint_definitions.fetch('event_types').each do |event_type_id, event_type|
    reverse_event_type_id = EVENT_TYPE_REPLACEMENTS.invert.fetch(event_type_id, event_type_id)
    event_type_raw = event_types_raw.delete(event_type_id) || event_types_raw.delete(reverse_event_type_id)

    event_type_raw['value_list']&.transform_values! { |v|
      VALUE_LIST_FIXES.fetch(v, v)
    }

    ['name', 'description', 'conversion', 'default_value', 'url', 'access_mode'].each do |k|
      raw_value = event_type_raw[k]
      value = event_type.delete(k)

      if value.nil?
        next
      elsif raw_value.nil?
        event_type_raw[k] = value
      elsif raw_value != value
        regex = /@@viessmann(:?\-ess)?\.eventtype(:?\.name)?\.#{Regexp.escape(address)}\.description/

        if raw_value.match?(regex)
          next
        elsif value.match?(regex)
          event_type_raw[k] = value
        else
          raise "#{k} differs: #{raw_value.inspect} != #{value.inspect}"
        end
      end
    end

    raise 'enum_type differs' if event_type.delete('enum_type') != !event_type_raw.fetch('value_list', {}).empty?

    event_types[event_type_id] = event_type_raw
  end

  File.write t.name, event_types.to_yaml
end

file SYSTEM_EVENT_TYPES => [SYSTEM_EVENT_TYPES_RAW, TRANSLATIONS_RAW] do |t|
  syste_event_types_raw, translations_raw = t.sources.map { |source| load_yaml(source) }

  reverse_translations = translations_raw.map { |k, v| [v.fetch('de'), k] }.to_h

  syste_event_types = syste_event_types_raw.map { |k, v|
    v['value_list']&.transform_values! { |v| "@@#{reverse_translations.fetch(v)}" }

    [k, v]
  }.compact.to_h

  File.write t.name, syste_event_types.to_yaml
end

file DATAPOINT_DEFINITIONS => DATAPOINT_DEFINITIONS_RAW do |t|
  datapoint_definitions_raw = load_yaml(t.source)

  datapoints = datapoint_definitions_raw.fetch('datapoints')
  event_types = datapoint_definitions_raw.fetch('event_types')

  datapoints = datapoints.map { |_, v|
    datapoint_type_id = v.delete('address')
    v['event_types'] = v['event_types'].map { |id|
      event_type_id = event_types.fetch(id).fetch('address')
      EVENT_TYPE_REPLACEMENTS.fetch(event_type_id, event_type_id)
    }
    [datapoint_type_id, v]
  }.to_h

  event_types = event_types.map { |_, v|
    event_type_id = v.delete('address')
    [event_type_id, v]
  }.to_h

  EVENT_TYPE_REPLACEMENTS.each do |from, to |
    if event_types.key?(to)
      event_types.delete(from)
    else
      event_types[to] = event_types.delete(from)
    end
  end

  datapoint_definitions = {
    'datapoints' => datapoints,
    'event_types' => event_types,
  }

  File.write t.name, datapoint_definitions.to_yaml
end

file DATAPOINT_TYPES => [DATAPOINT_DEFINITIONS, DATAPOINT_TYPES_RAW] do |t|
  datapoint_definitions, datapoint_types_raw = t.sources.map { |source| load_yaml(source) }

  datapoint_types = datapoint_types_raw.map { |device_id, v|
    # Remove unsupported devices.
    next if device_id == 'ecnStatusDataPoint'
    next if device_id.start_with?('BESS')
    next if device_id.start_with?('CU401B')
    next if device_id == 'EA2'
    next if device_id == 'VCaldens'
    next if device_id == 'VirtualHydraulicCalibration'
    next if device_id == 'puffermgm'
    next if device_id.start_with?('DEKATEL_')
    next if device_id.start_with?('Dekamatik_')
    next if device_id.start_with?('Solartrol_')
    next if device_id.start_with?('GWG_')
    next if device_id.start_with?('MBus')
    next if device_id.start_with?('HV_')
    next if device_id.start_with?('VBlock')
    next if device_id.start_with?('VCOM')
    next if device_id.start_with?('Vitocom')
    next if device_id.start_with?('Vitogate')
    next if device_id.start_with?('WILO')
    next if device_id.start_with?('_VITODATA')

    # TODO: Handle with `identification_extension` etc.
    next if device_id.match?(/_\d+$/)

    v['identification'] = Integer(v.fetch('identification'), 16)
    v['identification_extension'] = Integer(v.fetch('identification_extension', '0'), 16)
    v['identification_extension_till'] = Integer(v.fetch('identification_extension_till', '0'), 16)

    device_id = EVENT_TYPE_REPLACEMENTS.fetch(device_id, device_id)

    [device_id, v]
  }.compact.to_h

  File.write t.name, datapoint_types.to_yaml
end

DUMMY_EVENT_TYPES = ['GWG_Kennung', 'ecnStatusEventType', 'ecnsysEventType~Error', 'ecnsysEventType~ErrorIndex']

file DEVICES => [DATAPOINT_DEFINITIONS, DATAPOINT_TYPES, EVENT_TYPES, SYSTEM_EVENT_TYPES] do |t|
  datapoint_definitions, datapoint_types, event_types, system_event_types = t.sources.map { |source| load_yaml(source) }

  datapoints = datapoint_definitions.delete('datapoints')

  devices = datapoints.map { |device_id, v|
    datapoint_type = datapoint_types[device_id]
    next if datapoint_type.nil?

    v['identification'] = datapoint_type.fetch('identification')
    v['identification_extension'] = datapoint_type.fetch('identification_extension')
    v['identification_extension_till'] = datapoint_type.fetch('identification_extension_till')

    device_event_types = v['event_types'].map { |event_type_id|
      # Remove unneeded/unsupported event types.
      next if event_type_id.start_with?('Node_')
      next if event_type_id.start_with?('nciNet')

      event_type = event_types.fetch(event_type_id)

      [event_type_id, event_type]
    }.compact.to_h

    v['event_types'] = device_event_types.merge(system_event_types).map { |type_id, type|
      fc_read = type['fc_read']
      fc_write = type['fc_write']
      next if fc_read.nil? && fc_write.nil?
      next unless fc_read == 'virtual_read' || fc_write == 'virtual_write'

      type_id
    }.compact

    # Remove devices without any supported event types.
    next if (v['event_types'] - DUMMY_EVENT_TYPES).empty?

    [device_id, v]
  }.compact.to_h

  File.write t.name, devices.sort_by_key.to_yaml
end
