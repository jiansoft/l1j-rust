/// 全職業技能系統 - 基於官方數據。
///
/// 天堂1 共 7 個職業：王族、騎士、妖精、法師、黑暗妖精、龍騎士、幻術師。
/// 黑暗妖精已在 darkelf_skills.rs 中實作，此檔案涵蓋其餘 6 個職業。
///
/// 資料來源：LoA 美版天堂 3.63、巴哈姆特攻略百科、天堂透視鏡、17173 攻略。

// ===========================================================================
// 共通結構
// ===========================================================================

/// 職業類型。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharClass {
    Royal = 0,         // 王族
    Knight = 1,        // 騎士
    Elf = 2,           // 妖精
    Mage = 3,          // 法師
    DarkElf = 4,       // 黑暗妖精
    DragonKnight = 5,  // 龍騎士
    Illusionist = 6,   // 幻術師
}

/// 技能類型。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkillKind {
    Active,   // 主動
    Passive,  // 被動
    Toggle,   // 開關
}

/// 技能目標。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkillTarget {
    SelfOnly,      // 自身
    SingleEnemy,   // 單一敵人
    SingleAlly,    // 單一友軍
    AreaEnemy,     // 範圍敵方
    AreaAlly,      // 範圍友方
    ClanMember,    // 血盟成員
}

/// 職業技能模板（官方數據）。
#[derive(Debug, Clone)]
pub struct ClassSkill {
    pub skill_id: i32,
    pub name: &'static str,
    pub class: CharClass,
    pub grade: i32,           // 技能等級 1-5
    pub learn_level: i32,     // 學習所需角色等級
    pub mp_cost: i32,
    pub hp_cost: i32,
    pub reagent_cost: i32,    // 觸媒/材料消耗數量
    pub reagent_item_id: i32, // 觸媒 item ID (0=無)
    pub cast_time_ms: i32,    // 施法延遲 ms
    pub cooldown_ms: i32,     // 冷卻時間 ms
    pub duration_sec: i32,    // 持續時間秒 (0=瞬發)
    pub kind: SkillKind,
    pub target: SkillTarget,
    pub effect_desc: &'static str,
}

// ===========================================================================
// 王族 (Royal/Prince/Princess) 技能
// ===========================================================================

pub fn royal_skills() -> Vec<ClassSkill> {
    vec![
        // === 通用魔法（王族可學 Lv10-20）===
        ClassSkill { skill_id: 1, name: "光箭", class: CharClass::Royal, grade: 1, learn_level: 10,
            mp_cost: 3, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 440, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "壓縮魔法能量射出，造成些許傷害" },
        ClassSkill { skill_id: 3, name: "保護罩", class: CharClass::Royal, grade: 1, learn_level: 10,
            mp_cost: 8, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 1800,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "防禦+2" },
        ClassSkill { skill_id: 9, name: "初級治癒術", class: CharClass::Royal, grade: 1, learn_level: 10,
            mp_cost: 4, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleAlly,
            effect_desc: "恢復少量體力" },
        // === 王族專屬技能 ===
        ClassSkill { skill_id: 113, name: "精準目標", class: CharClass::Royal, grade: 1, learn_level: 15,
            mp_cost: 1, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 100, cooldown_ms: 0, duration_sec: 16,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "目標傷害減免-3" },
        ClassSkill { skill_id: 114, name: "灼熱武器", class: CharClass::Royal, grade: 2, learn_level: 30,
            mp_cost: 20, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 100, cooldown_ms: 0, duration_sec: 640,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "攻擊成功+5、額外攻擊+5" },
        ClassSkill { skill_id: 115, name: "閃亮之盾", class: CharClass::Royal, grade: 2, learn_level: 55,
            mp_cost: 20, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 100, cooldown_ms: 0, duration_sec: 640,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "防禦力+8" },
        ClassSkill { skill_id: 116, name: "呼喚盟友", class: CharClass::Royal, grade: 3, learn_level: 60,
            mp_cost: 50, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 100, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::ClanMember,
            effect_desc: "召喚血盟成員到自己位置" },
        ClassSkill { skill_id: 117, name: "勇猛意志", class: CharClass::Royal, grade: 3, learn_level: 50,
            mp_cost: 25, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 100, cooldown_ms: 0, duration_sec: 640,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "30%機率攻擊力×1.5倍" },
        ClassSkill { skill_id: 118, name: "援護盟友", class: CharClass::Royal, grade: 3, learn_level: 55,
            mp_cost: 50, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 100, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::ClanMember,
            effect_desc: "傳送到血盟成員所在位置" },
    ]
}

// ===========================================================================
// 騎士 (Knight) 技能
// ===========================================================================

pub fn knight_skills() -> Vec<ClassSkill> {
    vec![
        ClassSkill { skill_id: 120, name: "衝擊之暈", class: CharClass::Knight, grade: 2, learn_level: 50,
            mp_cost: 15, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 0, cooldown_ms: 8000, duration_sec: 5,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "暈眩3-6秒，50%命中率（等級差±5%/級）" },
        ClassSkill { skill_id: 121, name: "反擊屏障", class: CharClass::Knight, grade: 2, learn_level: 50,
            mp_cost: 10, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 0, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Passive, target: SkillTarget::SelfOnly,
            effect_desc: "35%機率反彈物理傷害（不反彈弓箭），等級越高機率越高" },
        ClassSkill { skill_id: 122, name: "增幅防禦", class: CharClass::Knight, grade: 2, learn_level: 50,
            mp_cost: 5, hp_cost: 50, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 0, cooldown_ms: 0, duration_sec: 32,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "傷害減免+1（Lv50），每5級額外+1，最高+7（Lv80）" },
        ClassSkill { skill_id: 123, name: "尖刺盔甲", class: CharClass::Knight, grade: 1, learn_level: 30,
            mp_cost: 10, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 0, cooldown_ms: 0, duration_sec: 192,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "物理命中率+6" },
        ClassSkill { skill_id: 124, name: "堅固防護", class: CharClass::Knight, grade: 2, learn_level: 50,
            mp_cost: 10, hp_cost: 100, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 0, cooldown_ms: 0, duration_sec: 64,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "迴避率+15（需裝備盾牌）" },
    ]
}

// ===========================================================================
// 妖精 (Elf) 技能
// ===========================================================================

pub fn elf_skills() -> Vec<ClassSkill> {
    vec![
        // 精靈魔法
        ClassSkill { skill_id: 130, name: "三重矢", class: CharClass::Elf, grade: 2, learn_level: 30,
            mp_cost: 15, hp_cost: 0, reagent_cost: 3, reagent_item_id: 40745,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "同時射出3支箭矢攻擊目標" },
        ClassSkill { skill_id: 131, name: "精靈之火", class: CharClass::Elf, grade: 2, learn_level: 30,
            mp_cost: 12, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 440, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "火屬性精靈魔法攻擊" },
        ClassSkill { skill_id: 132, name: "封印禁地", class: CharClass::Elf, grade: 2, learn_level: 30,
            mp_cost: 15, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 16,
            kind: SkillKind::Active, target: SkillTarget::AreaEnemy,
            effect_desc: "範圍內敵人無法使用魔法" },
        ClassSkill { skill_id: 133, name: "召喚屬性精靈", class: CharClass::Elf, grade: 3, learn_level: 45,
            mp_cost: 30, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 1000, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "召喚風/水/火/土屬性精靈作戰" },
        ClassSkill { skill_id: 134, name: "風之神射", class: CharClass::Elf, grade: 3, learn_level: 45,
            mp_cost: 25, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "強力風屬性箭矢攻擊" },
        ClassSkill { skill_id: 135, name: "生命之泉", class: CharClass::Elf, grade: 2, learn_level: 30,
            mp_cost: 20, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 300,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "持續回復HP" },
    ]
}

// ===========================================================================
// 龍騎士 (Dragon Knight) 技能
// ===========================================================================

pub fn dragon_knight_skills() -> Vec<ClassSkill> {
    vec![
        // 1級 (Lv15)
        ClassSkill { skill_id: 140, name: "龍之護鎧", class: CharClass::DragonKnight, grade: 1, learn_level: 15,
            mp_cost: 10, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 300,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "傷害減免+5，Lv80後每2級額外+1" },
        ClassSkill { skill_id: 141, name: "燃燒擊砍", class: CharClass::DragonKnight, grade: 1, learn_level: 15,
            mp_cost: 10, hp_cost: 5, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 200, cooldown_ms: 2000, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "強力火焰斬擊" },
        ClassSkill { skill_id: 142, name: "護衛毀滅", class: CharClass::DragonKnight, grade: 1, learn_level: 15,
            mp_cost: 15, hp_cost: 10, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 200, cooldown_ms: 3000, duration_sec: 10,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "目標AC降低10" },
        ClassSkill { skill_id: 143, name: "岩漿噴吐", class: CharClass::DragonKnight, grade: 1, learn_level: 15,
            mp_cost: 12, hp_cost: 5, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 440, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::AreaEnemy,
            effect_desc: "目標及周邊敵人火焰傷害" },
        // 2級 (Lv30)
        ClassSkill { skill_id: 144, name: "血之渴望", class: CharClass::DragonKnight, grade: 2, learn_level: 30,
            mp_cost: 15, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 300,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "攻擊速度和移動速度提升" },
        ClassSkill { skill_id: 145, name: "屠宰者", class: CharClass::DragonKnight, grade: 2, learn_level: 30,
            mp_cost: 20, hp_cost: 10, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 200, cooldown_ms: 4000, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "3連擊，弱點曝光×3時吸取150HP+傷害加成" },
        ClassSkill { skill_id: 146, name: "恐懼無助", class: CharClass::DragonKnight, grade: 2, learn_level: 30,
            mp_cost: 15, hp_cost: 5, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 16,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "降低目標閃避率" },
        // 3級 (Lv45)
        ClassSkill { skill_id: 147, name: "致命身軀", class: CharClass::DragonKnight, grade: 3, learn_level: 45,
            mp_cost: 0, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 0, cooldown_ms: 0, duration_sec: 0,
            kind: SkillKind::Passive, target: SkillTarget::SelfOnly,
            effect_desc: "23%機率反射敵人30點傷害" },
        ClassSkill { skill_id: 148, name: "奪命之雷", class: CharClass::DragonKnight, grade: 3, learn_level: 45,
            mp_cost: 25, hp_cost: 10, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 440, cooldown_ms: 3000, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::AreaEnemy,
            effect_desc: "落雷攻擊1-4目標，28-44傷害，60%機率束縛1-4秒" },
    ]
}

// ===========================================================================
// 幻術師 (Illusionist) 技能
// ===========================================================================

pub fn illusionist_skills() -> Vec<ClassSkill> {
    vec![
        ClassSkill { skill_id: 150, name: "幻覺：化身", class: CharClass::Illusionist, grade: 2, learn_level: 30,
            mp_cost: 35, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 12,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "攻擊力/魔攻+10(+6)，受傷增加50%，命中100%" },
        ClassSkill { skill_id: 151, name: "疼痛的歡愉", class: CharClass::Illusionist, grade: 2, learn_level: 30,
            mp_cost: 40, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 3000, duration_sec: 0,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "依自身損失HP計算傷害：(損失HP÷5)×1.6，命中100%" },
        ClassSkill { skill_id: 152, name: "骷髏毀壞", class: CharClass::Illusionist, grade: 1, learn_level: 15,
            mp_cost: 20, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 2,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "10傷害，20-30%機率昏迷1秒" },
        ClassSkill { skill_id: 153, name: "幻想", class: CharClass::Illusionist, grade: 2, learn_level: 30,
            mp_cost: 30, hp_cost: 25, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 5,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "30%機率石化，無法移動/攻擊（被攻擊解除）" },
        ClassSkill { skill_id: 154, name: "混亂", class: CharClass::Illusionist, grade: 2, learn_level: 30,
            mp_cost: 15, hp_cost: 10, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 8,
            kind: SkillKind::Active, target: SkillTarget::SingleEnemy,
            effect_desc: "11傷害，20-30%機率混亂8秒（無法使用魔法）" },
        ClassSkill { skill_id: 155, name: "鏡像", class: CharClass::Illusionist, grade: 3, learn_level: 45,
            mp_cost: 20, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 360, cooldown_ms: 0, duration_sec: 32,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "產生分身，吸引敵人攻擊" },
        ClassSkill { skill_id: 156, name: "立方體：燃燒", class: CharClass::Illusionist, grade: 3, learn_level: 45,
            mp_cost: 30, hp_cost: 0, reagent_cost: 0, reagent_item_id: 0,
            cast_time_ms: 1000, cooldown_ms: 0, duration_sec: 300,
            kind: SkillKind::Active, target: SkillTarget::SelfOnly,
            effect_desc: "召喚火焰立方體，持續對附近敵人造成火焰傷害" },
    ]
}

// ===========================================================================
// 彙總所有職業技能
// ===========================================================================

/// 取得所有職業技能（不含法師通用魔法和黑暗妖精，它們有獨立模組）。
pub fn all_class_skills() -> Vec<ClassSkill> {
    let mut all = Vec::new();
    all.extend(royal_skills());
    all.extend(knight_skills());
    all.extend(elf_skills());
    all.extend(dragon_knight_skills());
    all.extend(illusionist_skills());
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_classes_have_skills() {
        let all = all_class_skills();
        let royal_count = all.iter().filter(|s| s.class == CharClass::Royal).count();
        let knight_count = all.iter().filter(|s| s.class == CharClass::Knight).count();
        let elf_count = all.iter().filter(|s| s.class == CharClass::Elf).count();
        let dk_count = all.iter().filter(|s| s.class == CharClass::DragonKnight).count();
        let illu_count = all.iter().filter(|s| s.class == CharClass::Illusionist).count();

        assert!(royal_count >= 7, "Royal should have >=7 skills, got {}", royal_count);
        assert!(knight_count >= 5, "Knight should have >=5 skills, got {}", knight_count);
        assert!(elf_count >= 5, "Elf should have >=5 skills, got {}", elf_count);
        assert!(dk_count >= 8, "DragonKnight should have >=8 skills, got {}", dk_count);
        assert!(illu_count >= 6, "Illusionist should have >=6 skills, got {}", illu_count);
    }

    #[test]
    fn test_stun_bash_official() {
        let skills = knight_skills();
        let stun = skills.iter().find(|s| s.name == "衝擊之暈").unwrap();
        assert_eq!(stun.mp_cost, 15);
        assert_eq!(stun.cooldown_ms, 8000);
        assert_eq!(stun.learn_level, 50);
    }

    #[test]
    fn test_counter_barrier_passive() {
        let skills = knight_skills();
        let cb = skills.iter().find(|s| s.name == "反擊屏障").unwrap();
        assert_eq!(cb.kind, SkillKind::Passive);
        assert!(cb.effect_desc.contains("35%"));
    }

    #[test]
    fn test_amp_defense_scaling() {
        let skills = knight_skills();
        let ad = skills.iter().find(|s| s.name == "增幅防禦").unwrap();
        assert_eq!(ad.hp_cost, 50);
        assert_eq!(ad.duration_sec, 32);
        assert!(ad.effect_desc.contains("Lv50"));
    }

    #[test]
    fn test_royal_call_clan() {
        let skills = royal_skills();
        let call = skills.iter().find(|s| s.name == "呼喚盟友").unwrap();
        assert_eq!(call.target, SkillTarget::ClanMember);
        assert_eq!(call.mp_cost, 50);
    }

    #[test]
    fn test_dragon_knight_slaughter() {
        let skills = dragon_knight_skills();
        let slaughter = skills.iter().find(|s| s.name == "屠宰者").unwrap();
        assert_eq!(slaughter.mp_cost, 20);
        assert_eq!(slaughter.hp_cost, 10);
        assert!(slaughter.effect_desc.contains("150HP"));
    }

    #[test]
    fn test_illusionist_pain() {
        let skills = illusionist_skills();
        let pain = skills.iter().find(|s| s.name == "疼痛的歡愉").unwrap();
        assert_eq!(pain.cooldown_ms, 3000);
        assert!(pain.effect_desc.contains("1.6"));
    }

    #[test]
    fn test_no_duplicate_ids() {
        let all = all_class_skills();
        let mut ids: Vec<i32> = all.iter().map(|s| s.skill_id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), all.len(), "Duplicate skill IDs found!");
    }
}
