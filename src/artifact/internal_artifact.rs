use log::error;
use regex::Regex;
use std::hash::{Hash, Hasher};
use strum_macros::Display;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ArtifactStatKey {
    HealingBonus,
    CriticalDamage,
    Critical,
    Atk,
    AtkPercentage,
    ElementalMastery,
    Recharge,
    HpPercentage,
    Hp,
    DefPercentage,
    Def,
    ElectroBonus,
    PyroBonus,
    HydroBonus,
    CryoBonus,
    AnemoBonus,
    GeoBonus,
    PhysicalBonus,
    DendroBonus,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ArtifactSlotKey {
    Flower,
    Plume,
    Sands,
    Goblet,
    Circlet,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum ArtifactSetKey {
    ArchaicPetra,
    HeartOfDepth,
    BlizzardStrayer,
    RetracingBolide,
    NoblesseOblige,
    GladiatorsFinale,
    MaidenBeloved,
    ViridescentVenerer,
    Lavawalker,
    CrimsonWitchOfFlames,
    Thundersoother,
    ThunderingFury,
    BloodstainedChivalry,
    WanderersTroupe,
    Scholar,
    Gambler,
    TinyMiracle,
    MartialArtist,
    BraveHeart,
    ResolutionOfSojourner,
    DefenderWill,
    Berserker,
    Instructor,
    Exile,
    Adventurer,
    LuckyDog,
    TravelingDoctor,
    PrayersForWisdom,
    PrayersToSpringtime,
    PrayersForIllumination,
    PrayersForDestiny,
    PaleFlame,
    TenacityOfTheMillelith,
    EmblemOfSeveredFate,
    ShimenawasReminiscence,
    HuskOfOpulentDreams,
    OceanHuedClam,
    VermillionHereafter,
    EchoesOfAnOffering,
    DeepwoodMemories,
    GildedDreams,
    DesertPavilionChronicle,
    FlowerOfParadiseLost,
    NymphsDream,
    VourukashasGlow,
    MarechausseeHunter,
    GoldenTroupe,
    SongOfDaysPast,
    NighttimeWhispersInTheEchoingWoods,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum CharacterKey {
    KamisatoAyaka,
    Jean,
    Traveler,
    Lisa,
    Barbara,
    Kaeya,
    Diluc,
    Razor,
    Amber,
    Venti,
    Xiangling,
    Beidou,
    Xingqiu,
    Xiao,
    Ningguang,
    Klee,
    Zhongli,
    Fischl,
    Bennett,
    Tartaglia,
    Noelle,
    Qiqi,
    Chongyun,
    Ganyu,
    Albedo,
    Diona,
    Mona,
    Keqing,
    Sucrose,
    Xinyan,
    Rosaria,
    HuTao,
    KaedeharaKazuha,
    Yanfei,
    Yoimiya,
    Thoma,
    Eula,
    RaidenShogun,
    Sayu,
    SangonomiyaKokomi,
    Gorou,
    KujouSara,
    AratakiItto,
    YaeMiko,
    ShikanoinHeizou,
    Yelan,
    Kirara,
    Aloy,
    Shenhe,
    YunJin,
    KukiShinobu,
    KamisatoAyato,
    Collei,
    Dori,
    Tighnari,
    Nilou,
    Cyno,
    Candace,
    Nahida,
    Layla,
    Wanderer,
    Faruzan,
    Yaoyao,
    Alhaitham,
    Dehya,
    Mika,
    Kaveh,
    Baizhu,
    Lynette,
    Lyney,
    Freminet,
    Wriothesley,
    Neuvillette,
    Charlotte,
    Furina,
    Chevreuse,
    Navia,
    Gaming,
    Liuyun
}

#[derive(Debug, Clone)]
pub struct ArtifactStat {
    pub key: ArtifactStatKey,
    pub value: f64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct InternalArtifact {
    pub set_key: ArtifactSetKey,
    pub slot_key: ArtifactSlotKey,
    pub rarity: u32,
    pub level: u32,
    pub lock: bool,
    pub location: Option<CharacterKey>,
    pub main_stat: ArtifactStat,
    pub sub_stat_1: Option<ArtifactStat>,
    pub sub_stat_2: Option<ArtifactStat>,
    pub sub_stat_3: Option<ArtifactStat>,
    pub sub_stat_4: Option<ArtifactStat>,
}

impl Hash for ArtifactStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for ArtifactStat {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for ArtifactStat {}

impl ArtifactStatKey {
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<ArtifactStatKey> {
        match name {
            "治疗加成" => Some(ArtifactStatKey::HealingBonus),
            "暴击伤害" => Some(ArtifactStatKey::CriticalDamage),
            "暴击率" => Some(ArtifactStatKey::Critical),
            "攻击力" => {
                if is_percentage {
                    Some(ArtifactStatKey::AtkPercentage)
                } else {
                    Some(ArtifactStatKey::Atk)
                }
            }
            "元素精通" => Some(ArtifactStatKey::ElementalMastery),
            "元素充能效率" => Some(ArtifactStatKey::Recharge),
            "生命值" => {
                if is_percentage {
                    Some(ArtifactStatKey::HpPercentage)
                } else {
                    Some(ArtifactStatKey::Hp)
                }
            }
            "防御力" => {
                if is_percentage {
                    Some(ArtifactStatKey::DefPercentage)
                } else {
                    Some(ArtifactStatKey::Def)
                }
            }
            "雷元素伤害加成" => Some(ArtifactStatKey::ElectroBonus),
            "火元素伤害加成" => Some(ArtifactStatKey::PyroBonus),
            "水元素伤害加成" => Some(ArtifactStatKey::HydroBonus),
            "冰元素伤害加成" => Some(ArtifactStatKey::CryoBonus),
            "风元素伤害加成" => Some(ArtifactStatKey::AnemoBonus),
            "岩元素伤害加成" => Some(ArtifactStatKey::GeoBonus),
            "草元素伤害加成" => Some(ArtifactStatKey::DendroBonus),
            "物理伤害加成" => Some(ArtifactStatKey::PhysicalBonus),
            _ => {
                if name.starts_with("雷") {
                    return Some(ArtifactStatKey::ElectroBonus);
                } else if name.starts_with("火") {
                    return Some(ArtifactStatKey::PyroBonus);
                } else if name.starts_with("水") {
                    return Some(ArtifactStatKey::HydroBonus);
                } else if name.starts_with("冰") {
                    return Some(ArtifactStatKey::CryoBonus);
                } else if name.starts_with("风") {
                    return Some(ArtifactStatKey::AnemoBonus);
                } else if name.starts_with("岩") {
                    return Some(ArtifactStatKey::GeoBonus);
                } else if name.starts_with("草") {
                    return Some(ArtifactStatKey::DendroBonus);
                } else if name.starts_with("物理") {
                    return Some(ArtifactStatKey::PhysicalBonus);
                } else {
                    return None;
                }
            }
        }
    }
}

impl ArtifactStat {
    // e.g "生命值+4,123", "暴击率+10%"
    pub fn from_zh_cn_raw(s: &str) -> Option<ArtifactStat> {
        let temp: Vec<&str> = s.split("+").collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains("%");
        let stat_key = match ArtifactStatKey::from_zh_cn(temp[0], is_percentage) {
            Some(v) => v,
            None => return None,
        };

        let re = Regex::new("[%,]").unwrap();
        let value = match re.replace_all(temp[1], "").parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                error!("stat `{}` parse error", s);
                return None;
            }
        };
        // if is_percentage {
        //     value /= 100.0;
        // }

        Some(ArtifactStat {
            key: stat_key,
            value,
        })
    }
}

impl ArtifactSetKey {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSetKey> {
        // let s = match get_real_artifact_name_chs(s) {
        //     Some(v) => v,
        //     None => return None,
        // };
        // println!("name: {}", s);
        match s {
            "磐陀裂生之花" => Some(ArtifactSetKey::ArchaicPetra),
            "嵯峨群峰之翼" => Some(ArtifactSetKey::ArchaicPetra),
            "星罗圭壁之晷" => Some(ArtifactSetKey::ArchaicPetra),
            // "壁" is different
            "星罗圭璧之晷" => Some(ArtifactSetKey::ArchaicPetra),
            "巉岩琢塑之樽" => Some(ArtifactSetKey::ArchaicPetra),
            "不动玄石之相" => Some(ArtifactSetKey::ArchaicPetra),
            "历经风雪的思念" => Some(ArtifactSetKey::BlizzardStrayer),
            "摧冰而行的执望" => Some(ArtifactSetKey::BlizzardStrayer),
            "冰雪故园的终期" => Some(ArtifactSetKey::BlizzardStrayer),
            "遍结寒霜的傲骨" => Some(ArtifactSetKey::BlizzardStrayer),
            "破冰踏雪的回音" => Some(ArtifactSetKey::BlizzardStrayer),
            "染血的铁之心" => Some(ArtifactSetKey::BloodstainedChivalry),
            "染血的黑之羽" => Some(ArtifactSetKey::BloodstainedChivalry),
            "骑士染血之时" => Some(ArtifactSetKey::BloodstainedChivalry),
            "染血骑士之杯" => Some(ArtifactSetKey::BloodstainedChivalry),
            "染血的铁假面" => Some(ArtifactSetKey::BloodstainedChivalry),
            "魔女的炎之花" => Some(ArtifactSetKey::CrimsonWitchOfFlames),
            "魔女常燃之羽" => Some(ArtifactSetKey::CrimsonWitchOfFlames),
            "魔女破灭之时" => Some(ArtifactSetKey::CrimsonWitchOfFlames),
            "魔女的心之火" => Some(ArtifactSetKey::CrimsonWitchOfFlames),
            "焦灼的魔女帽" => Some(ArtifactSetKey::CrimsonWitchOfFlames),
            "角斗士的留恋" => Some(ArtifactSetKey::GladiatorsFinale),
            "角斗士的归宿" => Some(ArtifactSetKey::GladiatorsFinale),
            "角斗士的希冀" => Some(ArtifactSetKey::GladiatorsFinale),
            "角斗士的酣醉" => Some(ArtifactSetKey::GladiatorsFinale),
            "角斗士的凯旋" => Some(ArtifactSetKey::GladiatorsFinale),
            "饰金胸花" => Some(ArtifactSetKey::HeartOfDepth),
            "追忆之风" => Some(ArtifactSetKey::HeartOfDepth),
            "坚铜罗盘" => Some(ArtifactSetKey::HeartOfDepth),
            "沉波之盏" => Some(ArtifactSetKey::HeartOfDepth),
            "酒渍船帽" => Some(ArtifactSetKey::HeartOfDepth),
            "渡火者的决绝" => Some(ArtifactSetKey::Lavawalker),
            "渡火者的解脱" => Some(ArtifactSetKey::Lavawalker),
            "渡火者的煎熬" => Some(ArtifactSetKey::Lavawalker),
            "渡火者的醒悟" => Some(ArtifactSetKey::Lavawalker),
            "渡火者的智慧" => Some(ArtifactSetKey::Lavawalker),
            "远方的少女之心" => Some(ArtifactSetKey::MaidenBeloved),
            "少女飘摇的思念" => Some(ArtifactSetKey::MaidenBeloved),
            "少女苦短的良辰" => Some(ArtifactSetKey::MaidenBeloved),
            "少女片刻的闲暇" => Some(ArtifactSetKey::MaidenBeloved),
            "少女易逝的芳颜" => Some(ArtifactSetKey::MaidenBeloved),
            "宗室之花" => Some(ArtifactSetKey::NoblesseOblige),
            "宗室之翎" => Some(ArtifactSetKey::NoblesseOblige),
            "宗室时计" => Some(ArtifactSetKey::NoblesseOblige),
            "宗室银瓮" => Some(ArtifactSetKey::NoblesseOblige),
            "宗室面具" => Some(ArtifactSetKey::NoblesseOblige),
            "夏祭之花" => Some(ArtifactSetKey::RetracingBolide),
            "夏祭终末" => Some(ArtifactSetKey::RetracingBolide),
            "夏祭之刻" => Some(ArtifactSetKey::RetracingBolide),
            "夏祭水玉" => Some(ArtifactSetKey::RetracingBolide),
            "夏祭之面" => Some(ArtifactSetKey::RetracingBolide),
            "平雷之心" => Some(ArtifactSetKey::Thundersoother),
            "平雷之羽" => Some(ArtifactSetKey::Thundersoother),
            "平雷之刻" => Some(ArtifactSetKey::Thundersoother),
            "平雷之器" => Some(ArtifactSetKey::Thundersoother),
            "平雷之冠" => Some(ArtifactSetKey::Thundersoother),
            "雷鸟的怜悯" => Some(ArtifactSetKey::ThunderingFury),
            "雷灾的孑遗" => Some(ArtifactSetKey::ThunderingFury),
            "雷霆的时计" => Some(ArtifactSetKey::ThunderingFury),
            "降雷的凶兆" => Some(ArtifactSetKey::ThunderingFury),
            "唤雷的头冠" => Some(ArtifactSetKey::ThunderingFury),
            "野花记忆的绿野" => Some(ArtifactSetKey::ViridescentVenerer),
            "猎人青翠的箭羽" => Some(ArtifactSetKey::ViridescentVenerer),
            "翠绿猎人的笃定" => Some(ArtifactSetKey::ViridescentVenerer),
            "翠绿猎人的容器" => Some(ArtifactSetKey::ViridescentVenerer),
            "翠绿的猎人之冠" => Some(ArtifactSetKey::ViridescentVenerer),
            "乐团的晨光" => Some(ArtifactSetKey::WanderersTroupe),
            "琴师的箭羽" => Some(ArtifactSetKey::WanderersTroupe),
            "终幕的时计" => Some(ArtifactSetKey::WanderersTroupe),
            "终末的时计" => Some(ArtifactSetKey::WanderersTroupe),
            "吟游者之壶" => Some(ArtifactSetKey::WanderersTroupe),
            "指挥的礼帽" => Some(ArtifactSetKey::WanderersTroupe),
            "战狂的蔷薇" => Some(ArtifactSetKey::Berserker),
            "战狂的翎羽" => Some(ArtifactSetKey::Berserker),
            "战狂的时计" => Some(ArtifactSetKey::Berserker),
            "战狂的骨杯" => Some(ArtifactSetKey::Berserker),
            "战狂的鬼面" => Some(ArtifactSetKey::Berserker),
            "勇士的勋章" => Some(ArtifactSetKey::BraveHeart),
            "勇士的期许" => Some(ArtifactSetKey::BraveHeart),
            "勇士的坚毅" => Some(ArtifactSetKey::BraveHeart),
            "勇士的壮行" => Some(ArtifactSetKey::BraveHeart),
            "勇士的冠冕" => Some(ArtifactSetKey::BraveHeart),
            "守护之花" => Some(ArtifactSetKey::DefenderWill),
            "守护徽印" => Some(ArtifactSetKey::DefenderWill),
            "守护座钟" => Some(ArtifactSetKey::DefenderWill),
            "守护之皿" => Some(ArtifactSetKey::DefenderWill),
            "守护束带" => Some(ArtifactSetKey::DefenderWill),
            "流放者之花" => Some(ArtifactSetKey::Exile),
            "流放者之羽" => Some(ArtifactSetKey::Exile),
            "流放者怀表" => Some(ArtifactSetKey::Exile),
            "流放者之杯" => Some(ArtifactSetKey::Exile),
            "流放者头冠" => Some(ArtifactSetKey::Exile),
            "赌徒的胸花" => Some(ArtifactSetKey::Gambler),
            "赌徒的羽饰" => Some(ArtifactSetKey::Gambler),
            "赌徒的怀表" => Some(ArtifactSetKey::Gambler),
            "赌徒的骰盅" => Some(ArtifactSetKey::Gambler),
            "赌徒的耳环" => Some(ArtifactSetKey::Gambler),
            "教官的胸花" => Some(ArtifactSetKey::Instructor),
            "教官的羽饰" => Some(ArtifactSetKey::Instructor),
            "教官的怀表" => Some(ArtifactSetKey::Instructor),
            "教官的茶杯" => Some(ArtifactSetKey::Instructor),
            "教官的帽子" => Some(ArtifactSetKey::Instructor),
            "武人的红花" => Some(ArtifactSetKey::MartialArtist),
            "武人的羽饰" => Some(ArtifactSetKey::MartialArtist),
            "武人的水漏" => Some(ArtifactSetKey::MartialArtist),
            "武人的酒杯" => Some(ArtifactSetKey::MartialArtist),
            "武人的头巾" => Some(ArtifactSetKey::MartialArtist),
            "祭水礼冠" => Some(ArtifactSetKey::PrayersForDestiny),
            "祭火礼冠" => Some(ArtifactSetKey::PrayersForIllumination),
            "祭雷礼冠" => Some(ArtifactSetKey::PrayersForWisdom),
            "祭冰礼冠" => Some(ArtifactSetKey::PrayersToSpringtime),
            "故人之心" => Some(ArtifactSetKey::ResolutionOfSojourner),
            "归乡之羽" => Some(ArtifactSetKey::ResolutionOfSojourner),
            "逐光之石" => Some(ArtifactSetKey::ResolutionOfSojourner),
            "异国之盏" => Some(ArtifactSetKey::ResolutionOfSojourner),
            "感别之冠" => Some(ArtifactSetKey::ResolutionOfSojourner),
            "学士的书签" => Some(ArtifactSetKey::Scholar),
            "学士的羽笔" => Some(ArtifactSetKey::Scholar),
            "学士的时钟" => Some(ArtifactSetKey::Scholar),
            "学士的墨杯" => Some(ArtifactSetKey::Scholar),
            "学士的镜片" => Some(ArtifactSetKey::Scholar),
            "奇迹之花" => Some(ArtifactSetKey::TinyMiracle),
            "奇迹之羽" => Some(ArtifactSetKey::TinyMiracle),
            "奇迹之沙" => Some(ArtifactSetKey::TinyMiracle),
            "奇迹之杯" => Some(ArtifactSetKey::TinyMiracle),
            "奇迹耳坠" => Some(ArtifactSetKey::TinyMiracle),
            "冒险家之花" => Some(ArtifactSetKey::Adventurer),
            "冒险家尾羽" => Some(ArtifactSetKey::Adventurer),
            "冒险家怀表" => Some(ArtifactSetKey::Adventurer),
            "冒险家金杯" => Some(ArtifactSetKey::Adventurer),
            "冒险家头带" => Some(ArtifactSetKey::Adventurer),
            "幸运儿绿花" => Some(ArtifactSetKey::LuckyDog),
            "幸运儿鹰羽" => Some(ArtifactSetKey::LuckyDog),
            "幸运儿沙漏" => Some(ArtifactSetKey::LuckyDog),
            "幸运儿之杯" => Some(ArtifactSetKey::LuckyDog),
            "幸运儿银冠" => Some(ArtifactSetKey::LuckyDog),
            "游医的银莲" => Some(ArtifactSetKey::TravelingDoctor),
            "游医的枭羽" => Some(ArtifactSetKey::TravelingDoctor),
            "游医的怀钟" => Some(ArtifactSetKey::TravelingDoctor),
            "游医的药壶" => Some(ArtifactSetKey::TravelingDoctor),
            "游医的方巾" => Some(ArtifactSetKey::TravelingDoctor),
            "勋绩之花" => Some(ArtifactSetKey::TenacityOfTheMillelith),
            "昭武翎羽" => Some(ArtifactSetKey::TenacityOfTheMillelith),
            "金铜时晷" => Some(ArtifactSetKey::TenacityOfTheMillelith),
            "盟誓金爵" => Some(ArtifactSetKey::TenacityOfTheMillelith),
            "将帅兜鍪" => Some(ArtifactSetKey::TenacityOfTheMillelith),
            "无垢之花" => Some(ArtifactSetKey::PaleFlame),
            "贤医之羽" => Some(ArtifactSetKey::PaleFlame),
            "停摆之刻" => Some(ArtifactSetKey::PaleFlame),
            "超越之盏" => Some(ArtifactSetKey::PaleFlame),
            "嗤笑之面" => Some(ArtifactSetKey::PaleFlame),
            "明威之镡" => Some(ArtifactSetKey::EmblemOfSeveredFate),
            "切落之羽" => Some(ArtifactSetKey::EmblemOfSeveredFate),
            "雷云之笼" => Some(ArtifactSetKey::EmblemOfSeveredFate),
            "绯花之壶" => Some(ArtifactSetKey::EmblemOfSeveredFate),
            "华饰之兜" => Some(ArtifactSetKey::EmblemOfSeveredFate),
            "羁缠之花" => Some(ArtifactSetKey::ShimenawasReminiscence),
            "思忆之矢" => Some(ArtifactSetKey::ShimenawasReminiscence),
            "朝露之时" => Some(ArtifactSetKey::ShimenawasReminiscence),
            "祈望之心" => Some(ArtifactSetKey::ShimenawasReminiscence),
            "无常之面" => Some(ArtifactSetKey::ShimenawasReminiscence),
            "荣花之期" => Some(ArtifactSetKey::HuskOfOpulentDreams),
            "华馆之羽" => Some(ArtifactSetKey::HuskOfOpulentDreams),
            "众生之谣" => Some(ArtifactSetKey::HuskOfOpulentDreams),
            "梦醒之瓢" => Some(ArtifactSetKey::HuskOfOpulentDreams),
            "形骸之笠" => Some(ArtifactSetKey::HuskOfOpulentDreams),
            "海染之花" => Some(ArtifactSetKey::OceanHuedClam),
            "渊宫之羽" => Some(ArtifactSetKey::OceanHuedClam),
            "离别之贝" => Some(ArtifactSetKey::OceanHuedClam),
            "真珠之笼" => Some(ArtifactSetKey::OceanHuedClam),
            "海祇之冠" => Some(ArtifactSetKey::OceanHuedClam),
            "生灵之华" | "阳辔之遗" | "潜光片羽" | "结契之刻" | "虺雷之姿" => {
                Some(ArtifactSetKey::VermillionHereafter)
            }
            "魂香之花" | "祝祀之凭" | "垂玉之叶" | "涌泉之盏" | "浮溯之珏" => {
                Some(ArtifactSetKey::EchoesOfAnOffering)
            }
            "迷宫的游人" | "翠蔓的智者" | "贤智的定期" | "迷误者之灯" | "月桂的宝冠" => {
                Some(ArtifactSetKey::DeepwoodMemories)
            }
            "梦中的铁花" | "裁断的翎羽" | "沉金的岁月" | "如蜜的终宴" | "沙王的投影" => {
                Some(ArtifactSetKey::GildedDreams)
            }
            "流沙贵嗣的遗宝"
            | "黄金邦国的结末"
            | "众王之都的开端"
            | "失落迷途的机芯"
            | "迷醉长梦的守护" => Some(ArtifactSetKey::DesertPavilionChronicle),
            "紫晶的花冠" | "谢落的筵席" | "月女的华彩" | "凝结的时刻" | "守秘的魔瓶" => {
                Some(ArtifactSetKey::FlowerOfParadiseLost)
            }
            "恶龙的单片镜"
            | "坏巫师的羽杖"
            | "旅途中的鲜花"
            | "水仙的时时刻刻"
            | "勇者们的茶会" => Some(ArtifactSetKey::NymphsDream),
            "灵光明烁之心" | "琦色灵彩之羽" | "灵光源起之蕊" | "久远花落之时" | "无边酣乐之筵" => {
                Some(ArtifactSetKey::VourukashasGlow)
            }
            "猎人的胸花" | "杰作的序曲" | "裁判的时刻" | "遗忘的容器" | "老兵的容颜" => {
                Some(ArtifactSetKey::MarechausseeHunter)
            }
            "黄金乐曲的变奏"
            | "黄金飞鸟的落羽"
            | "黄金时代的先声"
            | "黄金之夜的喧嚣"
            | "黄金剧团的奖赏" => Some(ArtifactSetKey::GoldenTroupe),
            "昔时传奏之诗" | "昔时浮想之思" | "昔时遗落之誓" | "昔时回映之音" | "昔时应许之梦" => {
                Some(ArtifactSetKey::SongOfDaysPast)
            }
            "慈爱的淑女帽" | "诚恳的蘸水笔" | "无私的妆饰花" | "忠实的砂时计" | "慷慨的墨水瓶" => {
                Some(ArtifactSetKey::NighttimeWhispersInTheEchoingWoods)
            }

            _ => None,
        }
    }
}

impl ArtifactSlotKey {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSlotKey> {
        // let s = match get_real_artifact_name_chs(s) {
        //     Some(v) => v,
        //     None => return None,
        // };
        match s {
            "磐陀裂生之花" => Some(ArtifactSlotKey::Flower),
            "嵯峨群峰之翼" => Some(ArtifactSlotKey::Plume),
            "星罗圭壁之晷" => Some(ArtifactSlotKey::Sands),
            "星罗圭璧之晷" => Some(ArtifactSlotKey::Sands),
            "巉岩琢塑之樽" => Some(ArtifactSlotKey::Goblet),
            "不动玄石之相" => Some(ArtifactSlotKey::Circlet),
            "历经风雪的思念" => Some(ArtifactSlotKey::Flower),
            "摧冰而行的执望" => Some(ArtifactSlotKey::Plume),
            "冰雪故园的终期" => Some(ArtifactSlotKey::Sands),
            "遍结寒霜的傲骨" => Some(ArtifactSlotKey::Goblet),
            "破冰踏雪的回音" => Some(ArtifactSlotKey::Circlet),
            "染血的铁之心" => Some(ArtifactSlotKey::Flower),
            "染血的黑之羽" => Some(ArtifactSlotKey::Plume),
            "骑士染血之时" => Some(ArtifactSlotKey::Sands),
            "染血骑士之杯" => Some(ArtifactSlotKey::Goblet),
            "染血的铁假面" => Some(ArtifactSlotKey::Circlet),
            "魔女的炎之花" => Some(ArtifactSlotKey::Flower),
            "魔女常燃之羽" => Some(ArtifactSlotKey::Plume),
            "魔女破灭之时" => Some(ArtifactSlotKey::Sands),
            "魔女的心之火" => Some(ArtifactSlotKey::Goblet),
            "焦灼的魔女帽" => Some(ArtifactSlotKey::Circlet),
            "角斗士的留恋" => Some(ArtifactSlotKey::Flower),
            "角斗士的归宿" => Some(ArtifactSlotKey::Plume),
            "角斗士的希冀" => Some(ArtifactSlotKey::Sands),
            "角斗士的酣醉" => Some(ArtifactSlotKey::Goblet),
            "角斗士的凯旋" => Some(ArtifactSlotKey::Circlet),
            "饰金胸花" => Some(ArtifactSlotKey::Flower),
            "追忆之风" => Some(ArtifactSlotKey::Plume),
            "坚铜罗盘" => Some(ArtifactSlotKey::Sands),
            "沉波之盏" => Some(ArtifactSlotKey::Goblet),
            "酒渍船帽" => Some(ArtifactSlotKey::Circlet),
            "渡火者的决绝" => Some(ArtifactSlotKey::Flower),
            "渡火者的解脱" => Some(ArtifactSlotKey::Plume),
            "渡火者的煎熬" => Some(ArtifactSlotKey::Sands),
            "渡火者的醒悟" => Some(ArtifactSlotKey::Goblet),
            "渡火者的智慧" => Some(ArtifactSlotKey::Circlet),
            "远方的少女之心" => Some(ArtifactSlotKey::Flower),
            "少女飘摇的思念" => Some(ArtifactSlotKey::Plume),
            "少女苦短的良辰" => Some(ArtifactSlotKey::Sands),
            "少女片刻的闲暇" => Some(ArtifactSlotKey::Goblet),
            "少女易逝的芳颜" => Some(ArtifactSlotKey::Circlet),
            "宗室之花" => Some(ArtifactSlotKey::Flower),
            "宗室之翎" => Some(ArtifactSlotKey::Plume),
            "宗室时计" => Some(ArtifactSlotKey::Sands),
            "宗室银瓮" => Some(ArtifactSlotKey::Goblet),
            "宗室面具" => Some(ArtifactSlotKey::Circlet),
            "夏祭之花" => Some(ArtifactSlotKey::Flower),
            "夏祭终末" => Some(ArtifactSlotKey::Plume),
            "夏祭之刻" => Some(ArtifactSlotKey::Sands),
            "夏祭水玉" => Some(ArtifactSlotKey::Goblet),
            "夏祭之面" => Some(ArtifactSlotKey::Circlet),
            "平雷之心" => Some(ArtifactSlotKey::Flower),
            "平雷之羽" => Some(ArtifactSlotKey::Plume),
            "平雷之刻" => Some(ArtifactSlotKey::Sands),
            "平雷之器" => Some(ArtifactSlotKey::Goblet),
            "平雷之冠" => Some(ArtifactSlotKey::Circlet),
            "雷鸟的怜悯" => Some(ArtifactSlotKey::Flower),
            "雷灾的孑遗" => Some(ArtifactSlotKey::Plume),
            "雷霆的时计" => Some(ArtifactSlotKey::Sands),
            "降雷的凶兆" => Some(ArtifactSlotKey::Goblet),
            "唤雷的头冠" => Some(ArtifactSlotKey::Circlet),
            "野花记忆的绿野" => Some(ArtifactSlotKey::Flower),
            "猎人青翠的箭羽" => Some(ArtifactSlotKey::Plume),
            "翠绿猎人的笃定" => Some(ArtifactSlotKey::Sands),
            "翠绿猎人的容器" => Some(ArtifactSlotKey::Goblet),
            "翠绿的猎人之冠" => Some(ArtifactSlotKey::Circlet),
            "乐团的晨光" => Some(ArtifactSlotKey::Flower),
            "琴师的箭羽" => Some(ArtifactSlotKey::Plume),
            "终幕的时计" => Some(ArtifactSlotKey::Sands),
            "终末的时计" => Some(ArtifactSlotKey::Sands),
            "吟游者之壶" => Some(ArtifactSlotKey::Goblet),
            "指挥的礼帽" => Some(ArtifactSlotKey::Circlet),
            "战狂的蔷薇" => Some(ArtifactSlotKey::Flower),
            "战狂的翎羽" => Some(ArtifactSlotKey::Plume),
            "战狂的时计" => Some(ArtifactSlotKey::Sands),
            "战狂的骨杯" => Some(ArtifactSlotKey::Goblet),
            "战狂的鬼面" => Some(ArtifactSlotKey::Circlet),
            "勇士的勋章" => Some(ArtifactSlotKey::Flower),
            "勇士的期许" => Some(ArtifactSlotKey::Plume),
            "勇士的坚毅" => Some(ArtifactSlotKey::Sands),
            "勇士的壮行" => Some(ArtifactSlotKey::Goblet),
            "勇士的冠冕" => Some(ArtifactSlotKey::Circlet),
            "守护之花" => Some(ArtifactSlotKey::Flower),
            "守护徽印" => Some(ArtifactSlotKey::Plume),
            "守护座钟" => Some(ArtifactSlotKey::Sands),
            "守护之皿" => Some(ArtifactSlotKey::Goblet),
            "守护束带" => Some(ArtifactSlotKey::Circlet),
            "流放者之花" => Some(ArtifactSlotKey::Flower),
            "流放者之羽" => Some(ArtifactSlotKey::Plume),
            "流放者怀表" => Some(ArtifactSlotKey::Sands),
            "流放者之杯" => Some(ArtifactSlotKey::Goblet),
            "流放者头冠" => Some(ArtifactSlotKey::Circlet),
            "赌徒的胸花" => Some(ArtifactSlotKey::Flower),
            "赌徒的羽饰" => Some(ArtifactSlotKey::Plume),
            "赌徒的怀表" => Some(ArtifactSlotKey::Sands),
            "赌徒的骰盅" => Some(ArtifactSlotKey::Goblet),
            "赌徒的耳环" => Some(ArtifactSlotKey::Circlet),
            "教官的胸花" => Some(ArtifactSlotKey::Flower),
            "教官的羽饰" => Some(ArtifactSlotKey::Plume),
            "教官的怀表" => Some(ArtifactSlotKey::Sands),
            "教官的茶杯" => Some(ArtifactSlotKey::Goblet),
            "教官的帽子" => Some(ArtifactSlotKey::Circlet),
            "武人的红花" => Some(ArtifactSlotKey::Flower),
            "武人的羽饰" => Some(ArtifactSlotKey::Plume),
            "武人的水漏" => Some(ArtifactSlotKey::Sands),
            "武人的酒杯" => Some(ArtifactSlotKey::Goblet),
            "武人的头巾" => Some(ArtifactSlotKey::Circlet),
            "祭水礼冠" => Some(ArtifactSlotKey::Circlet),
            "祭火礼冠" => Some(ArtifactSlotKey::Circlet),
            "祭雷礼冠" => Some(ArtifactSlotKey::Circlet),
            "祭冰礼冠" => Some(ArtifactSlotKey::Circlet),
            "故人之心" => Some(ArtifactSlotKey::Flower),
            "归乡之羽" => Some(ArtifactSlotKey::Plume),
            "逐光之石" => Some(ArtifactSlotKey::Sands),
            "异国之盏" => Some(ArtifactSlotKey::Goblet),
            "感别之冠" => Some(ArtifactSlotKey::Circlet),
            "学士的书签" => Some(ArtifactSlotKey::Flower),
            "学士的羽笔" => Some(ArtifactSlotKey::Plume),
            "学士的时钟" => Some(ArtifactSlotKey::Sands),
            "学士的墨杯" => Some(ArtifactSlotKey::Goblet),
            "学士的镜片" => Some(ArtifactSlotKey::Circlet),
            "奇迹之花" => Some(ArtifactSlotKey::Flower),
            "奇迹之羽" => Some(ArtifactSlotKey::Plume),
            "奇迹之沙" => Some(ArtifactSlotKey::Sands),
            "奇迹之杯" => Some(ArtifactSlotKey::Goblet),
            "奇迹耳坠" => Some(ArtifactSlotKey::Circlet),
            "冒险家之花" => Some(ArtifactSlotKey::Flower),
            "冒险家尾羽" => Some(ArtifactSlotKey::Plume),
            "冒险家怀表" => Some(ArtifactSlotKey::Sands),
            "冒险家金杯" => Some(ArtifactSlotKey::Goblet),
            "冒险家头带" => Some(ArtifactSlotKey::Circlet),
            "幸运儿绿花" => Some(ArtifactSlotKey::Flower),
            "幸运儿鹰羽" => Some(ArtifactSlotKey::Plume),
            "幸运儿沙漏" => Some(ArtifactSlotKey::Sands),
            "幸运儿之杯" => Some(ArtifactSlotKey::Goblet),
            "幸运儿银冠" => Some(ArtifactSlotKey::Circlet),
            "游医的银莲" => Some(ArtifactSlotKey::Flower),
            "游医的枭羽" => Some(ArtifactSlotKey::Plume),
            "游医的怀钟" => Some(ArtifactSlotKey::Sands),
            "游医的药壶" => Some(ArtifactSlotKey::Goblet),
            "游医的方巾" => Some(ArtifactSlotKey::Circlet),
            "勋绩之花" => Some(ArtifactSlotKey::Flower),
            "昭武翎羽" => Some(ArtifactSlotKey::Plume),
            "金铜时晷" => Some(ArtifactSlotKey::Sands),
            "盟誓金爵" => Some(ArtifactSlotKey::Goblet),
            "将帅兜鍪" => Some(ArtifactSlotKey::Circlet),
            "无垢之花" => Some(ArtifactSlotKey::Flower),
            "贤医之羽" => Some(ArtifactSlotKey::Plume),
            "停摆之刻" => Some(ArtifactSlotKey::Sands),
            "超越之盏" => Some(ArtifactSlotKey::Goblet),
            "嗤笑之面" => Some(ArtifactSlotKey::Circlet),
            "明威之镡" => Some(ArtifactSlotKey::Flower),
            "切落之羽" => Some(ArtifactSlotKey::Plume),
            "雷云之笼" => Some(ArtifactSlotKey::Sands),
            "绯花之壶" => Some(ArtifactSlotKey::Goblet),
            "华饰之兜" => Some(ArtifactSlotKey::Circlet),
            "羁缠之花" => Some(ArtifactSlotKey::Flower),
            "思忆之矢" => Some(ArtifactSlotKey::Plume),
            "朝露之时" => Some(ArtifactSlotKey::Sands),
            "祈望之心" => Some(ArtifactSlotKey::Goblet),
            "无常之面" => Some(ArtifactSlotKey::Circlet),
            "荣花之期" => Some(ArtifactSlotKey::Flower),
            "华馆之羽" => Some(ArtifactSlotKey::Plume),
            "众生之谣" => Some(ArtifactSlotKey::Sands),
            "梦醒之瓢" => Some(ArtifactSlotKey::Goblet),
            "形骸之笠" => Some(ArtifactSlotKey::Circlet),
            "海染之花" => Some(ArtifactSlotKey::Flower),
            "渊宫之羽" => Some(ArtifactSlotKey::Plume),
            "离别之贝" => Some(ArtifactSlotKey::Sands),
            "真珠之笼" => Some(ArtifactSlotKey::Goblet),
            "海祇之冠" => Some(ArtifactSlotKey::Circlet),
            "生灵之华" => Some(ArtifactSlotKey::Flower),
            "阳辔之遗" => Some(ArtifactSlotKey::Sands),
            "潜光片羽" => Some(ArtifactSlotKey::Plume),
            "结契之刻" => Some(ArtifactSlotKey::Goblet),
            "虺雷之姿" => Some(ArtifactSlotKey::Circlet),
            "魂香之花" => Some(ArtifactSlotKey::Flower),
            "祝祀之凭" => Some(ArtifactSlotKey::Sands),
            "垂玉之叶" => Some(ArtifactSlotKey::Plume),
            "涌泉之盏" => Some(ArtifactSlotKey::Goblet),
            "浮溯之珏" => Some(ArtifactSlotKey::Circlet),
            "迷宫的游人" => Some(ArtifactSlotKey::Flower),
            "翠蔓的智者" => Some(ArtifactSlotKey::Plume),
            "贤智的定期" => Some(ArtifactSlotKey::Sands),
            "迷误者之灯" => Some(ArtifactSlotKey::Goblet),
            "月桂的宝冠" => Some(ArtifactSlotKey::Circlet),
            "梦中的铁花" => Some(ArtifactSlotKey::Flower),
            "裁断的翎羽" => Some(ArtifactSlotKey::Plume),
            "沉金的岁月" => Some(ArtifactSlotKey::Sands),
            "如蜜的终宴" => Some(ArtifactSlotKey::Goblet),
            "沙王的投影" => Some(ArtifactSlotKey::Circlet),
            "流沙贵嗣的遗宝" => Some(ArtifactSlotKey::Circlet),
            "黄金邦国的结末" => Some(ArtifactSlotKey::Plume),
            "众王之都的开端" => Some(ArtifactSlotKey::Flower),
            "失落迷途的机芯" => Some(ArtifactSlotKey::Sands),
            "迷醉长梦的守护" => Some(ArtifactSlotKey::Goblet),
            "紫晶的花冠" => Some(ArtifactSlotKey::Circlet),
            "谢落的筵席" => Some(ArtifactSlotKey::Plume),
            "月女的华彩" => Some(ArtifactSlotKey::Flower),
            "凝结的时刻" => Some(ArtifactSlotKey::Sands),
            "守秘的魔瓶" => Some(ArtifactSlotKey::Goblet),
            "旅途中的鲜花" => Some(ArtifactSlotKey::Flower),
            "坏巫师的羽杖" => Some(ArtifactSlotKey::Plume),
            "水仙的时时刻刻" => Some(ArtifactSlotKey::Sands),
            "勇者们的茶会" => Some(ArtifactSlotKey::Goblet),
            "恶龙的单片镜" => Some(ArtifactSlotKey::Circlet),
            "灵光源起之蕊" => Some(ArtifactSlotKey::Flower),
            "琦色灵彩之羽" => Some(ArtifactSlotKey::Plume),
            "久远花落之时" => Some(ArtifactSlotKey::Sands),
            "无边酣乐之筵" => Some(ArtifactSlotKey::Goblet),
            "灵光明烁之心" => Some(ArtifactSlotKey::Circlet),
            "猎人的胸花" => Some(ArtifactSlotKey::Flower),
            "杰作的序曲" => Some(ArtifactSlotKey::Plume),
            "裁判的时刻" => Some(ArtifactSlotKey::Sands),
            "遗忘的容器" => Some(ArtifactSlotKey::Goblet),
            "老兵的容颜" => Some(ArtifactSlotKey::Circlet),
            "黄金乐曲的变奏" => Some(ArtifactSlotKey::Flower),
            "黄金飞鸟的落羽" => Some(ArtifactSlotKey::Plume),
            "黄金时代的先声" => Some(ArtifactSlotKey::Sands),
            "黄金之夜的喧嚣" => Some(ArtifactSlotKey::Goblet),
            "黄金剧团的奖赏" => Some(ArtifactSlotKey::Circlet),
            "昔时传奏之诗" => Some(ArtifactSlotKey::Circlet),
            "昔时浮想之思" => Some(ArtifactSlotKey::Plume),
            "昔时遗落之誓" => Some(ArtifactSlotKey::Flower),
            "昔时回映之音" => Some(ArtifactSlotKey::Sands),
            "昔时应许之梦" => Some(ArtifactSlotKey::Goblet),
            "慈爱的淑女帽" => Some(ArtifactSlotKey::Circlet),
            "诚恳的蘸水笔" => Some(ArtifactSlotKey::Plume),
            "无私的妆饰花" => Some(ArtifactSlotKey::Flower),
            "忠实的砂时计" => Some(ArtifactSlotKey::Sands),
            "慷慨的墨水瓶" => Some(ArtifactSlotKey::Goblet),
            _ => None,
        }
    }
}

impl CharacterKey {
    pub fn from_zh_cn(s: &str) -> Option<CharacterKey> {
        match s {
            "神里绫华" => Some(CharacterKey::KamisatoAyaka),
            "琴" => Some(CharacterKey::Jean),
            "旅行者" => Some(CharacterKey::Traveler),
            "丽莎" => Some(CharacterKey::Lisa),
            "芭芭拉" => Some(CharacterKey::Barbara),
            "凯亚" => Some(CharacterKey::Kaeya),
            "迪卢克" => Some(CharacterKey::Diluc),
            "雷泽" => Some(CharacterKey::Razor),
            "安柏" => Some(CharacterKey::Amber),
            "温迪" => Some(CharacterKey::Venti),
            "香菱" => Some(CharacterKey::Xiangling),
            "北斗" => Some(CharacterKey::Beidou),
            "行秋" => Some(CharacterKey::Xingqiu),
            "魈" => Some(CharacterKey::Xiao),
            "凝光" => Some(CharacterKey::Ningguang),
            "可莉" => Some(CharacterKey::Klee),
            "钟离" => Some(CharacterKey::Zhongli),
            "菲谢尔" => Some(CharacterKey::Fischl),
            "班尼特" => Some(CharacterKey::Bennett),
            "达达利亚" => Some(CharacterKey::Tartaglia),
            "诺艾尔" => Some(CharacterKey::Noelle),
            "七七" => Some(CharacterKey::Qiqi),
            "重云" => Some(CharacterKey::Chongyun),
            "甘雨" => Some(CharacterKey::Ganyu),
            "阿贝多" => Some(CharacterKey::Albedo),
            "迪奥娜" => Some(CharacterKey::Diona),
            "莫娜" => Some(CharacterKey::Mona),
            "刻晴" => Some(CharacterKey::Keqing),
            "砂糖" => Some(CharacterKey::Sucrose),
            "辛焱" => Some(CharacterKey::Xinyan),
            "罗莎莉亚" => Some(CharacterKey::Rosaria),
            "胡桃" => Some(CharacterKey::HuTao),
            "枫原万叶" => Some(CharacterKey::KaedeharaKazuha),
            "烟绯" => Some(CharacterKey::Yanfei),
            "宵宫" => Some(CharacterKey::Yoimiya),
            "托马" => Some(CharacterKey::Thoma),
            "优菈" => Some(CharacterKey::Eula),
            "雷电将军" => Some(CharacterKey::RaidenShogun),
            "早柚" => Some(CharacterKey::Sayu),
            "珊瑚宫心海" => Some(CharacterKey::SangonomiyaKokomi),
            "五郎" => Some(CharacterKey::Gorou),
            "九条裟罗" => Some(CharacterKey::KujouSara),
            "荒泷一斗" => Some(CharacterKey::AratakiItto),
            "八重神子" => Some(CharacterKey::YaeMiko),
            "鹿野院平藏" => Some(CharacterKey::ShikanoinHeizou),
            "夜兰" => Some(CharacterKey::Yelan),
            "绮良良" => Some(CharacterKey::Kirara),
            "埃洛伊" => Some(CharacterKey::Aloy),
            "申鹤" => Some(CharacterKey::Shenhe),
            "云堇" => Some(CharacterKey::YunJin),
            "久岐忍" => Some(CharacterKey::KukiShinobu),
            "神里绫人" => Some(CharacterKey::KamisatoAyato),
            "柯莱" => Some(CharacterKey::Collei),
            "多莉" => Some(CharacterKey::Dori),
            "提纳里" => Some(CharacterKey::Tighnari),
            "妮露" => Some(CharacterKey::Nilou),
            "赛诺" => Some(CharacterKey::Cyno),
            "坎蒂丝" => Some(CharacterKey::Candace),
            "纳西妲" => Some(CharacterKey::Nahida),
            "莱依拉" => Some(CharacterKey::Layla),
            "流浪者" => Some(CharacterKey::Wanderer),
            "珐露珊" => Some(CharacterKey::Faruzan),
            "瑶瑶" => Some(CharacterKey::Yaoyao),
            "艾尔海森" => Some(CharacterKey::Alhaitham),
            "迪希雅" => Some(CharacterKey::Dehya),
            "米卡" => Some(CharacterKey::Mika),
            "卡维" => Some(CharacterKey::Kaveh),
            "白术" => Some(CharacterKey::Baizhu),
            "琳妮特" => Some(CharacterKey::Lynette),
            "林尼" => Some(CharacterKey::Lyney),
            "菲米尼" => Some(CharacterKey::Freminet),
            "莱欧斯利" => Some(CharacterKey::Wriothesley),
            "那维莱特" => Some(CharacterKey::Neuvillette),
            "夏洛蒂" => Some(CharacterKey::Charlotte),
            "芙宁娜" => Some(CharacterKey::Furina),
            "夏沃蕾" => Some(CharacterKey::Chevreuse),
            "娜维娅" => Some(CharacterKey::Navia),
            "嘉明" => Some(CharacterKey::Gaming),
            "闲云" => Some(CharacterKey::Liuyun),
            _ => None,
        }
    }
}
