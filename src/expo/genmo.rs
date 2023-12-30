use crate::artifact::internal_artifact::{
    ArtifactSetKey, ArtifactSlotKey, ArtifactStat, ArtifactStatKey, InternalArtifact,
};
use anyhow::Result;
use serde::ser::{Serialize, SerializeMap, Serializer};

struct GenmoArtifact<'a> {
    artifact: &'a InternalArtifact,
}

impl<'a> Serialize for GenmoArtifact<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let extract_stat_name = |maybe_stat: &Option<ArtifactStat>| match maybe_stat {
            None => "flatATK",
            Some(stat) => stat.key.to_genmo(),
        };

        let extract_stat_value = |maybe_stat: &Option<ArtifactStat>| match maybe_stat {
            None => 0.0,
            Some(stat) => stat.value,
        };

        let artifact = &self.artifact;
        let mut root = serializer.serialize_map(Some(13))?;
        root.serialize_entry("asKey", artifact.set_key.to_genmo())?;
        root.serialize_entry("rarity", &artifact.rarity)?;
        root.serialize_entry("slot", artifact.slot_key.to_genmo())?;
        root.serialize_entry("level", &artifact.level)?;
        root.serialize_entry("mainStat", artifact.main_stat.key.to_genmo())?;
        root.serialize_entry("subStat1Type", &extract_stat_name(&artifact.sub_stat_1))?;
        root.serialize_entry("subStat1Value", &extract_stat_value(&artifact.sub_stat_1))?;
        root.serialize_entry("subStat2Type", &extract_stat_name(&artifact.sub_stat_2))?;
        root.serialize_entry("subStat2Value", &extract_stat_value(&artifact.sub_stat_2))?;
        root.serialize_entry("subStat3Type", &extract_stat_name(&artifact.sub_stat_3))?;
        root.serialize_entry("subStat3Value", &extract_stat_value(&artifact.sub_stat_3))?;
        root.serialize_entry("subStat4Type", &extract_stat_name(&artifact.sub_stat_4))?;
        root.serialize_entry("subStat4Value", &extract_stat_value(&artifact.sub_stat_4))?;
        root.end()
    }
}

impl ArtifactStatKey {
    pub fn to_genmo(&self) -> &'static str {
        match self {
            ArtifactStatKey::HealingBonus => "healing",
            ArtifactStatKey::CriticalDamage => "critDamage",
            ArtifactStatKey::Critical => "critRate",
            ArtifactStatKey::Atk => "flatATK",
            ArtifactStatKey::AtkPercentage => "percentATK",
            ArtifactStatKey::ElementalMastery => "elementalMastery",
            ArtifactStatKey::Recharge => "energyRecharge",
            ArtifactStatKey::HpPercentage => "percentHP",
            ArtifactStatKey::Hp => "flatHP",
            ArtifactStatKey::DefPercentage => "percentDEF",
            ArtifactStatKey::Def => "flatDEF",
            ArtifactStatKey::ElectroBonus => "electroDamage",
            ArtifactStatKey::PyroBonus => "pyroDamage",
            ArtifactStatKey::HydroBonus => "hydroDamage",
            ArtifactStatKey::CryoBonus => "cryoDamage",
            ArtifactStatKey::AnemoBonus => "anemoDamage",
            ArtifactStatKey::GeoBonus => "geoDamage",
            ArtifactStatKey::PhysicalBonus => "physicalDamage",
            ArtifactStatKey::DendroBonus => "dendroDamage",
        }
    }
}

impl ArtifactSlotKey {
    pub fn to_genmo(&self) -> &'static str {
        match self {
            ArtifactSlotKey::Flower => "flower",
            ArtifactSlotKey::Plume => "plume",
            ArtifactSlotKey::Sands => "eon",
            ArtifactSlotKey::Goblet => "goblet",
            ArtifactSlotKey::Circlet => "circlet",
        }
    }
}

impl ArtifactSetKey {
    pub fn to_genmo(&self) -> &'static str {
        match self {
            ArtifactSetKey::ArchaicPetra => "archaic_petra",
            ArtifactSetKey::HeartOfDepth => "heart_of_depth",
            ArtifactSetKey::BlizzardStrayer => "blizzard_walker",
            ArtifactSetKey::RetracingBolide => "retracing_bolide",
            ArtifactSetKey::NoblesseOblige => "noblesse_oblige",
            ArtifactSetKey::GladiatorsFinale => "gladiators_finale",
            ArtifactSetKey::MaidenBeloved => "maiden_beloved",
            ArtifactSetKey::ViridescentVenerer => "viridescent_venerer",
            ArtifactSetKey::Lavawalker => "lavawalker",
            ArtifactSetKey::CrimsonWitchOfFlames => "crimson_witch_of_flames",
            ArtifactSetKey::Thundersoother => "thundersoother",
            ArtifactSetKey::ThunderingFury => "thundering_fury",
            ArtifactSetKey::BloodstainedChivalry => "bloodstained_chivalry",
            ArtifactSetKey::WanderersTroupe => "wanderers_troupe",
            ArtifactSetKey::Scholar => "scholar",
            ArtifactSetKey::Gambler => "gambler",
            ArtifactSetKey::TinyMiracle => "tiny_miracle",
            ArtifactSetKey::MartialArtist => "martial_artist",
            ArtifactSetKey::BraveHeart => "brave_heart",
            ArtifactSetKey::ResolutionOfSojourner => "resolution_of_sojourner",
            ArtifactSetKey::DefenderWill => "defenders_will",
            ArtifactSetKey::Berserker => "berserker",
            ArtifactSetKey::Instructor => "instructor",
            ArtifactSetKey::Exile => "the_exile",
            ArtifactSetKey::PrayersForWisdom => "prayers_of_wisdom",
            ArtifactSetKey::PrayersToSpringtime => "prayers_of_springtime",
            ArtifactSetKey::PrayersForIllumination => "prayers_of_illumination",
            ArtifactSetKey::PrayersForDestiny => "prayers_of_destiny",
            ArtifactSetKey::PaleFlame => "pale_flame",
            ArtifactSetKey::TenacityOfTheMillelith => "tenacity_of_the_millelith",
            ArtifactSetKey::EmblemOfSeveredFate => "seal_of_insulation",
            ArtifactSetKey::ShimenawasReminiscence => "reminiscence_of_shime",
            ArtifactSetKey::HuskOfOpulentDreams => "husk_of_opulent_dreams",
            ArtifactSetKey::OceanHuedClam => "divine_chorus",
            ArtifactSetKey::VermillionHereafter => "vermillion_hereafter",
            ArtifactSetKey::EchoesOfAnOffering => "echoes_of_an_offering",
            ArtifactSetKey::DeepwoodMemories => "deepwood_memories",
            ArtifactSetKey::GildedDreams => "gilded_dreams",
            ArtifactSetKey::DesertPavilionChronicle => "desert_pavilion_chronicle",
            ArtifactSetKey::FlowerOfParadiseLost => "flower_of_paradise_lost",
            ArtifactSetKey::NymphsDream => "nymphs_dream",
            ArtifactSetKey::VourukashasGlow => "vourukashas_glow",
            ArtifactSetKey::MarechausseeHunter => "marechaussee_hunter",
            ArtifactSetKey::GoldenTroupe => "golden_troupe",
            ArtifactSetKey::SongOfDaysPast => "song_of_days_past",
            ArtifactSetKey::NighttimeWhispersInTheEchoingWoods => {
                "nighttime_whispers_in_the_echoing_woods"
            }
            // Not supported by Mingyulab
            ArtifactSetKey::Adventurer => unreachable!(),
            ArtifactSetKey::LuckyDog => unreachable!(),
            ArtifactSetKey::TravelingDoctor => unreachable!(),
        }
    }
}

pub struct GenmoFormat<'a> {
    artifacts: Vec<GenmoArtifact<'a>>,
}

impl<'a> GenmoFormat<'a> {
    pub fn new(results: &'a Vec<InternalArtifact>) -> GenmoFormat {
        let artifacts: Vec<GenmoArtifact<'a>> = results
            .into_iter()
            .filter(|artifact| {
                artifact.set_key != ArtifactSetKey::Adventurer
                    && artifact.set_key != ArtifactSetKey::LuckyDog
                    && artifact.set_key != ArtifactSetKey::TravelingDoctor
            })
            .map(|artifact| GenmoArtifact { artifact })
            .collect();
        GenmoFormat { artifacts }
    }
}

impl<'a> Serialize for GenmoFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self.artifacts)
    }
}
