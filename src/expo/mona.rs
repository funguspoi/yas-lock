use anyhow::Result;
use std::convert::From;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::artifact::internal_artifact::{
    ArtifactSetKey, ArtifactSlotKey, ArtifactStat, ArtifactStatKey, CharacterKey, InternalArtifact,
};

type MonaArtifact = InternalArtifact;

impl ArtifactStatKey {
    pub fn to_mona(&self) -> String {
        let temp = match self {
            ArtifactStatKey::HealingBonus => "cureEffect",
            ArtifactStatKey::CriticalDamage => "criticalDamage",
            ArtifactStatKey::Critical => "critical",
            ArtifactStatKey::Atk => "attackStatic",
            ArtifactStatKey::AtkPercentage => "attackPercentage",
            ArtifactStatKey::ElementalMastery => "elementalMastery",
            ArtifactStatKey::Recharge => "recharge",
            ArtifactStatKey::HpPercentage => "lifePercentage",
            ArtifactStatKey::Hp => "lifeStatic",
            ArtifactStatKey::DefPercentage => "defendPercentage",
            ArtifactStatKey::Def => "defendStatic",
            ArtifactStatKey::ElectroBonus => "thunderBonus",
            ArtifactStatKey::PyroBonus => "fireBonus",
            ArtifactStatKey::HydroBonus => "waterBonus",
            ArtifactStatKey::CryoBonus => "iceBonus",
            ArtifactStatKey::AnemoBonus => "windBonus",
            ArtifactStatKey::GeoBonus => "rockBonus",
            ArtifactStatKey::PhysicalBonus => "physicalBonus",
            ArtifactStatKey::DendroBonus => "dendroBonus",
        };
        String::from(temp)
    }
}

impl ArtifactSetKey {
    pub fn to_mona(&self) -> String {
        let same = self.to_string();
        let temp = match self {
            ArtifactSetKey::ArchaicPetra => "archaicPetra",
            ArtifactSetKey::HeartOfDepth => "heartOfDepth",
            ArtifactSetKey::BlizzardStrayer => "blizzardStrayer",
            ArtifactSetKey::RetracingBolide => "retracingBolide",
            ArtifactSetKey::NoblesseOblige => "noblesseOblige",
            ArtifactSetKey::GladiatorsFinale => "gladiatorFinale",
            ArtifactSetKey::MaidenBeloved => "maidenBeloved",
            ArtifactSetKey::ViridescentVenerer => "viridescentVenerer",
            ArtifactSetKey::Lavawalker => "lavaWalker",
            ArtifactSetKey::CrimsonWitchOfFlames => "crimsonWitch",
            ArtifactSetKey::Thundersoother => "thunderSmoother",
            ArtifactSetKey::ThunderingFury => "thunderingFury",
            ArtifactSetKey::BloodstainedChivalry => "bloodstainedChivalry",
            ArtifactSetKey::WanderersTroupe => "wandererTroupe",
            ArtifactSetKey::Scholar => "scholar",
            ArtifactSetKey::Gambler => "gambler",
            ArtifactSetKey::TinyMiracle => "tinyMiracle",
            ArtifactSetKey::MartialArtist => "martialArtist",
            ArtifactSetKey::BraveHeart => "braveHeart",
            ArtifactSetKey::ResolutionOfSojourner => "resolutionOfSojourner",
            ArtifactSetKey::DefenderWill => "defenderWill",
            ArtifactSetKey::Berserker => "berserker",
            ArtifactSetKey::Instructor => "instructor",
            ArtifactSetKey::Exile => "exile",
            ArtifactSetKey::Adventurer => "adventurer",
            ArtifactSetKey::LuckyDog => "luckyDog",
            ArtifactSetKey::TravelingDoctor => "travelingDoctor",
            ArtifactSetKey::PrayersForWisdom => "prayersForWisdom",
            ArtifactSetKey::PrayersToSpringtime => "prayersToSpringtime",
            ArtifactSetKey::PrayersForIllumination => "prayersForIllumination",
            ArtifactSetKey::PrayersForDestiny => "prayersForDestiny",
            ArtifactSetKey::PaleFlame => "paleFlame",
            ArtifactSetKey::TenacityOfTheMillelith => "tenacityOfTheMillelith",
            ArtifactSetKey::EmblemOfSeveredFate => "emblemOfSeveredFate",
            ArtifactSetKey::ShimenawasReminiscence => "shimenawaReminiscence",
            ArtifactSetKey::HuskOfOpulentDreams => "huskOfOpulentDreams",
            ArtifactSetKey::OceanHuedClam => "oceanHuedClam",
            _ => same.as_str(),
        };
        String::from(temp)
    }
}

impl ArtifactSlotKey {
    pub fn to_mona(&self) -> String {
        let temp = match self {
            ArtifactSlotKey::Flower => "flower",
            ArtifactSlotKey::Plume => "feather",
            ArtifactSlotKey::Sands => "sand",
            ArtifactSlotKey::Goblet => "cup",
            ArtifactSlotKey::Circlet => "head",
        };
        String::from(temp)
    }
}

impl CharacterKey {
    pub fn to_mona(&self) -> String {
        let temp = match self {
            CharacterKey::KamisatoAyaka => "神里绫华",
            CharacterKey::Jean => "琴",
            CharacterKey::Traveler => "旅行者",
            CharacterKey::Lisa => "丽莎",
            CharacterKey::Barbara => "芭芭拉",
            CharacterKey::Kaeya => "凯亚",
            CharacterKey::Diluc => "迪卢克",
            CharacterKey::Razor => "雷泽",
            CharacterKey::Amber => "安柏",
            CharacterKey::Venti => "温迪",
            CharacterKey::Xiangling => "香菱",
            CharacterKey::Beidou => "北斗",
            CharacterKey::Xingqiu => "行秋",
            CharacterKey::Xiao => "魈",
            CharacterKey::Ningguang => "凝光",
            CharacterKey::Klee => "可莉",
            CharacterKey::Zhongli => "钟离",
            CharacterKey::Fischl => "菲谢尔",
            CharacterKey::Bennett => "班尼特",
            CharacterKey::Tartaglia => "达达利亚",
            CharacterKey::Noelle => "诺艾尔",
            CharacterKey::Qiqi => "七七",
            CharacterKey::Chongyun => "重云",
            CharacterKey::Ganyu => "甘雨",
            CharacterKey::Albedo => "阿贝多",
            CharacterKey::Diona => "迪奥娜",
            CharacterKey::Mona => "莫娜",
            CharacterKey::Keqing => "刻晴",
            CharacterKey::Sucrose => "砂糖",
            CharacterKey::Xinyan => "辛焱",
            CharacterKey::Rosaria => "罗莎莉亚",
            CharacterKey::HuTao => "胡桃",
            CharacterKey::KaedeharaKazuha => "枫原万叶",
            CharacterKey::Yanfei => "烟绯",
            CharacterKey::Yoimiya => "宵宫",
            CharacterKey::Thoma => "托马",
            CharacterKey::Eula => "优菈",
            CharacterKey::RaidenShogun => "雷电将军",
            CharacterKey::Sayu => "早柚",
            CharacterKey::SangonomiyaKokomi => "珊瑚宫心海",
            CharacterKey::Gorou => "五郎",
            CharacterKey::KujouSara => "九条裟罗",
            CharacterKey::AratakiItto => "荒泷一斗",
            CharacterKey::YaeMiko => "八重神子",
            CharacterKey::ShikanoinHeizou => "鹿野院平藏",
            CharacterKey::Yelan => "夜兰",
            CharacterKey::Kirara => "绮良良",
            CharacterKey::Aloy => "埃洛伊",
            CharacterKey::Shenhe => "申鹤",
            CharacterKey::YunJin => "云堇",
            CharacterKey::KukiShinobu => "久岐忍",
            CharacterKey::KamisatoAyato => "神里绫人",
            CharacterKey::Collei => "柯莱",
            CharacterKey::Dori => "多莉",
            CharacterKey::Tighnari => "提纳里",
            CharacterKey::Nilou => "妮露",
            CharacterKey::Cyno => "赛诺",
            CharacterKey::Candace => "坎蒂丝",
            CharacterKey::Nahida => "纳西妲",
            CharacterKey::Layla => "莱依拉",
            CharacterKey::Wanderer => "流浪者",
            CharacterKey::Faruzan => "珐露珊",
            CharacterKey::Yaoyao => "瑶瑶",
            CharacterKey::Alhaitham => "艾尔海森",
            CharacterKey::Dehya => "迪希雅",
            CharacterKey::Mika => "米卡",
            CharacterKey::Kaveh => "卡维",
            CharacterKey::Baizhu => "白术",
            CharacterKey::Lynette => "琳妮特",
            CharacterKey::Lyney => "林尼",
            CharacterKey::Freminet => "菲米尼",
            CharacterKey::Wriothesley => "莱欧斯利",
            CharacterKey::Neuvillette => "那维莱特",
            CharacterKey::Charlotte => "夏洛蒂",
            CharacterKey::Furina => "芙宁娜",
            CharacterKey::Chevreuse => "夏沃蕾",
            CharacterKey::Navia => "娜维娅",
        };
        String::from(temp)
    }
}

impl Serialize for ArtifactStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;
        root.serialize_entry("name", &self.key.to_mona())?;
        let value = match self.key {
            ArtifactStatKey::Atk
            | ArtifactStatKey::ElementalMastery
            | ArtifactStatKey::Hp
            | ArtifactStatKey::Def => self.value,
            _ => self.value / 100.0,
        };
        root.serialize_entry("value", &value)?;
        root.end()
    }
}

impl Serialize for MonaArtifact {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(8))?;

        root.serialize_entry("setName", &self.set_key.to_mona())?;
        root.serialize_entry("position", &self.slot_key.to_mona())?;
        root.serialize_entry("mainTag", &self.main_stat)?;

        let mut sub_stats: Vec<&ArtifactStat> = vec![];
        if let Some(ref s) = self.sub_stat_1 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_2 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_3 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_4 {
            sub_stats.push(s);
        }
        // let mut subs = serializer.serialize_seq(Some(sub_stats.len()))?;
        //
        // for i in sub_stats {
        //     subs.serialize_element(i);
        // }
        // subs.end();
        // subs.

        root.serialize_entry("normalTags", &sub_stats)?;
        root.serialize_entry("omit", &false)?;
        root.serialize_entry("level", &self.level)?;
        root.serialize_entry("star", &self.rarity)?;
        let equip = match &self.location {
            Some(l) => l.to_mona(),
            None => String::from(""),
        };
        root.serialize_entry("equip", &equip)?; // TODO: 流浪者

        root.end()
    }
}

pub struct MonaFormat<'a> {
    version: String,
    flower: Vec<&'a MonaArtifact>,
    feather: Vec<&'a MonaArtifact>,
    cup: Vec<&'a MonaArtifact>,
    sand: Vec<&'a MonaArtifact>,
    head: Vec<&'a MonaArtifact>,
}

impl<'a> Serialize for MonaFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(6))?;
        root.serialize_entry("version", &self.version)?;
        root.serialize_entry("flower", &self.flower)?;
        root.serialize_entry("feather", &self.feather)?;
        root.serialize_entry("sand", &self.sand)?;
        root.serialize_entry("cup", &self.cup)?;
        root.serialize_entry("head", &self.head)?;
        root.end()
    }
}

impl<'a> MonaFormat<'a> {
    pub fn new(results: &Vec<InternalArtifact>) -> MonaFormat {
        let mut flower: Vec<&MonaArtifact> = Vec::new();
        let mut feather: Vec<&MonaArtifact> = Vec::new();
        let mut cup: Vec<&MonaArtifact> = Vec::new();
        let mut sand: Vec<&MonaArtifact> = Vec::new();
        let mut head: Vec<&MonaArtifact> = Vec::new();

        for art in results.iter() {
            match art.slot_key {
                ArtifactSlotKey::Flower => flower.push(art),
                ArtifactSlotKey::Plume => feather.push(art),
                ArtifactSlotKey::Sands => sand.push(art),
                ArtifactSlotKey::Goblet => cup.push(art),
                ArtifactSlotKey::Circlet => head.push(art),
            }
        }

        MonaFormat {
            flower,
            feather,
            cup,
            sand,
            head,

            version: String::from("1"),
        }
    }
}
