use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_jump, jump,
        attack_air
    );
}

// FIGHTER_STATUS_KIND_JUMP //

#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE {
        if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP {
            if !fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool() {
                fighter.status_pre_Jump_sub_param(
                    L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                    L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                    L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                    L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                    L2CValue::I32(0)
                );
            }
            return L2CValue::I32(1);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_POWBLOCK_QUAKE_JUMP);
    fighter.status_pre_Jump_sub_param(
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
        L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
        L2CValue::I32(0)
    );
    return L2CValue::I32(0);
}

#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP {
        fighter.sub_jump_item_rocketbelt();
        fighter.status_Jump_sub(L2CValue::Hash40s("invalid"), L2CValue::F32(0.0));
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Jump_Main as *const () as _))
    }
    else {
        // uncomment to let sideB ride remove double jump on hit
        //let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        //WorkModule::set_int(fighter.module_accessor, jump_count_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        fighter.status_Jump_sub(L2CValue::new_hash(0x62abde441), L2CValue::F32(0.0));
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Jump_Main as *const () as _))
    }
}

// FIGHTER_STATUS_KIND_ATTACK_AIR

#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if [
        hash40("attack_air_hi"),
        hash40("attack_air_lw")
    ].contains(&motion) {
        // Usually there's code in here to check for the random turnip pulls. However... we don't want that.
        // Instead, we want to force the turnip count to go in a rotation of 1 > 2 > 3 > 1 > 2 > 3 ...
        let mut turnip_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM);
        // Adds 1 to the turnip count. If the new turnip count is not 1, 2, or 3, reset it back to 1.
        turnip_num += 1;
        if !(1..=3).contains(&turnip_num) {
            turnip_num = 1;
        }
        WorkModule::set_int(fighter.module_accessor, turnip_num, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM);
    }
    if [
        hash40("attack_air_f"),
        hash40("attack_air_b")
    ].contains(&motion) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}