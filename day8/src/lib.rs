// day8/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 8 Challenge Endpoints

// dependencies
use common_features::WebRequest;
use domain::PokemonWeight;
use reqwest::Client;
use std::convert::Infallible;
use utilities::parameter_extractor;

// endpoint which returns the weight in kilograms of a Pokemon
pub async fn svc_get_pokemon_weight(request: WebRequest) -> Result<String, Infallible> {
    // create a reqwest client
    let client = Client::new();

    let segments = parameter_extractor(request.uri().path());

    let pokedex_number = segments[1];

    // get the weight in kilograms of the Pokemon
    let pokemon = client
        .get(&format!(
            "https://pokeapi.co/api/v2/pokemon/{}",
            pokedex_number
        ))
        .send()
        .await
        .unwrap()
        .json::<PokemonWeight>()
        .await
        .unwrap();

    // create a response body
    let response_msg = (pokemon.weight / 10f32).to_string();

    // return the response
    Ok(response_msg)
}
