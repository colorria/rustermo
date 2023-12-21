use rand::Rng;
use std::{fs, path::Path};

use crate::rustermo::jogo::Jogo;

mod rustermo;

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
    let palavras: Vec<String> = palavras.lines().map(String::from).collect();
    let index_aleatorio = rand::thread_rng().gen_range(0..palavras.len());
    let mut jogo: Jogo = Jogo::cria(palavras[index_aleatorio].clone(), 6);

    jogo.loop_principal();
}
