#![allow(non_snake_case, non_upper_case_globals)]

pub mod Offsets {
    // https://raw.githubusercontent.com/a2x/cs2-dumper/refs/heads/main/output/offsets.rs
    pub mod client_dll {
        use crate::offset::offsets::cs2_dumper::offsets::client_dll;

        pub const dwEntityList: usize = client_dll::dwEntityList;
        pub const dwLocalPlayerController: usize = client_dll::dwLocalPlayerController;
        pub const dwLocalPlayerPawn: usize = client_dll::dwLocalPlayerPawn;
        pub const dwPlantedC4: usize = client_dll::dwPlantedC4;
        pub const dwViewAngles: usize = client_dll::dwViewAngles;
        pub const dwViewMatrix: usize = client_dll::dwViewMatrix;
    }

    // https://raw.githubusercontent.com/a2x/cs2-dumper/refs/heads/main/output/client_dll.rs
    pub mod C_BaseEntity {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::C_BaseEntity;

        pub const m_iHealth: usize = C_BaseEntity::m_iHealth; // int32
        pub const m_iTeamNum: usize = C_BaseEntity::m_iTeamNum; // uint8
        pub const m_pGameSceneNode: usize = C_BaseEntity::m_pGameSceneNode; // CGameSceneNode*
        pub const m_fFlags: usize = C_BaseEntity::m_fFlags; // uint32
        pub const m_nSubclassID: usize = C_BaseEntity::m_nSubclassID; // CUtlStringToken
    }

    pub mod CBasePlayerController {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CBasePlayerController;

        pub const m_hPawn: usize = CBasePlayerController::m_hPawn; // CHandle<C_BasePlayerPawn>
        pub const m_iszPlayerName: usize = CBasePlayerController::m_iszPlayerName; // char[128]
    }

    pub mod CCSPlayerController {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CCSPlayerController;

        pub const m_hPlayerPawn: usize = CCSPlayerController::m_hPlayerPawn; // CHandle<C_CSPlayerPawn>
        pub const m_bPawnIsAlive: usize = CCSPlayerController::m_bPawnIsAlive; // bool
    }

    pub mod C_BasePlayerPawn {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::C_BasePlayerPawn;

        pub const m_pObserverServices: usize = C_BasePlayerPawn::m_pObserverServices; // CPlayer_ObserverServices*
        pub const m_pCameraServices: usize = C_BasePlayerPawn::m_pCameraServices; // CPlayer_CameraServices*
        pub const m_vOldOrigin: usize = C_BasePlayerPawn::m_vOldOrigin; // Vector
    }

    pub mod C_CSPlayerPawn {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::C_CSPlayerPawn;

        pub const m_aimPunchCache: usize = C_CSPlayerPawn::m_aimPunchCache; // CUtlVector<QAngle>
        pub const m_entitySpottedState: usize = C_CSPlayerPawn::m_entitySpottedState; // EntitySpottedState_t
        pub const m_ArmorValue: usize = C_CSPlayerPawn::m_ArmorValue; // int32
        pub const m_iShotsFired: usize = C_CSPlayerPawn::m_iShotsFired; // int32
        pub const m_vecLastClipCameraPos: usize = C_CSPlayerPawn::m_vecLastClipCameraPos; // Vector
        pub const m_angEyeAngles: usize = C_CSPlayerPawn::m_angEyeAngles; // QAngle
        pub const m_pClippingWeapon: usize = C_CSPlayerPawn::m_pClippingWeapon; // C_CSWeaponBase*
        pub const m_iIDEntIndex: usize = C_CSPlayerPawn::m_iIDEntIndex; // CEntityIndex
    }

    pub mod CGameSceneNode {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CGameSceneNode;

        pub const m_vecAbsOrigin: usize = CGameSceneNode::m_vecAbsOrigin; // Vector
    }

    pub mod CCSPlayerBase_CameraServices {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CCSPlayerBase_CameraServices;

        pub const m_iFOVStart: usize = CCSPlayerBase_CameraServices::m_iFOVStart; // uint32
    }

    pub mod EntitySpottedState_t {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::EntitySpottedState_t;

        pub const m_bSpottedByMask: usize = EntitySpottedState_t::m_bSpottedByMask; // uint32[2]
    }

    pub mod CSkeletonInstance {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CSkeletonInstance;

        pub const m_modelState: usize = CSkeletonInstance::m_modelState; // CModelState
    }

    pub mod CPlayer_ObserverServices {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CPlayer_ObserverServices;

        pub const m_hObserverTarget: usize = CPlayer_ObserverServices::m_hObserverTarget; // CHandle<C_BaseEntity>
    }

    pub mod C_PlantedC4 {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::C_PlantedC4;

        pub const m_nBombSite: usize = C_PlantedC4::m_nBombSite; // int32
    }

    pub mod CBasePlayerWeaponVData {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::CBasePlayerWeaponVData;

        pub const m_iMaxClip1: usize = CBasePlayerWeaponVData::m_iMaxClip1; // int32
    }

    pub mod C_BasePlayerWeapon {
        use crate::offset::client_dll::cs2_dumper::schemas::client_dll::C_BasePlayerWeapon;

        pub const m_iClip1: usize = C_BasePlayerWeapon::m_iClip1; // int32
    }
}

pub mod ProgramConfig {
    pub mod Package {
        pub const Name: &str = "ProExt";
        pub const Description: &str = "An open-source, external CS2 cheat.";
        pub const Executable: &str = "proext.exe";
        pub const Version: &str = env!("CARGO_PKG_VERSION");
        pub const Authors: &str = env!("CARGO_PKG_AUTHORS");
    }

    pub mod Imgui {
        pub const FontSize: f32 = 13.0;

        pub mod FontPaths {
            pub const Chinese: &str = "C:/Windows/Fonts/msyh.ttc";
            pub const Cryillic: &str = "C:/Windows/Fonts/Arial.ttf";
            pub const Arabic: &str = "C:/Windows/Fonts/calibri.ttf";
        }
    }

    pub mod Keys {
        use glutin::event::VirtualKeyCode;
        use mki::Keyboard;

        pub const Available: [&str; 20] = [
            "Alt",
            "Left Mouse",
            "Middle Mouse",
            "Right Mouse",
            "Side Mouse",
            "Extra Mouse",
            "Shift",
            "Control",
            "F1",
            "F2",
            "F3",
            "F4",
            "F5",
            "F6",
            "F7",
            "F8",
            "F9",
            "F10",
            "F11",
            "F12",
        ];

        pub const ToggleKey: VirtualKeyCode = VirtualKeyCode::Home;
        pub const ToggleKeyMKI: Keyboard = Keyboard::Home;

        pub const ExitKey: VirtualKeyCode = VirtualKeyCode::End;
        pub const ExitKeyMKI: Keyboard = Keyboard::Other(0x23);
    }

    pub mod TargetProcess {
        pub const Executable: &str = "cs2.exe";
        pub const MaxAttempts: u32 = 30;
        pub const InitAddressesMaxAttempts: u32 = 15;

        pub mod Window {
            pub const Title: &str = "反恐精英：全球攻势";
            pub const Class: &str = "SDL_app";
        }
    }

    pub mod CheckDelays {
        use std::time::Duration;

        pub const AttachProcess: Duration = Duration::from_millis(1000);
        pub const InitAddresses: Duration = Duration::from_millis(1000);
    }

    pub mod ThreadDelays {
        use std::time::Duration;

        pub const UpdateConfigs: Duration = Duration::from_millis(250);
        pub const WindowTasks: Duration = Duration::from_millis(25);
        pub const IOTasks: Duration = Duration::from_millis(25);
    }

    pub mod CheatDelays {
        use std::time::Duration;

        pub const Toggle: Duration = Duration::from_millis(200);
        pub const Aimbot: Duration = Duration::from_millis(10);
        pub const AimbotOffEntity: Duration = Duration::from_millis(500);
        pub const TriggerbotOffEntity: Duration = Duration::from_millis(500);
    }
}
