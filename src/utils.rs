use osu_db::listing::Listing;
use std::collections::HashSet;
use std::hash::Hash;

pub fn get_all_beatmapset_ids(db_file: &str) -> Vec<i32> {
    let listing = Listing::from_file(db_file).unwrap();
    let mut beatmao_set_ids = listing
        .beatmaps
        .iter()
        .map(|x| x.beatmapset_id)
        .collect::<Vec<i32>>();

    dedup(&mut beatmao_set_ids);

    beatmao_set_ids
}

pub fn get_all_beatmap_ids(db_file: &str) -> Vec<i32> {
    let listing = Listing::from_file(db_file).unwrap();
    listing
        .beatmaps
        .iter()
        .map(|x| x.beatmap_id)
        .collect::<Vec<i32>>()
}

pub fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}
