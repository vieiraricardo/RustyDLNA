// UPnP/DLNA protocol messages

use crate::config::*;

/// Extract the ST (Search Target) from an M-SEARCH request
pub fn extract_search_st(request: &str) -> Option<String> {
    request.lines().find(|line| line.starts_with("ST:")).and_then(|line| {
        line.strip_prefix("ST:").map(|st| st.trim().to_string())
    })
}

/// Generate SSDP M-SEARCH response based on the requested ST
/// Returns None if we don't support the requested search target
pub fn ssdp_search_response_for(requested_st: &str) -> Option<String> {
    match requested_st {
        "ssdp:all" | "upnp:rootdevice" | "uuid:4d696e69-444c-164e-9d41-b827eb96c6c2"
        | "urn:schemas-upnp-org:device:MediaServer:1" => {
            Some(format!(
                "HTTP/1.1 200 OK\r\n\
                CACHE-CONTROL: max-age={}\r\n\
                EXT:\r\n\
                LOCATION: http://{}:{}/rootDesc.xml\r\n\
                SERVER: DLNA/1.0 DLNADOC/1.50 UPnP/1.0 {}\r\n\
                ST: {}\r\n\
                USN: {}::{}\r\n\
                \r\n",
                CACHE_MAX_AGE,
                IP_ADDRESS,
                HTTP_PORT,
                SERVER_ID,
                requested_st,
                if requested_st == "upnp:rootdevice" || requested_st.starts_with("uuid:") {
                    format!("uuid:{}", DEVICE_UUID)
                } else {
                    format!("uuid:{}::{}", DEVICE_UUID, requested_st)
                },
                if requested_st.starts_with("uuid:") {
                    requested_st
                } else {
                    requested_st
                }
            ))
        }
        _ => None,
    }
}

/// SSDP NOTIFY messages for different device types
/// Some TVs look for specific types, so we send all three
pub fn ssdp_notify_messages() -> Vec<String> {
    let base = format!(
        "HOST: {}:{}\r\n\
        CACHE-CONTROL: max-age={}\r\n\
        LOCATION: http://{}:{}/rootDesc.xml\r\n\
        SERVER: DLNA/1.0 DLNADOC/1.50 UPnP/1.0 {}\r\n",
        SSDP_MULTICAST_ADDR, SSDP_PORT, CACHE_MAX_AGE, IP_ADDRESS, HTTP_PORT, SERVER_ID
    );

    vec![
        // upnp:rootdevice
        format!(
            "NOTIFY * HTTP/1.1\r\n{}\r\nNT: upnp:rootdevice\r\n\
            NTS: ssdp:alive\r\n\
            USN: uuid:{}::upnp:rootdevice\r\n\r\n",
            base, DEVICE_UUID
        ),
        // uuid
        format!(
            "NOTIFY * HTTP/1.1\r\n{}\r\nNT: uuid:{}\r\n\
            NTS: ssdp:alive\r\n\
            USN: uuid:{}\r\n\r\n",
            base, DEVICE_UUID, DEVICE_UUID
        ),
        // MediaServer device type
        format!(
            "NOTIFY * HTTP/1.1\r\n{}\r\nNT: urn:schemas-upnp-org:device:MediaServer:1\r\n\
            NTS: ssdp:alive\r\n\
            USN: uuid:{}::urn:schemas-upnp-org:device:MediaServer:1\r\n\r\n",
            base, DEVICE_UUID
        ),
    ]
}

/// Root device description XML (rootDesc.xml)
pub fn root_device_xml() -> String {
    format!(
        r#"<?xml version="1.0"?>
<root xmlns="urn:schemas-upnp-org:device-1-0">
    <specVersion><major>1</major><minor>0</minor></specVersion>
    <device>
        <deviceType>urn:schemas-upnp-org:device:MediaServer:1</deviceType>
        <friendlyName>{}</friendlyName>
        <manufacturer>{}</manufacturer>
        <manufacturerURL>http://www.netgear.com/</manufacturerURL>
        <modelDescription>RustyDLNA on Linux</modelDescription>
        <modelName>Windows Media Connect compatible (MiniDLNA)</modelName>
        <modelNumber>1.3.0</modelNumber>
        <modelURL>http://www.netgear.com</modelURL>
        <serialNumber>00000000</serialNumber>
        <UDN>uuid:{}</UDN>
        <dlna:X_DLNADOC xmlns:dlna="urn:schemas-dlna-org:device-1-0">DMS-1.50</dlna:X_DLNADOC>
        <presentationURL>/</presentationURL>
        <iconList>
            <icon><mimetype>image/png</mimetype><width>48</width><height>48</height>
                <depth>24</depth><url>/icons/sm.png</url></icon>
            <icon><mimetype>image/png</mimetype><width>120</width><height>120</height>
                <depth>24</depth><url>/icons/lrg.png</url></icon>
            <icon><mimetype>image/jpeg</mimetype><width>48</width><height>48</height>
                <depth>24</depth><url>/icons/sm.jpg</url></icon>
            <icon><mimetype>image/jpeg</mimetype><width>120</width><height>120</height>
                <depth>24</depth><url>/icons/lrg.jpg</url></icon>
        </iconList>
        <serviceList>
            <service>
                <serviceType>urn:schemas-upnp-org:service:ContentDirectory:1</serviceType>
                <serviceId>urn:upnp-org:serviceId:ContentDirectory</serviceId>
                <controlURL>/ctl/ContentDir</controlURL>
                <eventSubURL>/evt/ContentDir</eventSubURL>
                <SCPDURL>/ContentDir.xml</SCPDURL>
            </service>
            <service>
                <serviceType>urn:schemas-upnp-org:service:ConnectionManager:1</serviceType>
                <serviceId>urn:upnp-org:serviceId:ConnectionManager</serviceId>
                <controlURL>/ctl/ConnectionMgr</controlURL>
                <eventSubURL>/evt/ConnectionMgr</eventSubURL>
                <SCPDURL>/ConnectionMgr.xml</SCPDURL>
            </service>
            <service>
                <serviceType>urn:microsoft.com:service:X_MS_MediaReceiverRegistrar:1</serviceType>
                <serviceId>urn:microsoft.com:serviceId:X_MS_MediaReceiverRegistrar</serviceId>
                <controlURL>/ctl/X_MS_MediaReceiverRegistrar</controlURL>
                <eventSubURL>/evt/X_MS_MediaReceiverRegistrar</eventSubURL>
                <SCPDURL>/X_MS_MediaReceiverRegistrar.xml</SCPDURL>
            </service>
        </serviceList>
    </device>
</root>"#,
        DEVICE_FRIENDLY_NAME, DEVICE_FRIENDLY_NAME, DEVICE_UUID
    )
}

/// ContentDirectory Service Description (SCPD)
pub fn content_dir_scpd() -> &'static str {
    r#"<?xml version="1.0"?><scpd xmlns="urn:schemas-upnp-org:service-1-0">
    <specVersion><major>1</major><minor>0</minor></specVersion>
    <actionList>
        <action>
            <name>GetSearchCapabilities</name>
            <argumentList>
                <argument><name>SearchCaps</name><direction>out</direction>
                    <relatedStateVariable>SearchCapabilities</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>GetSortCapabilities</name>
            <argumentList>
                <argument><name>SortCaps</name><direction>out</direction>
                    <relatedStateVariable>SortCapabilities</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>GetSystemUpdateID</name>
            <argumentList>
                <argument><name>Id</name><direction>out</direction>
                    <relatedStateVariable>SystemUpdateID</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>Browse</name>
            <argumentList>
                <argument><name>ObjectID</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_ObjectID</relatedStateVariable></argument>
                <argument><name>BrowseFlag</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_BrowseFlag</relatedStateVariable></argument>
                <argument><name>Filter</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_Filter</relatedStateVariable></argument>
                <argument><name>StartingIndex</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_Index</relatedStateVariable></argument>
                <argument><name>RequestedCount</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_Count</relatedStateVariable></argument>
                <argument><name>SortCriteria</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_SortCriteria</relatedStateVariable></argument>
                <argument><name>Result</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Result</relatedStateVariable></argument>
                <argument><name>NumberReturned</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Count</relatedStateVariable></argument>
                <argument><name>TotalMatches</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Count</relatedStateVariable></argument>
                <argument><name>UpdateID</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_UpdateID</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>Search</name>
            <argumentList>
                <argument><name>ContainerID</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_ObjectID</relatedStateVariable></argument>
                <argument><name>SearchCriteria</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_SearchCriteria</relatedStateVariable></argument>
                <argument><name>Filter</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_Filter</relatedStateVariable></argument>
                <argument><name>StartingIndex</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_Index</relatedStateVariable></argument>
                <argument><name>RequestedCount</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_Count</relatedStateVariable></argument>
                <argument><name>SortCriteria</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_SortCriteria</relatedStateVariable></argument>
                <argument><name>Result</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Result</relatedStateVariable></argument>
                <argument><name>NumberReturned</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Count</relatedStateVariable></argument>
                <argument><name>TotalMatches</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Count</relatedStateVariable></argument>
                <argument><name>UpdateID</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_UpdateID</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>UpdateObject</name>
            <argumentList>
                <argument><name>ObjectID</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_ObjectID</relatedStateVariable></argument>
                <argument><name>CurrentTagValue</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_TagValueList</relatedStateVariable></argument>
                <argument><name>NewTagValue</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_TagValueList</relatedStateVariable></argument>
            </argumentList>
        </action>
    </actionList>
    <serviceStateTable>
        <stateVariable sendEvents="yes"><name>TransferIDs</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_ObjectID</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_Result</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_SearchCriteria</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_BrowseFlag</name><dataType>string</dataType>
            <allowedValueList><allowedValue>BrowseMetadata</allowedValue><allowedValue>BrowseDirectChildren</allowedValue></allowedValueList>
        </stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_Filter</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_SortCriteria</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_Index</name><dataType>ui4</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_Count</name><dataType>ui4</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_UpdateID</name><dataType>ui4</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_TagValueList</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>SearchCapabilities</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>SortCapabilities</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>SystemUpdateID</name><dataType>ui4</dataType></stateVariable>
    </serviceStateTable>
</scpd>"#
}

/// ConnectionManager Service Description (SCPD)
pub fn connection_mgr_scpd() -> &'static str {
    r#"<?xml version="1.0"?><scpd xmlns="urn:schemas-upnp-org:service-1-0">
    <specVersion><major>1</major><minor>0</minor></specVersion>
    <actionList>
        <action>
            <name>GetProtocolInfo</name>
            <argumentList>
                <argument><name>Source</name><direction>out</direction>
                    <relatedStateVariable>SourceProtocolInfo</relatedStateVariable></argument>
                <argument><name>Sink</name><direction>out</direction>
                    <relatedStateVariable>SinkProtocolInfo</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>GetCurrentConnectionIDs</name>
            <argumentList>
                <argument><name>ConnectionIDs</name><direction>out</direction>
                    <relatedStateVariable>CurrentConnectionIDs</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>GetCurrentConnectionInfo</name>
            <argumentList>
                <argument><name>ConnectionID</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_ConnectionID</relatedStateVariable></argument>
                <argument><name>RcsID</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_RcsID</relatedStateVariable></argument>
                <argument><name>AVTransportID</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_AVTransportID</relatedStateVariable></argument>
                <argument><name>ProtocolInfo</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_ProtocolInfo</relatedStateVariable></argument>
                <argument><name>PeerConnectionManager</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_ConnectionManager</relatedStateVariable></argument>
                <argument><name>PeerConnectionID</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_ConnectionID</relatedStateVariable></argument>
                <argument><name>Direction</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Direction</relatedStateVariable></argument>
                <argument><name>Status</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_ConnectionStatus</relatedStateVariable></argument>
            </argumentList>
        </action>
    </actionList>
    <serviceStateTable>
        <stateVariable sendEvents="yes"><name>SourceProtocolInfo</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>SinkProtocolInfo</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>CurrentConnectionIDs</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_ConnectionStatus</name><dataType>string</dataType>
            <allowedValueList><allowedValue>OK</allowedValue><allowedValue>ContentFormatMismatch</allowedValue>
                <allowedValue>InsufficientBandwidth</allowedValue><allowedValue>UnreliableChannel</allowedValue>
                <allowedValue>Unknown</allowedValue></allowedValueList>
        </stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_ConnectionManager</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_Direction</name><dataType>string</dataType>
            <allowedValueList><allowedValue>Input</allowedValue><allowedValue>Output</allowedValue></allowedValueList>
        </stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_ProtocolInfo</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_ConnectionID</name><dataType>i4</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_AVTransportID</name><dataType>i4</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_RcsID</name><dataType>i4</dataType></stateVariable>
    </serviceStateTable>
</scpd>"#
}

/// Microsoft MediaReceiverRegistrar Service Description (SCPD)
pub fn media_receiver_registrar_scpd() -> &'static str {
    r#"<?xml version="1.0"?><scpd xmlns="urn:schemas-upnp-org:service-1-0">
    <specVersion><major>1</major><minor>0</minor></specVersion>
    <actionList>
        <action>
            <name>IsAuthorized</name>
            <argumentList>
                <argument><name>DeviceID</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_DeviceID</relatedStateVariable></argument>
                <argument><name>Result</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Result</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>IsValidated</name>
            <argumentList>
                <argument><name>DeviceID</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_DeviceID</relatedStateVariable></argument>
                <argument><name>Result</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_Result</relatedStateVariable></argument>
            </argumentList>
        </action>
        <action>
            <name>RegisterDevice</name>
            <argumentList>
                <argument><name>RegistrationReqMsg</name><direction>in</direction>
                    <relatedStateVariable>A_ARG_TYPE_RegistrationReqMsg</relatedStateVariable></argument>
                <argument><name>RegistrationRespMsg</name><direction>out</direction>
                    <relatedStateVariable>A_ARG_TYPE_RegistrationRespMsg</relatedStateVariable></argument>
            </argumentList>
        </action>
    </actionList>
    <serviceStateTable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_DeviceID</name><dataType>string</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_RegistrationReqMsg</name><dataType>bin.base64</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_RegistrationRespMsg</name><dataType>bin.base64</dataType></stateVariable>
        <stateVariable sendEvents="no"><name>A_ARG_TYPE_Result</name><dataType>int</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>AuthorizationDeniedUpdateID</name><dataType>ui4</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>AuthorizationGrantedUpdateID</name><dataType>ui4</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>ValidationRevokedUpdateID</name><dataType>ui4</dataType></stateVariable>
        <stateVariable sendEvents="yes"><name>ValidationSucceededUpdateID</name><dataType>ui4</dataType></stateVariable>
    </serviceStateTable>
</scpd>"#
}

/// GetSortCapabilities SOAP response
pub fn get_sort_capabilities_response() -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
    <s:Body>
        <u:GetSortCapabilitiesResponse xmlns:u="urn:schemas-upnp-org:service:ContentDirectory:1">
            <SortCaps>dc:title,dc:date,upnp:class,upnp:album,upnp:episodeNumber,upnp:originalTrackNumber</SortCaps>
        </u:GetSortCapabilitiesResponse>
    </s:Body>
</s:Envelope>"#
    )
}
