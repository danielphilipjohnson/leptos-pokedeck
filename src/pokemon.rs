use gloo_net::http::Request;
use serde::Deserialize;

pub const POKEMON_BATCH_SIZE: u32 = 10;

#[derive(Clone, Debug, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub sprites: PokemonSprites,
    pub types: Vec<PokemonType>,
    pub stats: Vec<PokemonStat>,
}

impl Pokemon {
    pub fn artwork(&self) -> Option<&str> {
        self.sprites
            .other
            .as_ref()
            .and_then(|other| other.official_artwork.as_ref())
            .and_then(|art| art.front_default.as_deref())
            .or_else(|| self.sprites.front_default.as_deref())
    }

    pub fn primary_type(&self) -> Option<&str> {
        self.types
            .iter()
            .min_by_key(|pokemon_type| pokemon_type.slot)
            .map(|pokemon_type| pokemon_type.r#type.name.as_str())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub other: Option<PokemonOtherSprites>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PokemonOtherSprites {
    #[serde(rename = "official-artwork")]
    pub official_artwork: Option<PokemonArtwork>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PokemonArtwork {
    pub front_default: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PokemonType {
    pub slot: u8,
    #[serde(rename = "type")]
    pub r#type: NamedResource,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PokemonStat {
    pub base_stat: u32,
    pub stat: NamedResource,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NamedResource {
    pub name: String,
}

pub async fn fetch_pokemon_page(page: u32) -> Result<Vec<Pokemon>, String> {
    let mut pokemon = Vec::with_capacity(POKEMON_BATCH_SIZE as usize);
    let start = page.saturating_mul(POKEMON_BATCH_SIZE).saturating_add(1);
    let end = start + POKEMON_BATCH_SIZE;

    for id in start..end {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{id}");
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|err| format!("request failed for id {id}: {err}"))?;

        let mut entry: Pokemon = response
            .json()
            .await
            .map_err(|err| format!("invalid response payload for id {id}: {err}"))?;

        entry
            .types
            .sort_by(|left, right| left.slot.cmp(&right.slot));
        pokemon.push(entry);
    }

    pokemon.sort_by_key(|p| p.id);
    Ok(pokemon)
}
