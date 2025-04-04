use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::State;

use crate::{
    entities::{
        action::Actions,
        character::{Character, Dialog},
        game::{Game, GameTrait},
        location::Location,
        player::Player,
    },
    AppData,
};

#[tauri::command]
pub fn start_game(state: State<'_, Mutex<AppData>>) -> GameData {
    let action1 = Actions::new(
        "checar bolso",
        vec![String::from("olharbolso"), String::from("checarbolso")],
        "Olhar bolso",
        "Voce olhou seu bolso e pegou uma chave",
        Some("fail_message".to_string()),
    );
    let action2 = Actions::new(
        "usar chave",
        vec![String::from("usarchave"), String::from("pegarchave")],
        "Usando a chave",
        "Voce destrancou a porta com a chave",
        Some("Voce precisa pegar a chave, tente usar 'checar bolso' antes".to_string()),
    );
    let action3 = Actions::new(
        "Abrir porta",
        vec![String::from("abrirporta"), String::from("virarmacaneta")],
        "Abrindo a porta",
        "Voce abriu a porta",
        Some("Voce precisa de uma chave para abrir a porta".to_string()),
    );

    let character1 = Character::new(
        "Lizybet",
        vec![Dialog::new("Como você está?", "Estou bem")],
        "bla bla bla",
        vec![(String::from(""), String::from(""))],
    );
    let character2 = Character::new(
        "Kim",
        vec![Dialog::new("Como você está?", "Estou bem")],
        "pi pi pi",
        vec![(String::from("Abrir porta"), String::from("Mudar dialogo"))],
    );

    let mut location = Location::new("Quarto", vec![String::from("O quarto é um desastre. O papel de parede descascado revela manchas escuras de umidade que se espalham como veias apodrecidas pela superfície das paredes. O carpete, outrora de um bege discreto, agora está manchado com marcas de algo derramado há muito tempo—talvez álcool, talvez café, talvez algo pior."), String::from("Uma cama de solteiro ocupa o canto, lençóis amassados e um travesseiro que já perdeu sua forma. O colchão afunda em alguns pontos, como se alguém tivesse dormido nele por semanas sem se mexer muito. Há um cheiro no ar, uma mistura de fumaça velha, suor e uma leve acidez alcoólica."), String::from("Perto da cama, uma cômoda manca, com uma gaveta entreaberta. Dentro dela, roupas emboladas, talvez esquecidas. No chão, uma garrafa vazia rola lentamente quando alguém pisa forte o suficiente. O abajur no criado-mudo está torto, a lâmpada queimada ou simplesmente ignorada."), String::from("A janela é a única promessa de algo lá fora, mas está coberta por cortinas encardidas, pesadas com a poeira do tempo. A luz que entra é fraca, filtrada, como se o próprio dia hesitasse em tocar esse lugar."), String::from("O espelho do banheiro, se houver um, deve estar rachado—ou foi virado para a parede. Afinal, quem gostaria de se encarar depois de noites e noites passadas aqui?")], vec![character1, character2], vec![String::from("banheiro"), String::from("corredor")]);

    location.add_action(action1);
    location.add_action(action2);
    location.add_action(action3);

    let action1 = Actions::new(
        "checar bolso",
        vec![String::from("olharbolso"), String::from("checarbolso")],
        "Olhar bolso",
        "Voce olhou seu bolso e pegou uma chave",
        Some("fail_message".to_string()),
    );
    let action2 = Actions::new(
        "usar chave",
        vec![String::from("usarchave"), String::from("pegarchave")],
        "Usando a chave",
        "Voce destrancou a porta com a chave",
        Some("Voce precisa pegar a chave, tente usar 'checar bolso' antes".to_string()),
    );
    let action3 = Actions::new(
        "Abrir porta",
        vec![String::from("abrirporta"), String::from("virarmacaneta")],
        "Abrindo a porta",
        "Voce abriu a porta",
        Some("Voce precisa de uma chave para abrir a porta".to_string()),
    );

    let character1 = Character::new(
        "Lizybet",
        vec![Dialog::new("Como você está?", "Estou bem")],
        "bla bla bla",
        vec![(String::from(""), String::from(""))],
    );
    let character2 = Character::new(
        "Kim",
        vec![Dialog::new("Como você está?", "Estou bem")],
        "pi pi pi",
        vec![(String::from("Abrir porta"), String::from("Mudar dialogo"))],
    );

    let mut location2 = Location::new("Quarto", vec![String::from("O quarto é um desastre. O papel de parede descascado revela manchas escuras de umidade que se espalham como veias apodrecidas pela superfície das paredes. O carpete, outrora de um bege discreto, agora está manchado com marcas de algo derramado há muito tempo—talvez álcool, talvez café, talvez algo pior."), String::from("Uma cama de solteiro ocupa o canto, lençóis amassados e um travesseiro que já perdeu sua forma. O colchão afunda em alguns pontos, como se alguém tivesse dormido nele por semanas sem se mexer muito. Há um cheiro no ar, uma mistura de fumaça velha, suor e uma leve acidez alcoólica."), String::from("Perto da cama, uma cômoda manca, com uma gaveta entreaberta. Dentro dela, roupas emboladas, talvez esquecidas. No chão, uma garrafa vazia rola lentamente quando alguém pisa forte o suficiente. O abajur no criado-mudo está torto, a lâmpada queimada ou simplesmente ignorada."), String::from("A janela é a única promessa de algo lá fora, mas está coberta por cortinas encardidas, pesadas com a poeira do tempo. A luz que entra é fraca, filtrada, como se o próprio dia hesitasse em tocar esse lugar."), String::from("O espelho do banheiro, se houver um, deve estar rachado—ou foi virado para a parede. Afinal, quem gostaria de se encarar depois de noites e noites passadas aqui?")], vec![character1, character2], vec![String::from("banheiro"), String::from("corredor")]);

    location2.add_action(action1);
    location2.add_action(action2);
    location2.add_action(action3);

    let game = Game::new(
        Player::new("Harry", vec![String::from("")]),
        vec![location],
        location2.name.clone(),
    );

    let return_game = GameData::from(&game);
    let game_trait_object: Arc<Mutex<dyn GameTrait + Send + Sync>> = Arc::new(Mutex::new(game));

    let mut app_data: std::sync::MutexGuard<'_, AppData> = state.lock().unwrap();
    app_data.game = Some(game_trait_object);
    return_game
}

#[derive(Serialize)]
pub struct GameData {
    pub player: Player,
    pub actual_location: String,
    pub locations: HashMap<String, Location>,
}

impl From<&Game> for GameData {
    fn from(game: &Game) -> Self {
        Self {
            actual_location: game.actual_location.clone(),
            player: game.player.clone(),
            locations: game.locations.clone(), // map other fields...
        }
    }
}
