use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};

use mint::Vector3;
use rand::{Rng, rngs::ThreadRng};

use crate::utils::mouse::{MOUSE_LOCKED, click_mouse, press_mouse, release_mouse};

use crate::cheat::functions::{WeaponType, calculate_distance, is_feature_toggled};
use crate::utils::cheat::config::{CONFIG, Config, TriggerbotConfig, TriggerbotConfigs};

pub static FEATURE_TOGGLED: LazyLock<Arc<Mutex<bool>>> =
    LazyLock::new(|| Arc::new(Mutex::new(CONFIG.lock().unwrap().triggerbot.default)));
pub static TOGGLE_CHANGED: LazyLock<Arc<Mutex<Instant>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Instant::now())));
pub static TB_SHOT_ENTITY: LazyLock<Arc<Mutex<Instant>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Instant::now())));
pub static TB_LOCKED_ENTITY: LazyLock<Arc<Mutex<Option<(Instant, u64)>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));
pub static TB_OFF_ENTITY: LazyLock<Arc<Mutex<Option<Instant>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn get_triggerbot_toggled(config: Config) -> bool {
    let feature = config.triggerbot;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed)
}

pub fn get_triggerbot_config(
    configs: TriggerbotConfigs,
    weapon_type: WeaponType,
) -> TriggerbotConfig {
    match weapon_type {
        WeaponType::Pistol => configs.pistol,
        WeaponType::Rifle => configs.rifle,
        WeaponType::Submachine => configs.submachine,
        WeaponType::Sniper => configs.sniper,
        WeaponType::Shotgun => configs.shotgun,
        WeaponType::MachineGun => configs.machinegun,
        WeaponType::Knife => configs.knife,
        _ => configs.other,
    }
}

pub fn run_triggerbot(
    address: u64,
    config: TriggerbotConfig,
    position: Vector3<f32>,
    local_position: Vector3<f32>,
    rng: &mut ThreadRng,
) {
    let mouse_locked = *MOUSE_LOCKED.lock().unwrap();
    let mut shot_entity = TB_SHOT_ENTITY.lock().unwrap();
    let mut locked_entity = TB_LOCKED_ENTITY.lock().unwrap();

    let distance = calculate_distance(position, local_position);

    if config.min_distance_enabled && distance < config.min_distance
        || config.max_distance_enabled && distance > config.max_distance
    {
        *locked_entity = None;
        release_mouse();
        return;
    }

    if locked_entity.is_none() {
        *locked_entity = Some((Instant::now(), address));
    }

    if let Some((locked_on, entity_address)) = *locked_entity {
        if entity_address != address {
            *locked_entity = None;
            return;
        }

        let delay_offset = if config.delay_offset == 0 {
            0.0
        } else {
            (rng.random_range(-(config.delay_offset as f32)..config.delay_offset as f32) * 1000.0)
                .trunc()
                / 1000.0
        };
        let delay = Duration::from_secs_f32(
            (config.delay as f32 + delay_offset).clamp(0.0, 500.0) / 1000.0,
        );

        if locked_on.elapsed() < delay {
            return;
        }
    }

    let interval_offset = if config.tap_interval_offset == 0 {
        0.0
    } else {
        (rng.random_range(-(config.tap_interval_offset as f32)..config.tap_interval_offset as f32)
            * 1000.0)
            .trunc()
            / 1000.0
    };
    let interval = Duration::from_secs_f32(
        (config.tap_interval as f32 + interval_offset).clamp(0.0, 500.0) / 1000.0,
    );

    if config.action == 0 && shot_entity.elapsed() >= interval {
        click_mouse();
        *shot_entity = Instant::now();
    } else if config.action == 1 && !mouse_locked {
        press_mouse();
    }
}
