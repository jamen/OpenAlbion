use crate::{
    CAInputProcess,
    CGameDefinitionManager,
    CGamePlayerInterface,
    CMainGameComponent,
    CPlayer,
    CWorld,
};

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessGameBase {
    pub vmt: *mut (),
    pub ca_input_process: CAInputProcess,
    pub world: *mut CWorld,
    pub player: *mut CPlayer,
    pub definition_manager: *const CGameDefinitionManager,
    pub component: *const CMainGameComponent,
    pub p_game_player_interface: *const CGamePlayerInterface,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBButtonExitMode {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBetting {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBlock {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessBoastUI {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCameraLookAround {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessClickPastText {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCombat {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessConsole {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreature {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreatureActivateZTarget {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreatureActivateZTargetOnPress {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlCreatureRightStick {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlFreeCamera {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessControlSpirit {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCreatureMovement {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCreatureMovementWatchForControlAngleChange {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCreditsUI {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCutScene {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessCycleSpecialCameraModes {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessDead {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessDebugControls {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessDigging {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFireheartMinigame {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFirstPerson {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFirstPersonLookAround {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFirstPersonTargeting {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFishing {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessFreezeControls {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessHeroAbilitiesScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessHeroInformationScreens {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInGameMenu {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventory {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryClothing {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryExperienceScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryMagicScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryMapScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryQuestsScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryStatsScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryTradeScreen {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessInventoryWeapons {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessJumpingAndRolling {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessLightning {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessMain {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessOracleMinigame {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessParalysed {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessPhotojournalCapture {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessProjectileTargetingAnalogueZoom {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessQuestCompletionUI {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessQuickAccessItems {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessQuickAccessMenu {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessRebootGame {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessSetRangedWeaponMode {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessSetRangedWeaponThirdPersonMode {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessSpecialAbilities {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessStrafe {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessTargetLockCycleTargets {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessTargetLockRightStickTargetSelect {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessTavernGame {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessToggleViewHeroMode {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessUseEnvironment {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessUseRangedWeapon {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessUseRangedWeaponZLock {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessWatchForRangedWeaponThirdPersonModeTermination {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessWatchForWillChargeUpThirdPersonModeTermination {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessYesNoQuestion {
    pub c_input_process_game_base: CInputProcessGameBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct CInputProcessZTarget {
    pub c_input_process_game_base: CInputProcessGameBase,
}
