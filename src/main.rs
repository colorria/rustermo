use std::{collections::HashMap, fs, path::Path};

use crate::rustermo::jogo::Jogo;

mod rustermo;

fn cria_mapa(palavras: Vec<String>) -> HashMap<String, String> {
    let mut mapa = HashMap::new();

    for palavra in palavras {
        if palavra.contains(",") {
            let mut parts = palavra.split(",");
            mapa.insert(
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            );
        } else {
            mapa.insert(palavra.clone(), palavra);
        }
    }

    return mapa;
}

fn main() {
    const PALAVRAS_PATH: &str = "palavras.txt";
    // vê se tem arquivo com as palavras. Se não tiver, explode
    if !Path::new(PALAVRAS_PATH).exists() {
        panic!("Deveria existir o arquivo {PALAVRAS_PATH}");
    }

    // escolhe uma palavra aleatoriamente
    let arquivo_palavras = fs::read_to_string(PALAVRAS_PATH);
    let palavras = match arquivo_palavras {
        Ok(p) => p,
        Err(_) => panic!("ué"),
    };
    let palavras: HashMap<String, String> = cria_mapa(palavras.lines().map(String::from).collect());
    let mut jogo: Jogo = Jogo::cria(&palavras, 6);

    jogo.loop_principal();
}
