use crate::rand::Rand;

pub fn gen_description(rand: &mut Rand) -> String {
    let clothing = rand.pick(&CLOTHING);
    let speech = rand.pick(&SPEECH);
    let physique = rand.pick(&PHYSIQUE);
    let features = rand.pick(&FEATURES);
    let hair = rand.pick(&HAIR);
    let virtue = rand.pick(&VIRTUE);
    let vice = rand.pick(&VICE);
    let misfortune = rand.pick(&MISFORTUNE);
    format!(
        "Wears {clothing} clothes, and {speech}. 
        Has a {physique} physique, {features} and {hair} hair.
        Is {virtue}, but {vice}. {misfortune} in the past."
    )
}

const PHYSIQUE: [&str; 10] = [
    "athletic",
    "brawny",
    "flabby",
    "lanky",
    "rugged",
    "scrawny",
    "short",
    "statuesque",
    "stout",
    "towering",
];

const FEATURES: [&str; 16] = [
    "bony features",
    "a broken nose",
    "pale skin",
    "flawless skin",
    "rat-like features",
    "sharp features",
    "a square jaw",
    "sunken features",
    "a birthmark on their face",
    "dark skin",
    "pockmarked skin",
    "rosy cheeks",
    "soft features",
    "tanned skin",
    "a tattooes on their face",
    "weathered skin",
];

const HAIR: [&str; 9] = [
    "no", "braided", "curly", "filthy", "frizzy", "long", "oily", "wavy", "wispy",
];

const SPEECH: [&str; 10] = [
    "speaks bluntly",
    "has a booming voice",
    "speaks cryptically",
    "often drones on in conversation",
    "speaks formally",
    "has a gravelly voice",
    "speaks precisely",
    "has a squeaky voice",
    "often stutters when speaking",
    "speaks mostly in whispers",
];
const CLOTHING: [&str; 10] = [
    "antique", "bloody", "elegant", "filthy", "foreign", "frayed", "frumpy", "livery", "rancid",
    "soiled",
];
const VIRTUE: [&str; 10] = [
    "ambitious",
    "cautious",
    "courageous",
    "disciplined",
    "gregarious",
    "honorable",
    "humble",
    "merciful",
    "serene",
    "tolerant",
];
const VICE: [&str; 10] = [
    "aggressive",
    "bitter",
    "craven",
    "deceitful",
    "greedy",
    "lazy",
    "nervous",
    "rude",
    "vain",
    "vengeful",
];
const MISFORTUNE: [&str; 9] = [
    "Was abandoned by their parents",
    "Had a crippling addiction",
    "Was blackmailed",
    "Was condemned by those closest to them",
    "Had a curse put on them",
    "Was swindled out of a large fortune",
    "Was demoted from a job they liked",
    "Was disowned by their parents",
    "Was exiled from their tribe",
];
