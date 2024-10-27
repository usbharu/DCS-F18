proto = Proto("dcs-bios","DCS-BIOS Protocol")

start_address = ProtoField.new("開始アドレス","dcs-bios.start_address",ftypes.UINT16,nil,base.HEX)
data_length = ProtoField.new("データ長","dcs-bios.data_length",ftypes.UINT16)
data = ProtoField.new("データ","dcs-bios.data",ftypes.BYTES)
data_text = ProtoField.new("テキスト","dcs-bios.data-text",ftypes.STRING)

proto.fields = {start_address,data_length,data,data_text}

function parse(buffer,pinfo,tree)
    pinfo.cols.protocol = "DCS-BIOS"
    local subtree = tree:add(proto,buffer())

    local len = buffer(2,2):le_uint()

    subtree:add_le(start_address,buffer(0,2))
    subtree:add_le(data_length,len)
    subtree:add(data,buffer(4,len))
    subtree:add(data_text,buffer(4,len))
end

function proto.dissector(buffer,pinfo,tree)
    buffer = buffer:range(4)
   while buffer:len() > 0 do
    local len = buffer(2,2):le_uint()
    parse(buffer,pinfo,tree)
    if (buffer:len() - (4+len)) == 0 then
        break
    end
    buffer = buffer:range(4+len)
   end
end
udp_table = DissectorTable.get("udp.port")
udp_table:add(5010,proto)