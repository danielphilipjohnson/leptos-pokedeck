use crate::pokemon::Pokemon;
use crate::theme::theme_for;
use leptos::*;

#[component]
pub fn PokemonCard(entry: Pokemon, delay: f32) -> impl IntoView {
    let primary_type = entry.primary_type().unwrap_or("unknown").to_string();
    let theme = theme_for(primary_type.as_str());
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
            let theme = theme_for(type_name.as_str());
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
