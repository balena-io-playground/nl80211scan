use neli::neli_enum;

use crate::consts::*;

#[neli_enum(serialized_type = "u16")]
pub enum Nl80211Attr {
    Unspec = NL80211_ATTR_UNSPEC as _,
    Wiphy = NL80211_ATTR_WIPHY as _,
    WiphyName = NL80211_ATTR_WIPHY_NAME as _,
    Ifindex = NL80211_ATTR_IFINDEX as _,
    Ifname = NL80211_ATTR_IFNAME as _,
    Iftype = NL80211_ATTR_IFTYPE as _,
    Mac = NL80211_ATTR_MAC as _,
    KeyData = NL80211_ATTR_KEY_DATA as _,
    KeyIdx = NL80211_ATTR_KEY_IDX as _,
    KeyCipher = NL80211_ATTR_KEY_CIPHER as _,
    KeySeq = NL80211_ATTR_KEY_SEQ as _,
    KeyDefault = NL80211_ATTR_KEY_DEFAULT as _,
    BeaconInterval = NL80211_ATTR_BEACON_INTERVAL as _,
    DtimPeriod = NL80211_ATTR_DTIM_PERIOD as _,
    BeaconHead = NL80211_ATTR_BEACON_HEAD as _,
    BeaconTail = NL80211_ATTR_BEACON_TAIL as _,
    StaAid = NL80211_ATTR_STA_AID as _,
    StaFlags = NL80211_ATTR_STA_FLAGS as _,
    StaListenInterval = NL80211_ATTR_STA_LISTEN_INTERVAL as _,
    StaSupportedRates = NL80211_ATTR_STA_SUPPORTED_RATES as _,
    StaVlan = NL80211_ATTR_STA_VLAN as _,
    StaInfo = NL80211_ATTR_STA_INFO as _,
    WiphyBands = NL80211_ATTR_WIPHY_BANDS as _,
    MntrFlags = NL80211_ATTR_MNTR_FLAGS as _,
    MeshId = NL80211_ATTR_MESH_ID as _,
    StaPlinkAction = NL80211_ATTR_STA_PLINK_ACTION as _,
    MpathNextHop = NL80211_ATTR_MPATH_NEXT_HOP as _,
    MpathInfo = NL80211_ATTR_MPATH_INFO as _,
    BssCtsProt = NL80211_ATTR_BSS_CTS_PROT as _,
    BssShortPreamble = NL80211_ATTR_BSS_SHORT_PREAMBLE as _,
    BssShortSlotTime = NL80211_ATTR_BSS_SHORT_SLOT_TIME as _,
    HtCapability = NL80211_ATTR_HT_CAPABILITY as _,
    SupportedIftypes = NL80211_ATTR_SUPPORTED_IFTYPES as _,
    RegAlpha2 = NL80211_ATTR_REG_ALPHA2 as _,
    RegRules = NL80211_ATTR_REG_RULES as _,
    MeshConfig = NL80211_ATTR_MESH_CONFIG as _,
    BssBasicRates = NL80211_ATTR_BSS_BASIC_RATES as _,
    WiphyTxqParams = NL80211_ATTR_WIPHY_TXQ_PARAMS as _,
    WiphyFreq = NL80211_ATTR_WIPHY_FREQ as _,
    WiphyChannelType = NL80211_ATTR_WIPHY_CHANNEL_TYPE as _,
    KeyDefaultMgmt = NL80211_ATTR_KEY_DEFAULT_MGMT as _,
    MgmtSubtype = NL80211_ATTR_MGMT_SUBTYPE as _,
    Ie = NL80211_ATTR_IE as _,
    MaxNumScanSsids = NL80211_ATTR_MAX_NUM_SCAN_SSIDS as _,
    ScanFrequencies = NL80211_ATTR_SCAN_FREQUENCIES as _,
    ScanSsids = NL80211_ATTR_SCAN_SSIDS as _,
    Generation = NL80211_ATTR_GENERATION as _,
    Bss = NL80211_ATTR_BSS as _,
    RegInitiator = NL80211_ATTR_REG_INITIATOR as _,
    RegType = NL80211_ATTR_REG_TYPE as _,
    SupportedCommands = NL80211_ATTR_SUPPORTED_COMMANDS as _,
    Frame = NL80211_ATTR_FRAME as _,
    Ssid = NL80211_ATTR_SSID as _,
    AuthType = NL80211_ATTR_AUTH_TYPE as _,
    ReasonCode = NL80211_ATTR_REASON_CODE as _,
    KeyType = NL80211_ATTR_KEY_TYPE as _,
    MaxScanIeLen = NL80211_ATTR_MAX_SCAN_IE_LEN as _,
    CipherSuites = NL80211_ATTR_CIPHER_SUITES as _,
    FreqBefore = NL80211_ATTR_FREQ_BEFORE as _,
    FreqAfter = NL80211_ATTR_FREQ_AFTER as _,
    FreqFixed = NL80211_ATTR_FREQ_FIXED as _,
    WiphyRetryShort = NL80211_ATTR_WIPHY_RETRY_SHORT as _,
    WiphyRetryLong = NL80211_ATTR_WIPHY_RETRY_LONG as _,
    WiphyFragThreshold = NL80211_ATTR_WIPHY_FRAG_THRESHOLD as _,
    WiphyRtsThreshold = NL80211_ATTR_WIPHY_RTS_THRESHOLD as _,
    TimedOut = NL80211_ATTR_TIMED_OUT as _,
    UseMfp = NL80211_ATTR_USE_MFP as _,
    StaFlags2 = NL80211_ATTR_STA_FLAGS2 as _,
    ControlPort = NL80211_ATTR_CONTROL_PORT as _,
    Testdata = NL80211_ATTR_TESTDATA as _,
    Privacy = NL80211_ATTR_PRIVACY as _,
    DisconnectedByAp = NL80211_ATTR_DISCONNECTED_BY_AP as _,
    StatusCode = NL80211_ATTR_STATUS_CODE as _,
    CipherSuitesPairwise = NL80211_ATTR_CIPHER_SUITES_PAIRWISE as _,
    CipherSuiteGroup = NL80211_ATTR_CIPHER_SUITE_GROUP as _,
    WpaVersions = NL80211_ATTR_WPA_VERSIONS as _,
    AkmSuites = NL80211_ATTR_AKM_SUITES as _,
    ReqIe = NL80211_ATTR_REQ_IE as _,
    RespIe = NL80211_ATTR_RESP_IE as _,
    PrevBssid = NL80211_ATTR_PREV_BSSID as _,
    Key = NL80211_ATTR_KEY as _,
    Keys = NL80211_ATTR_KEYS as _,
    Pid = NL80211_ATTR_PID as _,
    FourAddr = NL80211_ATTR_4ADDR as _,
    SurveyInfo = NL80211_ATTR_SURVEY_INFO as _,
    Pmkid = NL80211_ATTR_PMKID as _,
    MaxNumPmkids = NL80211_ATTR_MAX_NUM_PMKIDS as _,
    Duration = NL80211_ATTR_DURATION as _,
    Cookie = NL80211_ATTR_COOKIE as _,
    WiphyCoverageClass = NL80211_ATTR_WIPHY_COVERAGE_CLASS as _,
    TxRates = NL80211_ATTR_TX_RATES as _,
    FrameMatch = NL80211_ATTR_FRAME_MATCH as _,
    Ack = NL80211_ATTR_ACK as _,
    PsState = NL80211_ATTR_PS_STATE as _,
    Cqm = NL80211_ATTR_CQM as _,
    LocalStateChange = NL80211_ATTR_LOCAL_STATE_CHANGE as _,
    ApIsolate = NL80211_ATTR_AP_ISOLATE as _,
    WiphyTxPowerSetting = NL80211_ATTR_WIPHY_TX_POWER_SETTING as _,
    WiphyTxPowerLevel = NL80211_ATTR_WIPHY_TX_POWER_LEVEL as _,
    TxFrameTypes = NL80211_ATTR_TX_FRAME_TYPES as _,
    RxFrameTypes = NL80211_ATTR_RX_FRAME_TYPES as _,
    FrameType = NL80211_ATTR_FRAME_TYPE as _,
    ControlPortEthertype = NL80211_ATTR_CONTROL_PORT_ETHERTYPE as _,
    ControlPortNoEncrypt = NL80211_ATTR_CONTROL_PORT_NO_ENCRYPT as _,
    SupportIbssRsn = NL80211_ATTR_SUPPORT_IBSS_RSN as _,
    WiphyAntennaTx = NL80211_ATTR_WIPHY_ANTENNA_TX as _,
    WiphyAntennaRx = NL80211_ATTR_WIPHY_ANTENNA_RX as _,
    McastRate = NL80211_ATTR_MCAST_RATE as _,
    OffchannelTxOk = NL80211_ATTR_OFFCHANNEL_TX_OK as _,
    BssHtOpmode = NL80211_ATTR_BSS_HT_OPMODE as _,
    KeyDefaultTypes = NL80211_ATTR_KEY_DEFAULT_TYPES as _,
    MaxRemainOnChannelDuration = NL80211_ATTR_MAX_REMAIN_ON_CHANNEL_DURATION as _,
    MeshSetup = NL80211_ATTR_MESH_SETUP as _,
    WiphyAntennaAvailTx = NL80211_ATTR_WIPHY_ANTENNA_AVAIL_TX as _,
    WiphyAntennaAvailRx = NL80211_ATTR_WIPHY_ANTENNA_AVAIL_RX as _,
    SupportMeshAuth = NL80211_ATTR_SUPPORT_MESH_AUTH as _,
    StaPlinkState = NL80211_ATTR_STA_PLINK_STATE as _,
    WowlanTriggers = NL80211_ATTR_WOWLAN_TRIGGERS as _,
    WowlanTriggersSupported = NL80211_ATTR_WOWLAN_TRIGGERS_SUPPORTED as _,
    SchedScanInterval = NL80211_ATTR_SCHED_SCAN_INTERVAL as _,
    InterfaceCombinations = NL80211_ATTR_INTERFACE_COMBINATIONS as _,
    SoftwareIftypes = NL80211_ATTR_SOFTWARE_IFTYPES as _,
    RekeyData = NL80211_ATTR_REKEY_DATA as _,
    MaxNumSchedScanSsids = NL80211_ATTR_MAX_NUM_SCHED_SCAN_SSIDS as _,
    MaxSchedScanIeLen = NL80211_ATTR_MAX_SCHED_SCAN_IE_LEN as _,
    ScanSuppRates = NL80211_ATTR_SCAN_SUPP_RATES as _,
    HiddenSsid = NL80211_ATTR_HIDDEN_SSID as _,
    IeProbeResp = NL80211_ATTR_IE_PROBE_RESP as _,
    IeAssocResp = NL80211_ATTR_IE_ASSOC_RESP as _,
    StaWme = NL80211_ATTR_STA_WME as _,
    SupportApUapsd = NL80211_ATTR_SUPPORT_AP_UAPSD as _,
    RoamSupport = NL80211_ATTR_ROAM_SUPPORT as _,
    SchedScanMatch = NL80211_ATTR_SCHED_SCAN_MATCH as _,
    MaxMatchSets = NL80211_ATTR_MAX_MATCH_SETS as _,
    PmksaCandidate = NL80211_ATTR_PMKSA_CANDIDATE as _,
    TxNoCckRate = NL80211_ATTR_TX_NO_CCK_RATE as _,
    TdlsAction = NL80211_ATTR_TDLS_ACTION as _,
    TdlsDialogToken = NL80211_ATTR_TDLS_DIALOG_TOKEN as _,
    TdlsOperation = NL80211_ATTR_TDLS_OPERATION as _,
    TdlsSupport = NL80211_ATTR_TDLS_SUPPORT as _,
    TdlsExternalSetup = NL80211_ATTR_TDLS_EXTERNAL_SETUP as _,
    DeviceApSme = NL80211_ATTR_DEVICE_AP_SME as _,
    DontWaitForAck = NL80211_ATTR_DONT_WAIT_FOR_ACK as _,
    FeatureFlags = NL80211_ATTR_FEATURE_FLAGS as _,
    ProbeRespOffload = NL80211_ATTR_PROBE_RESP_OFFLOAD as _,
    ProbeResp = NL80211_ATTR_PROBE_RESP as _,
    DfsRegion = NL80211_ATTR_DFS_REGION as _,
    DisableHt = NL80211_ATTR_DISABLE_HT as _,
    HtCapabilityMask = NL80211_ATTR_HT_CAPABILITY_MASK as _,
    NoackMap = NL80211_ATTR_NOACK_MAP as _,
    InactivityTimeout = NL80211_ATTR_INACTIVITY_TIMEOUT as _,
    RxSignalDbm = NL80211_ATTR_RX_SIGNAL_DBM as _,
    BgScanPeriod = NL80211_ATTR_BG_SCAN_PERIOD as _,
    Wdev = NL80211_ATTR_WDEV as _,
    UserRegHintType = NL80211_ATTR_USER_REG_HINT_TYPE as _,
    ConnFailedReason = NL80211_ATTR_CONN_FAILED_REASON as _,
    AuthData = NL80211_ATTR_AUTH_DATA as _,
    VhtCapability = NL80211_ATTR_VHT_CAPABILITY as _,
    ScanFlags = NL80211_ATTR_SCAN_FLAGS as _,
    ChannelWidth = NL80211_ATTR_CHANNEL_WIDTH as _,
    CenterFreq1 = NL80211_ATTR_CENTER_FREQ1 as _,
    CenterFreq2 = NL80211_ATTR_CENTER_FREQ2 as _,
    P2pCtwindow = NL80211_ATTR_P2P_CTWINDOW as _,
    P2pOppps = NL80211_ATTR_P2P_OPPPS as _,
    LocalMeshPowerMode = NL80211_ATTR_LOCAL_MESH_POWER_MODE as _,
    AclPolicy = NL80211_ATTR_ACL_POLICY as _,
    MacAddrs = NL80211_ATTR_MAC_ADDRS as _,
    MacAclMax = NL80211_ATTR_MAC_ACL_MAX as _,
    RadarEvent = NL80211_ATTR_RADAR_EVENT as _,
    ExtCapa = NL80211_ATTR_EXT_CAPA as _,
    ExtCapaMask = NL80211_ATTR_EXT_CAPA_MASK as _,
    StaCapability = NL80211_ATTR_STA_CAPABILITY as _,
    StaExtCapability = NL80211_ATTR_STA_EXT_CAPABILITY as _,
    ProtocolFeatures = NL80211_ATTR_PROTOCOL_FEATURES as _,
    SplitWiphyDump = NL80211_ATTR_SPLIT_WIPHY_DUMP as _,
    DisableVht = NL80211_ATTR_DISABLE_VHT as _,
    VhtCapabilityMask = NL80211_ATTR_VHT_CAPABILITY_MASK as _,
    Mdid = NL80211_ATTR_MDID as _,
    IeRic = NL80211_ATTR_IE_RIC as _,
    CritProtId = NL80211_ATTR_CRIT_PROT_ID as _,
    MaxCritProtDuration = NL80211_ATTR_MAX_CRIT_PROT_DURATION as _,
    PeerAid = NL80211_ATTR_PEER_AID as _,
    CoalesceRule = NL80211_ATTR_COALESCE_RULE as _,
    ChSwitchCount = NL80211_ATTR_CH_SWITCH_COUNT as _,
    ChSwitchBlockTx = NL80211_ATTR_CH_SWITCH_BLOCK_TX as _,
    CsaIes = NL80211_ATTR_CSA_IES as _,
    CntdwnOffsBeacon = NL80211_ATTR_CNTDWN_OFFS_BEACON as _,
    CntdwnOffsPresp = NL80211_ATTR_CNTDWN_OFFS_PRESP as _,
    RxmgmtFlags = NL80211_ATTR_RXMGMT_FLAGS as _,
    StaSupportedChannels = NL80211_ATTR_STA_SUPPORTED_CHANNELS as _,
    StaSupportedOperClasses = NL80211_ATTR_STA_SUPPORTED_OPER_CLASSES as _,
    HandleDfs = NL80211_ATTR_HANDLE_DFS as _,
    Support5Mhz = NL80211_ATTR_SUPPORT_5_MHZ as _,
    Support10Mhz = NL80211_ATTR_SUPPORT_10_MHZ as _,
    OpmodeNotif = NL80211_ATTR_OPMODE_NOTIF as _,
    VendorId = NL80211_ATTR_VENDOR_ID as _,
    VendorSubcmd = NL80211_ATTR_VENDOR_SUBCMD as _,
    VendorData = NL80211_ATTR_VENDOR_DATA as _,
    VendorEvents = NL80211_ATTR_VENDOR_EVENTS as _,
    QosMap = NL80211_ATTR_QOS_MAP as _,
    MacHint = NL80211_ATTR_MAC_HINT as _,
    WiphyFreqHint = NL80211_ATTR_WIPHY_FREQ_HINT as _,
    MaxApAssocSta = NL80211_ATTR_MAX_AP_ASSOC_STA as _,
    TdlsPeerCapability = NL80211_ATTR_TDLS_PEER_CAPABILITY as _,
    SocketOwner = NL80211_ATTR_SOCKET_OWNER as _,
    CsaCOffsetsTx = NL80211_ATTR_CSA_C_OFFSETS_TX as _,
    MaxCsaCounters = NL80211_ATTR_MAX_CSA_COUNTERS as _,
    TdlsInitiator = NL80211_ATTR_TDLS_INITIATOR as _,
    UseRrm = NL80211_ATTR_USE_RRM as _,
    WiphyDynAck = NL80211_ATTR_WIPHY_DYN_ACK as _,
    Tsid = NL80211_ATTR_TSID as _,
    UserPrio = NL80211_ATTR_USER_PRIO as _,
    AdmittedTime = NL80211_ATTR_ADMITTED_TIME as _,
    SmpsMode = NL80211_ATTR_SMPS_MODE as _,
    OperClass = NL80211_ATTR_OPER_CLASS as _,
    MacMask = NL80211_ATTR_MAC_MASK as _,
    WiphySelfManagedReg = NL80211_ATTR_WIPHY_SELF_MANAGED_REG as _,
    ExtFeatures = NL80211_ATTR_EXT_FEATURES as _,
    SurveyRadioStats = NL80211_ATTR_SURVEY_RADIO_STATS as _,
    NetnsFd = NL80211_ATTR_NETNS_FD as _,
    SchedScanDelay = NL80211_ATTR_SCHED_SCAN_DELAY as _,
    RegIndoor = NL80211_ATTR_REG_INDOOR as _,
    MaxNumSchedScanPlans = NL80211_ATTR_MAX_NUM_SCHED_SCAN_PLANS as _,
    MaxScanPlanInterval = NL80211_ATTR_MAX_SCAN_PLAN_INTERVAL as _,
    MaxScanPlanIterations = NL80211_ATTR_MAX_SCAN_PLAN_ITERATIONS as _,
    SchedScanPlans = NL80211_ATTR_SCHED_SCAN_PLANS as _,
    Pbss = NL80211_ATTR_PBSS as _,
    BssSelect = NL80211_ATTR_BSS_SELECT as _,
    StaSupportP2pPs = NL80211_ATTR_STA_SUPPORT_P2P_PS as _,
    Pad = NL80211_ATTR_PAD as _,
    IftypeExtCapa = NL80211_ATTR_IFTYPE_EXT_CAPA as _,
    MuMimoGroupData = NL80211_ATTR_MU_MIMO_GROUP_DATA as _,
    MuMimoFollowMacAddr = NL80211_ATTR_MU_MIMO_FOLLOW_MAC_ADDR as _,
    ScanStartTimeTsf = NL80211_ATTR_SCAN_START_TIME_TSF as _,
    ScanStartTimeTsfBssid = NL80211_ATTR_SCAN_START_TIME_TSF_BSSID as _,
    MeasurementDuration = NL80211_ATTR_MEASUREMENT_DURATION as _,
    MeasurementDurationMandatory = NL80211_ATTR_MEASUREMENT_DURATION_MANDATORY as _,
    MeshPeerAid = NL80211_ATTR_MESH_PEER_AID as _,
    NanMasterPref = NL80211_ATTR_NAN_MASTER_PREF as _,
    Bands = NL80211_ATTR_BANDS as _,
    NanFunc = NL80211_ATTR_NAN_FUNC as _,
    NanMatch = NL80211_ATTR_NAN_MATCH as _,
    FilsKek = NL80211_ATTR_FILS_KEK as _,
    FilsNonces = NL80211_ATTR_FILS_NONCES as _,
    MulticastToUnicastEnabled = NL80211_ATTR_MULTICAST_TO_UNICAST_ENABLED as _,
    Bssid = NL80211_ATTR_BSSID as _,
    SchedScanRelativeRssi = NL80211_ATTR_SCHED_SCAN_RELATIVE_RSSI as _,
    SchedScanRssiAdjust = NL80211_ATTR_SCHED_SCAN_RSSI_ADJUST as _,
    TimeoutReason = NL80211_ATTR_TIMEOUT_REASON as _,
    FilsErpUsername = NL80211_ATTR_FILS_ERP_USERNAME as _,
    FilsErpRealm = NL80211_ATTR_FILS_ERP_REALM as _,
    FilsErpNextSeqNum = NL80211_ATTR_FILS_ERP_NEXT_SEQ_NUM as _,
    FilsErpRrk = NL80211_ATTR_FILS_ERP_RRK as _,
    FilsCacheId = NL80211_ATTR_FILS_CACHE_ID as _,
    Pmk = NL80211_ATTR_PMK as _,
    SchedScanMulti = NL80211_ATTR_SCHED_SCAN_MULTI as _,
    SchedScanMaxReqs = NL80211_ATTR_SCHED_SCAN_MAX_REQS as _,
    Want1x4WayHs = NL80211_ATTR_WANT_1X_4WAY_HS as _,
    PmkR0Name = NL80211_ATTR_PMKR0_NAME as _,
    PortAuthorized = NL80211_ATTR_PORT_AUTHORIZED as _,
    ExternalAuthAction = NL80211_ATTR_EXTERNAL_AUTH_ACTION as _,
    ExternalAuthSupport = NL80211_ATTR_EXTERNAL_AUTH_SUPPORT as _,
    Nss = NL80211_ATTR_NSS as _,
    AckSignal = NL80211_ATTR_ACK_SIGNAL as _,
    ControlPortOverNl80211 = NL80211_ATTR_CONTROL_PORT_OVER_NL80211 as _,
    TxqStats = NL80211_ATTR_TXQ_STATS as _,
    TxqLimit = NL80211_ATTR_TXQ_LIMIT as _,
    TxqMemoryLimit = NL80211_ATTR_TXQ_MEMORY_LIMIT as _,
    TxqQuantum = NL80211_ATTR_TXQ_QUANTUM as _,
    HeCapability = NL80211_ATTR_HE_CAPABILITY as _,
    FtmResponder = NL80211_ATTR_FTM_RESPONDER as _,
    FtmResponderStats = NL80211_ATTR_FTM_RESPONDER_STATS as _,
    Timeout = NL80211_ATTR_TIMEOUT as _,
    PeerMeasurements = NL80211_ATTR_PEER_MEASUREMENTS as _,
    AirtimeWeight = NL80211_ATTR_AIRTIME_WEIGHT as _,
    StaTxPowerSetting = NL80211_ATTR_STA_TX_POWER_SETTING as _,
    StaTxPower = NL80211_ATTR_STA_TX_POWER as _,
    SaePassword = NL80211_ATTR_SAE_PASSWORD as _,
    TwtResponder = NL80211_ATTR_TWT_RESPONDER as _,
    HeObssPd = NL80211_ATTR_HE_OBSS_PD as _,
    WiphyEdmgChannels = NL80211_ATTR_WIPHY_EDMG_CHANNELS as _,
    WiphyEdmgBwConfig = NL80211_ATTR_WIPHY_EDMG_BW_CONFIG as _,
    VlanId = NL80211_ATTR_VLAN_ID as _,
    HeBssColor = NL80211_ATTR_HE_BSS_COLOR as _,
    IftypeAkmSuits = NL80211_ATTR_IFTYPE_AKM_SUITES as _,
    TidConfig = NL80211_ATTR_TID_CONFIG as _,
    ControlPortNoPreauth = NL80211_ATTR_CONTROL_PORT_NO_PREAUTH as _,
    PmkLifetime = NL80211_ATTR_PMK_LIFETIME as _,
    PmkReauthThreshold = NL80211_ATTR_PMK_REAUTH_THRESHOLD as _,
    ReceiveMulticast = NL80211_ATTR_RECEIVE_MULTICAST as _,
    WiphyFreqOffset = NL80211_ATTR_WIPHY_FREQ_OFFSET as _,
    CenterFreq1Offset = NL80211_ATTR_CENTER_FREQ1_OFFSET as _,
    ScanFreqKhz = NL80211_ATTR_SCAN_FREQ_KHZ as _,
    He6GhzCapability = NL80211_ATTR_HE_6GHZ_CAPABILITY as _,
    FilsDiscovery = NL80211_ATTR_FILS_DISCOVERY as _,
    UnsolBcastProbeResp = NL80211_ATTR_UNSOL_BCAST_PROBE_RESP as _,
    S1gCapability = NL80211_ATTR_S1G_CAPABILITY as _,
    S1gCapabilityMask = NL80211_ATTR_S1G_CAPABILITY_MASK as _,
    SaePwe = NL80211_ATTR_SAE_PWE as _,
    ReconnectRequested = NL80211_ATTR_RECONNECT_REQUESTED as _,
    SarSpec = NL80211_ATTR_SAR_SPEC as _,
    DisableHe = NL80211_ATTR_DISABLE_HE as _,
}

impl neli::consts::genl::NlAttrType for Nl80211Attr {}

#[neli_enum(serialized_type = "u8")]
pub enum Nl80211Cmd {
    Unspec = 0,
    GetWiphy = 1,
    SetWiphy = 2,
    NewWiphy = 3,
    DelWiphy = 4,
    GetInterface = 5,
    SetInterface = 6,
    NewInterface = 7,
    DelInterface = 8,
    GetKey = 9,
    SetKey = 10,
    NewKey = 11,
    DelKey = 12,
    GetBeacon = 13,
    SetBeacon = 14,
    StartAp = 15,
    NewBeacon = 15,
    StopAp = 16,
    DelBeacon = 16,
    GetStation = 17,
    SetStation = 18,
    NewStation = 19,
    DelStation = 20,
    GetMpath = 21,
    SetMpath = 22,
    NewMpath = 23,
    DelMpath = 24,
    SetBss = 25,
    SetReg = 26,
    ReqSetReg = 27,
    GetMeshConfig = 28,
    SetMeshConfig = 29,
    SetMgmtExtraIe = 30,
    GetReg = 31,
    GetScan = 32,
    TriggerScan = 33,
    NewScanResults = 34,
    ScanAborted = 35,
    RegChange = 36,
    Authenticate = 37,
    Associate = 38,
    Deauthenticate = 39,
    Disassociate = 40,
    MichaelMicFailure = 41,
    RegBeaconHint = 42,
    JoinIbss = 43,
    LeaveIbss = 44,
    Testmode = 45,
    Connect = 46,
    Roam = 47,
    Disconnect = 48,
    SetWiphyNetns = 49,
    GetSurvey = 50,
    NewSurveyResults = 51,
    SetPmksa = 52,
    DelPmksa = 53,
    FlushPmksa = 54,
    RemainOnChannel = 55,
    CancelRemainOnChannel = 56,
    SetTxBitrateMask = 57,
    RegisterFrame = 58,
    RegisterAction = 58,
    Frame = 59,
    Action = 59,
    FrameTxStatus = 60,
    ActionTxStatus = 60,
    SetPowerSave = 61,
    GetPowerSave = 62,
    SetCqm = 63,
    NotifyCqm = 64,
    SetChannel = 65,
    SetWdsPeer = 66,
    FrameWaitCancel = 67,
    JoinMesh = 68,
    LeaveMesh = 69,
    UnprotDeauthenticate = 70,
    UnprotDisassociate = 71,
    NewPeerCandidate = 72,
    GetWowlan = 73,
    SetWowlan = 74,
    StartSchedScan = 75,
    StopSchedScan = 76,
    SchedScanResults = 77,
    SchedScanStopped = 78,
    SetRekeyOffload = 79,
    PmksaCandidate = 80,
    TdlsOper = 81,
    TdlsMgmt = 82,
    UnexpectedFrame = 83,
    ProbeClient = 84,
    RegisterBeacons = 85,
    Unexpected4addrFrame = 86,
    SetNoackMap = 87,
    ChSwitchNotify = 88,
    StartP2pDevice = 89,
    StopP2pDevice = 90,
    ConnFailed = 91,
    SetMcastRate = 92,
    SetMacAcl = 93,
    RadarDetect = 94,
    GetProtocolFeatures = 95,
    UpdateFtIes = 96,
    FtEvent = 97,
    CritProtocolStart = 98,
    CritProtocolStop = 99,
    GetCoalesce = 100,
    SetCoalesce = 101,
    ChannelSwitch = 102,
    Vendor = 103,
    SetQosMap = 104,
    AddTxTs = 105,
    DelTxTs = 106,
    GetMpp = 107,
    JoinOcb = 108,
    LeaveOcb = 109,
    ChSwitchStartedNotify = 110,
    TdlsChannelSwitch = 111,
    TdlsCancelChannelSwitch = 112,
    WiphyRegChange = 113,
    AbortScan = 114,
    StartNan = 115,
    StopNan = 116,
    AddNanFunction = 117,
    DelNanFunction = 118,
    ChangeNanConfig = 119,
    NanMatch = 120,
    SetMulticastToUnicast = 121,
    UpdateConnectParams = 122,
    SetPmk = 123,
    DelPmk = 124,
    PortAuthorized = 125,
    ReloadRegdb = 126,
    ExternalAuth = 127,
    StaOpmodeChanged = 128,
    ControlPortFrame = 129,
    GetFtmResponderStats = 130,
    PeerMeasurementStart = 131,
    PeerMeasurementResult = 132,
    PeerMeasurementComplete = 133,
    NotifyRadar = 134,
    UpdateOweInfo = 135,
    ProbeMeshLink = 136,
    SetTidConfig = 137,
    UnprotBeacon = 138,
    ControlPortFrameTxStatus = 139,
    SetSarSpecs = 140,
}

impl neli::consts::genl::Cmd for Nl80211Cmd {}
