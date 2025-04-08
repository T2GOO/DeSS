// UI ------------------------------------------------------

// Main window size
pub const DIM_WINDOW : [f32;2] = [1000f32, 720f32];

// Display name for app view
pub const DIS_APP_NAME : &str = "aDeSS";
pub const DIS_VIEW_DASHBOARD : &str = "Dashboard";
pub const DIS_VIEW_EXPLORER : &str = "Explorer";
pub const DIS_VIEW_SETTINGS : &str = "Settings";
pub const DIS_VIEW_TRANSFERT : &str = "Transfert";

// IPFS config --------------------------------------------

pub const IPFS_CONFIG_PATH_MAIN_SUFF : &str = ".ipfs";
pub const IPFS_CONFIG_PATH_CONFIG_SUFF : &str = ".ipfs/config";
pub const IPFS_CONFIG_PATH_SWARM_SUFF : &str = ".ipfs/swarm.key";

pub const SWARM_KEY_PREFIX : &str = "/key/swarm/psk/1.0.0/\n/base16/\n";
pub const SIZE_SWARM_KEY : usize = 32;