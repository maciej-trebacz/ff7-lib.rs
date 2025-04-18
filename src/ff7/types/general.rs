use serde::Serialize;

#[derive(Serialize)]
pub enum GameModule {
    None = 0,
    Field = 1,
    Battle = 2,
    World = 3,
    Menu = 5,
    Highway = 6,
    Chocobo = 7,
    SnowBoard = 8,
    Condor = 9,
    Submarine = 10,
    Jet = 11,
    ChangeDisc = 12,
    Snowboard2 = 14,
    Quit = 19,
    Start = 20,
    BattleSwirl = 23,
    Ending = 25,
    GameOver = 26,
    Intro = 27,
    Credits = 28,
}

#[derive(Serialize)]
pub struct FF7BasicData {
    pub current_module: u8,
    pub game_moment: u16,
    pub field_id: u16,
    pub field_fps: f64,
    pub field_current_model_id: u16,
    pub battle_fps: f64,
    pub world_fps: f64,
    pub in_game_time: u32,
    pub disc_id: u8,
    pub menu_visibility: u16,
    pub menu_locks: u16,
    pub field_movement_disabled: u8,
    pub field_menu_access_enabled: u8,
    pub party_locking_mask: u16,
    pub party_visibility_mask: u16,
    pub gil: u32,
    pub gp: u16,
    pub battle_count: u16,
    pub battle_escape_count: u16,
    pub field_battle_check: u32,
    pub game_obj_ptr: u32,
    pub battle_swirl_check: u8,
    pub instant_atb_check: u16,
    pub unfocus_patch_check: u8,
    pub ffnx_check: u8,
    pub step_id: u32,
    pub step_offset: u32,
    pub step_fraction: u32,
    pub danger_value: u32,
    pub battle_id: u16,
    pub invincibility_check: u16,
    pub exp_multiplier: u8,
    pub gil_multiplier: u8,
    pub ap_multiplier: u8,
    pub battle_chocobo_rating: u8,
    pub menu_always_enabled: u8,
    pub world_zoom_tilt_enabled: u8,
    pub world_zoom: u16,
    pub world_tilt: u16,
    pub world_speed_multiplier: u8,
    pub party_member_ids: Vec<u8>,
    pub key_items: Vec<u8>,
    pub zolom_coords: u32,
    pub world_map_type: u8,
    pub field_skip_dialogues_check: u8,
    pub field_tmp_vars: Vec<u8>,
    pub battle_queue: Vec<u8>,
    pub manual_slots_check: u8,
    pub slots_active: u8,
    pub formation_idx: u8,
    pub walk_anywhere_check: u8,
} 