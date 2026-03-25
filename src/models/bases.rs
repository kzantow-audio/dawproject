use crate::repositories::project::{
    ArrangementType, AuPluginType, AudioType, BoolParameterType, BoolPointType,
    BuiltinDeviceType, ChannelType, ClipType, ClipSlotType, ClipsType, CompressorType,
    DeviceParametersElementType, DeviceRoleType, DeviceType, EnumParameterType, EnumPointType,
    EqualizerType, FileReferenceType, IntegerParameterType, LaneType, LanesType, LimiterType,
    MarkerType, MarkersType, MediaFileType, NameableType, NoiseGateType, NotesType,
    ParameterType, PluginType, PointType, RealParameterType, RealPointType, ReferenceableType,
    SceneType, SendType, TimeSignatureParameterType, TimeSignaturePointType, TimeUnitType,
    TimelineType, TrackType, WarpsType,
};

// ---------------------------------------------------------------------------
// NameableTrait — types with name, color, comment
// ---------------------------------------------------------------------------

pub trait NameableTrait {
    fn get_name(&self) -> Option<&str>;
    fn get_color(&self) -> Option<&str>;
    fn get_comment(&self) -> Option<&str>;
}

macro_rules! impl_nameable {
    ($($type:ty),* $(,)?) => { $(
        impl NameableTrait for $type {
            fn get_name(&self) -> Option<&str> { self.name.as_deref() }
            fn get_color(&self) -> Option<&str> { self.color.as_deref() }
            fn get_comment(&self) -> Option<&str> { self.comment.as_deref() }
        }
    )* };
}

impl_nameable!(
    NameableType,
    ClipType,
    MarkerType,
    // Referenceable+
    ReferenceableType,
    SceneType,
    ArrangementType,
    SendType,
    // Parameter hierarchy
    ParameterType,
    RealParameterType,
    IntegerParameterType,
    BoolParameterType,
    EnumParameterType,
    TimeSignatureParameterType,
    // Lane hierarchy
    LaneType,
    TrackType,
    ChannelType,
    // Timeline hierarchy
    TimelineType,
    LanesType,
    NotesType,
    ClipsType,
    ClipSlotType,
    MarkersType,
    WarpsType,
    // MediaFile hierarchy
    MediaFileType,
    AudioType,
    // Device hierarchy
    DeviceType,
    BuiltinDeviceType,
    EqualizerType,
    CompressorType,
    NoiseGateType,
    LimiterType,
    // Plugin hierarchy
    PluginType,
    AuPluginType,
);

// ---------------------------------------------------------------------------
// ReferenceableTrait — types with id
// ---------------------------------------------------------------------------

pub trait ReferenceableTrait: NameableTrait {
    fn get_id(&self) -> Option<&str>;
}

macro_rules! impl_referenceable {
    ($($type:ty),* $(,)?) => { $(
        impl ReferenceableTrait for $type {
            fn get_id(&self) -> Option<&str> { self.id.as_deref() }
        }
    )* };
}

impl_referenceable!(
    ReferenceableType,
    SceneType,
    ArrangementType,
    SendType,
    // Parameter hierarchy
    ParameterType,
    RealParameterType,
    IntegerParameterType,
    BoolParameterType,
    EnumParameterType,
    TimeSignatureParameterType,
    // Lane hierarchy
    LaneType,
    TrackType,
    ChannelType,
    // Timeline hierarchy
    TimelineType,
    LanesType,
    NotesType,
    ClipsType,
    ClipSlotType,
    MarkersType,
    WarpsType,
    // MediaFile hierarchy
    MediaFileType,
    AudioType,
    // Device hierarchy
    DeviceType,
    BuiltinDeviceType,
    EqualizerType,
    CompressorType,
    NoiseGateType,
    LimiterType,
    // Plugin hierarchy
    PluginType,
    AuPluginType,
);

// ---------------------------------------------------------------------------
// ParameterTrait — types with parameter_id
// ---------------------------------------------------------------------------

pub trait ParameterTrait: ReferenceableTrait {
    fn get_parameter_id(&self) -> Option<i32>;
}

macro_rules! impl_parameter {
    ($($type:ty),* $(,)?) => { $(
        impl ParameterTrait for $type {
            fn get_parameter_id(&self) -> Option<i32> { self.parameter_id }
        }
    )* };
}

impl_parameter!(
    ParameterType,
    RealParameterType,
    IntegerParameterType,
    BoolParameterType,
    EnumParameterType,
    TimeSignatureParameterType,
);

// ---------------------------------------------------------------------------
// LaneTrait — Lane, Track, Channel
// ---------------------------------------------------------------------------

pub trait LaneTrait: ReferenceableTrait {}

impl LaneTrait for LaneType {}
impl LaneTrait for TrackType {}
impl LaneTrait for ChannelType {}

// ---------------------------------------------------------------------------
// TimelineTrait — types with time_unit and track reference
// ---------------------------------------------------------------------------

pub trait TimelineTrait: ReferenceableTrait {
    fn get_time_unit(&self) -> Option<&TimeUnitType>;
    fn get_track(&self) -> Option<&str>;
}

macro_rules! impl_timeline {
    ($($type:ty),* $(,)?) => { $(
        impl TimelineTrait for $type {
            fn get_time_unit(&self) -> Option<&TimeUnitType> { self.time_unit.as_ref() }
            fn get_track(&self) -> Option<&str> { self.track.as_deref() }
        }
    )* };
}

impl_timeline!(
    TimelineType,
    LanesType,
    NotesType,
    ClipsType,
    ClipSlotType,
    MarkersType,
    WarpsType,
    MediaFileType,
    AudioType,
);

// ---------------------------------------------------------------------------
// MediaFileTrait — types with file and duration
// ---------------------------------------------------------------------------

pub trait MediaFileTrait: TimelineTrait {
    fn get_file(&self) -> &FileReferenceType;
    fn get_duration(&self) -> f64;
}

macro_rules! impl_media_file {
    ($($type:ty),* $(,)?) => { $(
        impl MediaFileTrait for $type {
            fn get_file(&self) -> &FileReferenceType { &self.file }
            fn get_duration(&self) -> f64 { self.duration }
        }
    )* };
}

impl_media_file!(MediaFileType, AudioType);

// ---------------------------------------------------------------------------
// DeviceTrait — types with device fields
// ---------------------------------------------------------------------------

pub trait DeviceTrait: ReferenceableTrait {
    fn get_parameters(&self) -> Option<&DeviceParametersElementType>;
    fn get_enabled(&self) -> Option<&BoolParameterType>;
    fn get_state(&self) -> Option<&FileReferenceType>;
    fn get_device_id(&self) -> Option<&str>;
    fn get_device_name(&self) -> &str;
    fn get_device_role(&self) -> &DeviceRoleType;
    fn get_device_vendor(&self) -> Option<&str>;
    fn get_loaded(&self) -> Option<bool>;
}

macro_rules! impl_device {
    ($($type:ty),* $(,)?) => { $(
        impl DeviceTrait for $type {
            fn get_parameters(&self) -> Option<&DeviceParametersElementType> { self.parameters.as_ref() }
            fn get_enabled(&self) -> Option<&BoolParameterType> { self.enabled.as_ref() }
            fn get_state(&self) -> Option<&FileReferenceType> { self.state.as_ref() }
            fn get_device_id(&self) -> Option<&str> { self.device_id.as_deref() }
            fn get_device_name(&self) -> &str { &self.device_name }
            fn get_device_role(&self) -> &DeviceRoleType { &self.device_role }
            fn get_device_vendor(&self) -> Option<&str> { self.device_vendor.as_deref() }
            fn get_loaded(&self) -> Option<bool> { self.loaded }
        }
    )* };
}

impl_device!(
    DeviceType,
    BuiltinDeviceType,
    EqualizerType,
    CompressorType,
    NoiseGateType,
    LimiterType,
    PluginType,
    AuPluginType,
);

// ---------------------------------------------------------------------------
// PluginTrait — types with plugin_version
// ---------------------------------------------------------------------------

pub trait PluginTrait: DeviceTrait {
    fn get_plugin_version(&self) -> Option<&str>;
}

macro_rules! impl_plugin {
    ($($type:ty),* $(,)?) => { $(
        impl PluginTrait for $type {
            fn get_plugin_version(&self) -> Option<&str> { self.plugin_version.as_deref() }
        }
    )* };
}

impl_plugin!(PluginType, AuPluginType);

// ---------------------------------------------------------------------------
// BuiltinDeviceTrait — marker for builtin devices
// ---------------------------------------------------------------------------

pub trait BuiltinDeviceTrait: DeviceTrait {}

impl BuiltinDeviceTrait for BuiltinDeviceType {}
impl BuiltinDeviceTrait for EqualizerType {}
impl BuiltinDeviceTrait for CompressorType {}
impl BuiltinDeviceTrait for NoiseGateType {}
impl BuiltinDeviceTrait for LimiterType {}

// ---------------------------------------------------------------------------
// PointTrait — types with time
// ---------------------------------------------------------------------------

pub trait PointTrait {
    fn get_time(&self) -> &str;
}

macro_rules! impl_point {
    ($($type:ty),* $(,)?) => { $(
        impl PointTrait for $type {
            fn get_time(&self) -> &str { &self.time }
        }
    )* };
}

impl_point!(
    PointType,
    BoolPointType,
    EnumPointType,
    RealPointType,
    TimeSignaturePointType,
);
