use cgmath::Vector3;

// Window
pub const DEFAULT_CLEARCOL: [f32;4] = [0.8,0.8,0.95,1.0];
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const MSAA_COUNT: u32 = 4; // crashes on start if this isn't 4

// ColumnGrid
pub const COLUMN_GRID_SIDELEN: u8 = 3;
pub const COLUMN_GRID_Z_OFFSET: f32 = - (COLUMN_GRID_SIDELEN as f32 * 1.1);
pub const COLUMN_GRID_POS: Vector3<f32> = Vector3::new(0.0,0.0,COLUMN_GRID_Z_OFFSET);
pub const COLUMN_GRID_SLIDE_MULTIPLIER: f32 = 0.26; // higher == faster

// Camera
pub const CAMERA_FOV: f32 = 15.0;
pub const CAMERA_DISTANCE: f32 = 15.0 + (COLUMN_GRID_SIDELEN as f32 * 3.0);

// Nets
pub const NET_TEX_INDEX: u32 = 0;
pub const NET_COUNT: usize = 4;
pub const NET_SCALE: f32 = 0.5 / COLUMN_GRID_SIDELEN as f32;
pub const NET_EDGE_THICKNESS: f32 = NET_SCALE / 10.0;
pub const NET_LAYER: u8 = 4;
pub const NET_GAP: f32 = 0.86;
pub const NET_Y_OFFSET: f32 = -0.52;
pub const NET_RANDOM_CHANGES: u32 = 2;

// Buttons
pub const BUTTON_TEX_INDEX: u32 = 1;

// Timer
pub const TIMER_TEX_INDEX: u32 = 0;
pub const TIMER_DEFAULT_MAX: f32 = 480.0; //frames
pub const TIMER_REDUCTION_MULTIPLER: f32 = 0.99;
pub const TIMER_OPACITY_MAX: f32 = 0.5;

// Numbers
pub const NUMBER_TEX_INDEX_START: u32 = 2;

// Animations
pub const ANIM_SLIDE_OUT_LEN_FRAMES: f32 = 20.0;
pub const ANIM_SLIDE_IN_LEN_FRAMES: f32 = 20.0;
pub const ANIM_BETWEEN_SLIDES_FRAMES: f32 = 5.0;
