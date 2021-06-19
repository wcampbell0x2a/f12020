use deku::prelude::*;

#[derive(Debug, DekuRead)]
pub struct Packet {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_ident: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
    #[deku(ctx = "*packet_id")]
    packet_type: PacketType,
}

#[derive(Debug, DekuRead)]
#[deku(ctx = "packet_id: u8", id = "packet_id")]
pub enum PacketType {
    #[deku(id = "0")]
    Motion(PacketMotionData),
    #[deku(id = "1")]
    Session(PacketSessionData),
    #[deku(id = "2")]
    LapData(PacketLapData),
    #[deku(id = "3")]
    Event(PacketEvent),
    #[deku(id = "4")]
    Participant(PacketParticipantData),
    #[deku(id = "5")]
    CarSetup(PacketCarSetupData),
    #[deku(id = "6")]
    CarTelemetry(PacketCarTelemetryData),
    #[deku(id = "7")]
    CarStatus(PacketCarStatusData),
    #[deku(id = "8")]
    FinalClassification(PacketFinalClassificationData),
    //LobbyInfo    = 9,
}

#[derive(Debug, DekuRead)]
pub struct PacketMotionData {
    car_motion_data: CarMotionData,
    suspension_position: [f32; 4],
    suspension_velocity: [f32; 4],
    suspension_acceleration: [f32; 4],
    wheel_speed: [f32; 4],
    wheel_slip: [f32; 4],
    local_velocity_x: f32,
    local_velocity_y: f32,
    local_velocity_z: f32,
    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    angular_acceleration_x: f32,
    angular_acceleration_y: f32,
    angular_acceleration_z: f32,
    front_wheel_1s_angle: f32,
}

#[derive(Debug, DekuRead)]
pub struct CarMotionData {
    word_position_x: f32,
    word_position_y: f32,
    word_position_z: f32,
    word_velocity_x: f32,
    word_velocity_y: f32,
    word_velocity_z: f32,
    world_forward_dir_x: i16,
    world_forward_dir_y: i16,
    world_forward_dir_z: i16,
    world_right_dir_x: i16,
    world_right_dir_y: i16,
    world_right_dir_z: i16,
    g_force_lateral: f32,
    g_force_longitudinal: f32,
    g_force_vertical: f32,
    yaw: f32,
    patch: f32,
    roll: f32,
}

#[derive(Debug, DekuRead)]
pub struct MarshallZone {
    zone_start: f32,
    zone_flag: i8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct WeatherForecastSample {
    session_type: u8, // TODO enum
    time_offset: u8,
    weather: u8, // TODO enum
    track_temperature: i8,
    air_temperature: i8,
}

#[derive(Debug, DekuRead)]
pub struct PacketSessionData {
    weather: u8, // TODO enum
    track_temperature: i8,
    air_temperature: i8,
    total_laps: u8,
    track_length: u16,
    session_type: u8, // TODO enum
    track_id: i8,     // TODO enum
    formula: u8,      // TODO enum
    session_time_left: u16,
    session_duration: u16,
    pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    spectator_car_index: u8,
    sli_pro_native_support: u8, // TODO enum
    num_marshal_zones: u8,
    #[deku(count = "num_marshal_zones")]
    marshal_zones: Vec<MarshallZone>,
    safety_car_status: u8, // TODO enum
    network_game: u8,      // TODO enum
    num_weather_forecast_samples: u8,
    #[deku(count = "num_weather_forecast_samples")]
    weather_forcast_samples: Vec<WeatherForecastSample>,
}

#[derive(Debug, DekuRead)]
pub struct LapData {
    last_lap_time: f32,
    current_lap_time: f32,
    sector1_time_inms: u16,
    sector2_time_inms: u16,
    best_lap_time: f32,
    best_lap_num: u8,
    best_lap_sector1_time_in_ms: u16,
    best_lap_sector2_time_in_ms: u16,
    best_lap_sector3_time_in_ms: u16,
    best_overall_sector1_time_in_ms: u16,
    best_overall_sector1_lap_num: u8,
    best_overall_sector2_time_in_ms: u16,
    best_overall_sector2_lap_num: u8,
    best_overall_sector3_time_in_ms: u16,
    best_overall_sector3_lap_num: u8,
    lap_distance: f32,
    total_distance: f32,
    safety_car_delta: f32,
    car_position: u8,
    current_lap_num: u8,
    pit_status: u8,          // TODO enum
    sector: u8,              // TODO enum
    current_lap_invalid: u8, // TODO enum
    penalties: u8,
    grid_position: u8,
    driver_status: u8, // TODO enum
    result_status: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct PacketLapData {
    #[deku(count = "22")]
    lap_data: Vec<LapData>,
}

#[derive(Debug, DekuRead)]
#[deku(type = "[u8; 4]")]
pub enum PacketEvent {
    #[deku(id = "[b'S', b'S', b'T', b'A']")]
    SessionStarted,
    #[deku(id = "[b'S', b'E', b'N', b'D']")]
    SessionEnabled,
    #[deku(id = "[b'F', b'T', b'L', b'P']")]
    FastestLap,
    #[deku(id = "[b'R', b'T', b'M', b'T']")]
    Retirement,
    #[deku(id = "[b'D', b'R', b'S', b'E']")]
    DRSEnabled,
    #[deku(id = "[b'D', b'R', b'S', b'D']")]
    DRSDisabled,
    #[deku(id = "[b'T', b'M', b'P', b'T']")]
    TeamMateInPits,
    #[deku(id = "[b'C', b'H', b'Q', b'F']")]
    ChequeredFlag,
    #[deku(id = "[b'R', b'C', b'W', b'N']")]
    RaceWinner,
    #[deku(id = "[b'P', b'E', b'N', b'A']")]
    PenaltyIssued,
    #[deku(id = "[b'S', b'P', b'T', b'P']")]
    SpeedTrapTriggered,
}

#[derive(Debug, DekuRead)]
pub struct ParticipantData {
    ai_controlled: u8, // TODO bool
    driver_id: u8,     // TODO enum?
    time_id: u8,       // TODO enum?
    race_number: u8,
    nationality: u8,
    #[deku(count = "48")]
    name: Vec<u8>,
    your_telemetry: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct PacketParticipantData {
    num_active_cars: u8,
    #[deku(count = "num_active_cars")]
    participants: Vec<ParticipantData>,
}

#[derive(Debug, DekuRead)]
pub struct CarSetupData {
    front_wing: u8,
    rear_wing: u8,
    on_throttle: u8,
    off_throttle: u8,
    front_camber: f32,
    rear_camber: f32,
    front_toe: f32,
    rear_toe: f32,
    front_suspension: u8,
    rear_suspension: u8,
    front_anti_roll_bar: u8,
    rear_anti_roll_bar: u8,
    front_suspension_height: u8,
    rear_suspension_height: u8,
    break_pressure: u8,
    break_bias: u8,
    rear_left_tyre_pressure: f32,
    rear_right_tyre_pressure: f32,
    front_left_tyre_pressure: f32,
    front_right_tyre_pressure: f32,
    ballast: u8,
    fuel_load: f32,
}

#[derive(Debug, DekuRead)]
pub struct PacketCarSetupData {
    #[deku(count = "22")]
    car_setups: Vec<CarSetupData>,
}

#[derive(Debug, DekuRead)]
pub struct CarTelemetryData {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8, // TODO enum
    engine_rpm: u16,
    drs: u8, // TODO enum
    rev_lights_percent: u8,
    brakes_temperature: [u16; 4],
    tyres_surface_temperature: [u8; 4],
    tyres_inner_temperature: [u8; 4],
    engine_temperature: u16,
    tyres_pressure: [f32; 4],
    surface_type: [u8; 4],
}

#[derive(Debug, DekuRead)]
pub struct PacketCarTelemetryData {
    #[deku(count = "22")]
    car_telemetry_data: Vec<CarTelemetryData>,
    button_status: u32,
    mfd_panel_index: u8,
    mfd_panel_index_secondary_player: u8,
    suggested_gear: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct CarStatusData {
    traction_control: u8,
    anti_lock_brakes: u8,
    fuel_mix: u8,
    front_break_bias: u8,
    pit_limiter_status: u8,
    fuel_in_tank: f32,
    fuel_capacity: f32,
    fuel_remaining_laps: f32,
    max_rpm: u16,
    idle_rpm: u16,
    max_gears: u8,
    drs_allowed: u8,
    drs_activation_distance: u16,
    tyres_wear: [u8; 4],
    actual_tyre_compound: u8, // TODO enum
    visual_tyre_compound: u8, // TODO enum
    tyres_age_laps: u8,
    tyres_damage: [u8; 4],
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    drs_fault: u8, // TODO enum
    engine_damage: u8,
    gear_box_damage: u8,
    vehicle_fia_falgs: u8, // TODO enum
    ers_store_energy: f32,
    ers_deploy_mode: u8,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployted_this_lap: f32,
}

#[derive(Debug, DekuRead)]
pub struct PacketCarStatusData {
    #[deku(count = "22")]
    car_status_data: Vec<CarStatusData>,
}

#[derive(Debug, DekuRead)]
pub struct FinalClassificationData {
    position: u8,
    num_laps: u8,
    grid_position: u8,
    points: u8,
    num_pit_stops: u8,
    result_status: u8, // TODO enum
    best_lap_time: f32,
    total_race_time: f64,
    penalties_time: u8,
    num_penalties: u8,
    num_tyre_stints: u8,
    tyre_stints_actual: [u8; 8],
    tyre_stints_visual: [u8; 8],
}

#[derive(Debug, DekuRead)]
pub struct PacketFinalClassificationData {
    num_cars: u8,
    #[deku(count = "num_cars")]
    classificatin_data: Vec<FinalClassificationData>,
}

#[derive(Debug, DekuRead)]
pub struct LobbyInfoData {
    ai_controlled: u8,
    team_id: u8,
    nationality: u8,
    #[deku(count = "48")]
    name: Vec<u8>,
    ready_status: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct PacketLobbyInfoData {
    num_players: u8,
    #[deku(count = "22")]
    lobby_players: Vec<LobbyInfoData>,
}
