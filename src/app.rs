use crate::components::PokemonCard;
use crate::pokemon::{fetch_pokemon_page, Pokemon};
use leptos::*;

const FILTERS: [(&str, &str); 5] = [
    ("All", "all"),
    ("🌿 Grass", "grass"),
    ("🔥 Fire", "fire"),
    ("💧 Water", "water"),
    ("⚡ Electric", "electric"),
];

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = create_signal(0_u32);
    let (filter, set_filter) = create_signal("all");
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

            <div class="filter-section">
                {FILTERS
                    .iter()
                    .map(|(label, value)| {
                        let value_static = *value;
                        let label_text = *label;
                        view! {
                            <button
                                class="filter-btn"
                                class:active=move || filter.get() == value_static
                                on:click=move |_| {
                                    set_filter.set(value_static);
                                }
                            >
                                {label_text}
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
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
                    let active_filter = filter.get();
                    let filtered_entries: Vec<_> = entries
                        .into_iter()
                        .filter(|entry| {
                            active_filter == "all"
                                || entry.types.iter().any(|t| t.r#type.name == active_filter)
                        })
                        .collect();

                    if filtered_entries.is_empty() {
                        view! { <div class="status">{"No Pokémon match this type yet. Try loading more!"}</div> }
                    } else {
                        view! {
                            <div class="pokemon-container">
                                {filtered_entries
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, entry)| {
                                        view! { <PokemonCard entry delay=idx as f32 * 0.1 /> }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        }
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

            <footer class="site-footer" aria-label="Author links">
                <a href="https://danielphilipjohnson.com/" target="_blank" rel="noopener noreferrer">
                    Personal Site
                </a>
                <a href="https://github.com/danielphilipjohnson" target="_blank" rel="noopener noreferrer">
                    GitHub
                </a>
                <a href="https://bsky.app/profile/daniel-philips-enterprise.com" target="_blank" rel="noopener noreferrer">
                    Bluesky
                </a>
            </footer>
        </main>
    }
}
