use gloo_net::http::Request;
use leptos::*;
use serde::Deserialize;

const POKEMON_BATCH_SIZE: u32 = 10;

#[derive(Clone, Debug, Deserialize)]
struct Pokemon {
    id: u32,
    name: String,
    sprites: PokemonSprites,
    types: Vec<PokemonType>,
    stats: Vec<PokemonStat>,
}

impl Pokemon {
    fn artwork(&self) -> Option<&str> {
        self.sprites
            .other
            .as_ref()
            .and_then(|other| other.official_artwork.as_ref())
            .and_then(|art| art.front_default.as_deref())
            .or_else(|| self.sprites.front_default.as_deref())
    }

    fn primary_type(&self) -> Option<&str> {
        self.types
            .iter()
            .min_by_key(|pokemon_type| pokemon_type.slot)
            .map(|pokemon_type| pokemon_type.r#type.name.as_str())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct PokemonSprites {
    front_default: Option<String>,
    other: Option<PokemonOtherSprites>,
}

#[derive(Clone, Debug, Deserialize)]
struct PokemonOtherSprites {
    #[serde(rename = "official-artwork")]
    official_artwork: Option<PokemonArtwork>,
}

#[derive(Clone, Debug, Deserialize)]
struct PokemonArtwork {
    front_default: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct PokemonType {
    slot: u8,
    #[serde(rename = "type")]
    r#type: NamedResource,
}

#[derive(Clone, Debug, Deserialize)]
struct PokemonStat {
    base_stat: u32,
    stat: NamedResource,
}

#[derive(Clone, Debug, Deserialize)]
struct NamedResource {
    name: String,
}

#[derive(Clone, Copy)]
struct TypeTheme {
    card_border: &'static str,
    card_bg_start: &'static str,
    card_bg_end: &'static str,
    tag_bg_start: &'static str,
    tag_bg_end: &'static str,
    tag_shadow: &'static str,
}

const DEFAULT_THEME: TypeTheme = TypeTheme {
    card_border: "#2a75bb",
    card_bg_start: "#f4f4f4",
    card_bg_end: "#e8e8e8",
    tag_bg_start: "#7AC74C",
    tag_bg_end: "#5a9c3c",
    tag_shadow: "0 4px 12px rgba(122, 199, 76, 0.5)",
};

fn type_theme(type_name: &str) -> TypeTheme {
    match type_name {
        "bug" => TypeTheme {
            card_border: "#A6B91A",
            card_bg_start: "#f5f8d2",
            card_bg_end: "#dfe68f",
            tag_bg_start: "#A6B91A",
            tag_bg_end: "#8a9c14",
            tag_shadow: "0 4px 12px rgba(166, 185, 26, 0.5)",
        },
        "dark" => TypeTheme {
            card_border: "#705746",
            card_bg_start: "#e4d8d1",
            card_bg_end: "#cdbfb7",
            tag_bg_start: "#705746",
            tag_bg_end: "#564232",
            tag_shadow: "0 4px 12px rgba(112, 87, 70, 0.5)",
        },
        "dragon" => TypeTheme {
            card_border: "#6F35FC",
            card_bg_start: "#e4dbff",
            card_bg_end: "#c7b2ff",
            tag_bg_start: "#6F35FC",
            tag_bg_end: "#4f1ad9",
            tag_shadow: "0 4px 12px rgba(111, 53, 252, 0.5)",
        },
        "electric" => TypeTheme {
            card_border: "#F7D02C",
            card_bg_start: "#fff7c6",
            card_bg_end: "#ffe680",
            tag_bg_start: "#F7D02C",
            tag_bg_end: "#d4af1f",
            tag_shadow: "0 4px 12px rgba(247, 208, 44, 0.5)",
        },
        "fairy" => TypeTheme {
            card_border: "#D685AD",
            card_bg_start: "#ffe3f0",
            card_bg_end: "#f6bfd8",
            tag_bg_start: "#D685AD",
            tag_bg_end: "#b9648c",
            tag_shadow: "0 4px 12px rgba(214, 133, 173, 0.5)",
        },
        "fighting" => TypeTheme {
            card_border: "#C22E28",
            card_bg_start: "#f7d6d4",
            card_bg_end: "#e9a29b",
            tag_bg_start: "#C22E28",
            tag_bg_end: "#9b231f",
            tag_shadow: "0 4px 12px rgba(194, 46, 40, 0.5)",
        },
        "fire" => TypeTheme {
            card_border: "#EE8130",
            card_bg_start: "#fff3e0",
            card_bg_end: "#ffe0b2",
            tag_bg_start: "#EE8130",
            tag_bg_end: "#c86420",
            tag_shadow: "0 4px 12px rgba(238, 129, 48, 0.5)",
        },
        "flying" => TypeTheme {
            card_border: "#A98FF3",
            card_bg_start: "#f1e9ff",
            card_bg_end: "#d5c8fa",
            tag_bg_start: "#A98FF3",
            tag_bg_end: "#8970d8",
            tag_shadow: "0 4px 12px rgba(169, 143, 243, 0.5)",
        },
        "ghost" => TypeTheme {
            card_border: "#735797",
            card_bg_start: "#e5dcf3",
            card_bg_end: "#c9b8dd",
            tag_bg_start: "#735797",
            tag_bg_end: "#57406f",
            tag_shadow: "0 4px 12px rgba(115, 87, 151, 0.5)",
        },
        "grass" => TypeTheme {
            card_border: "#7AC74C",
            card_bg_start: "#e8f5e9",
            card_bg_end: "#c8e6c9",
            tag_bg_start: "#7AC74C",
            tag_bg_end: "#5a9c3c",
            tag_shadow: "0 4px 12px rgba(122, 199, 76, 0.5)",
        },
        "ground" => TypeTheme {
            card_border: "#E2BF65",
            card_bg_start: "#fdf0d1",
            card_bg_end: "#eed2a4",
            tag_bg_start: "#E2BF65",
            tag_bg_end: "#c79f47",
            tag_shadow: "0 4px 12px rgba(226, 191, 101, 0.5)",
        },
        "ice" => TypeTheme {
            card_border: "#96D9D6",
            card_bg_start: "#e7fafa",
            card_bg_end: "#c8ebea",
            tag_bg_start: "#96D9D6",
            tag_bg_end: "#75bebb",
            tag_shadow: "0 4px 12px rgba(150, 217, 214, 0.5)",
        },
        "normal" => TypeTheme {
            card_border: "#A8A77A",
            card_bg_start: "#f5f5dc",
            card_bg_end: "#e2e2c4",
            tag_bg_start: "#A8A77A",
            tag_bg_end: "#8e8d61",
            tag_shadow: "0 4px 12px rgba(168, 167, 122, 0.5)",
        },
        "poison" => TypeTheme {
            card_border: "#A33EA1",
            card_bg_start: "#f4e1f3",
            card_bg_end: "#deb3db",
            tag_bg_start: "#A33EA1",
            tag_bg_end: "#832e81",
            tag_shadow: "0 4px 12px rgba(163, 62, 161, 0.5)",
        },
        "psychic" => TypeTheme {
            card_border: "#F95587",
            card_bg_start: "#ffe0eb",
            card_bg_end: "#ffb3cc",
            tag_bg_start: "#F95587",
            tag_bg_end: "#d83a69",
            tag_shadow: "0 4px 12px rgba(249, 85, 135, 0.5)",
        },
        "rock" => TypeTheme {
            card_border: "#B6A136",
            card_bg_start: "#f1e7c2",
            card_bg_end: "#ddc67c",
            tag_bg_start: "#B6A136",
            tag_bg_end: "#9a832d",
            tag_shadow: "0 4px 12px rgba(182, 161, 54, 0.5)",
        },
        "steel" => TypeTheme {
            card_border: "#B7B7CE",
            card_bg_start: "#eef0fb",
            card_bg_end: "#d0d5e9",
            tag_bg_start: "#B7B7CE",
            tag_bg_end: "#9494b0",
            tag_shadow: "0 4px 12px rgba(183, 183, 206, 0.5)",
        },
        "water" => TypeTheme {
            card_border: "#6390F0",
            card_bg_start: "#e0ebff",
            card_bg_end: "#bad0ff",
            tag_bg_start: "#6390F0",
            tag_bg_end: "#4d74c7",
            tag_shadow: "0 4px 12px rgba(99, 144, 240, 0.5)",
        },
        _ => DEFAULT_THEME,
    }
}

async fn fetch_pokemon_page(page: u32) -> Result<Vec<Pokemon>, String> {
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

#[component]
fn App() -> impl IntoView {
    let (page, set_page) = create_signal(0_u32);
    let pokedex = create_local_resource(
        move || page.get(),
        |page| async move { fetch_pokemon_page(page).await },
    );
    let is_loading = pokedex.loading();
    let pokemon = create_rw_signal(Vec::<Pokemon>::new());
    let error = create_rw_signal(None::<String>);

    create_effect(move |_| match pokedex.get() {
        Some(Ok(entries)) => {
            error.set(None);
            pokemon.update(|list| {
                for entry in &entries {
                    if !list.iter().any(|existing| existing.id == entry.id) {
                        list.push(entry.clone());
                    }
                }
            });
        }
        Some(Err(message)) => error.set(Some(message)),
        None => {}
    });

    let pokedex_handle = pokedex.clone();
    let load_more = move |_| {
        if error.with(|err| err.is_some()) {
            error.set(None);
            pokedex_handle.refetch();
        } else {
            set_page.update(|value| *value = value.saturating_add(1));
        }
    };

    view! {
        <main class="app">
            <div class="header">
                <h1>{"⚡ POKÉDEX ⚡"}</h1>
                <p>{"Gotta Catch 'Em All!"}</p>
            </div>

            {move || {
                let entries = pokemon.get();
                if entries.is_empty() {
                    if let Some(message) = error.get() {
                        view! { <div class="status error">{message}</div> }
                    } else {
                        view! { <div class="status">{"Loading Pokémon..."}</div> }
                    }
                } else {
                    view! {
                        <div class="pokemon-container">
                            {entries
                                .into_iter()
                                .enumerate()
                                .map(|(idx, entry)| {
                                    view! { <PokemonCard entry delay=idx as f32 * 0.1 /> }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    }
                }
            }}

            {move || {
                if pokemon.with(|list| list.is_empty()) {
                    View::default()
                } else if let Some(message) = error.get() {
                    view! { <div class="status error">{message}</div> }.into_view()
                } else {
                    View::default()
                }
            }}

            <button
                class="show-more"
                on:click=load_more
                prop:disabled=move || is_loading.get()
            >
                {move || {
                    if is_loading.get() {
                        "Loading..."
                    } else if error.with(|msg| msg.is_some()) {
                        "Retry"
                    } else {
                        "Show More"
                    }
                }}
            </button>
        </main>
    }
}

#[component]
fn PokemonCard(entry: Pokemon, delay: f32) -> impl IntoView {
    let primary_type = entry.primary_type().unwrap_or("unknown").to_string();
    let theme = type_theme(primary_type.as_str());
    let card_style = format!(
        "border-color: {}; background: linear-gradient(145deg, {} 0%, {} 100%); animation-delay: {:.2}s;",
        theme.card_border, theme.card_bg_start, theme.card_bg_end, delay
    );

    let number = format!("#{:03}", entry.id);
    let display_name = entry.name.to_uppercase();
    let image_src = entry.artwork().map(str::to_string).unwrap_or_else(|| {
        String::from(
            "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/0.png",
        )
    });

    let stats = entry
        .stats
        .iter()
        .take(3)
        .map(|stat| {
            let label = stat_label(&stat.stat.name);
            let value = stat.base_stat.to_string();
            view! {
                <div class="stat-row">
                    <span class="stat-label">{label}</span>
                    <span class="stat-value">{value}</span>
                </div>
            }
        })
        .collect::<Vec<_>>();

    let resistances = entry
        .types
        .iter()
        .map(|pokemon_type| {
            let type_name = pokemon_type.r#type.name.clone();
            let theme = type_theme(type_name.as_str());
            let tag_style = format!(
                "background: linear-gradient(135deg, {} 0%, {} 100%); box-shadow: {};",
                theme.tag_bg_start, theme.tag_bg_end, theme.tag_shadow
            );
            let label = type_name.to_ascii_uppercase();
            view! {
                <span class="resistance-tag" style=tag_style>{label}</span>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="pokemon-card" style=card_style attr:data-primary-type=primary_type>
            <div class="card-header">
                <div class="pokemon-number">{number}</div>
                <div class="pokemon-name">{display_name.clone()}</div>
            </div>

            <div class="pokemon-image-container">
                <img src=image_src alt=display_name class="pokemon-image" />
            </div>

            <div class="stats-container">{stats}</div>

            <div class="resistance-container">
                <div class="resistance-label">{"Type Resistances"}</div>
                <div class="resistance-tags">{resistances}</div>
            </div>
        </div>
    }
}

fn stat_label(label: &str) -> String {
    label
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
