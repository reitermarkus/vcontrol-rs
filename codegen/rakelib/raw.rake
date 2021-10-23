def event_types(path)
  reader = Nokogiri::XML::Reader(File.open(path))

  reader.map { |node|
    next if node.empty_element?
    next unless node.name == 'EventType'

    node = Nokogiri::XML(node.outer_xml)

    next unless id = node.at('ID')&.text

    [
      strip_address(id),
      node.at('./EventType').children.map { |n|
        value = case name = n.name.underscore
        when 'address', 'vitocom_channel_id'
          n.text.empty? ? nil : Integer(n.text, 16)
        when /^(block|byte|bit)_(length|position)$/
          Integer(n.text)
        when /^conversion_(factor|offset)$/
          v = Float(n.text)
          v == 0.0 ? nil : v
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
          n.text.empty? ? nil : n.text.each_char.each_slice(2).map { |c| Integer(c.join, 16) }
        else
          n.text
        end

        name = name.to_sym

        next if name.in?([:text, :id])
        next if value.nil?

        [name, value]
      }.compact.to_h,
    ]
  }.compact.to_h
end

file EVENT_TYPES_RAW => EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file SYSTEM_EVENT_TYPES_RAW => SYSTEM_EVENT_TYPES_XML do |t|
  File.write t.name, event_types(t.source).to_yaml
end

file DEVICES_RAW => DATAPOINT_TYPES_XML do |t|
  devices = {}

  reader = Nokogiri::XML::Reader(File.open(t.source))

  reader.each do |node|
    if node.name == 'DataPointType'
      node = Nokogiri::XML(node.outer_xml)

      if device = node.at('./DataPointType/ID')&.text&.strip
        id = node.at('./DataPointType/Identification')&.text&.strip
        next if id.nil?

        description = node.at('./DataPointType/Description')&.text&.strip
        event_types = node.at('./DataPointType/EventTypeList')&.text&.strip&.split(';')
        devices[device] = {
          id: id,
          description: description,
          event_types: event_types.map { |event_type| strip_address(event_type) },
        }
      end
    end
  end

  File.write t.name, devices.sort_by_key.to_yaml
end
