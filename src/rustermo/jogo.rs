use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    io,
};

use rand::Rng;

use super::palavra::PalavraComparavel;

pub struct Jogo<'a> {
    banco_de_palavras: &'a HashMap<String, String>,
    palavra: PalavraComparavel,
    tentativas: Vec<PalavraComparavel>,
    qtde_tentativas: usize,
}

impl fmt::Display for Jogo<'_> {
    // printa tudo bonitinho ou algo do tipo
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut ret = String::from("");
        for i in 0..self.qtde_tentativas {
            let tentativa = self.tentativas.get(i);
            match tentativa {
                Some(text) => {
                    ret.push_str(&format!("{}", text));
                    ret.push_str("\n");
                }
                None => ret.push_str("_____\n"),
            }
        }
        write!(f, "{}", ret)
    }
}

impl<'a> Jogo<'a> {
    pub fn cria(palavras: &'a HashMap<String, String>, qtde_tentativas: usize) -> Self {
        let index_aleatorio = rand::thread_rng().gen_range(0..palavras.len());
        let palavra = palavras.keys().nth(index_aleatorio).unwrap().clone();
        let palavra_exibicao = palavras.values().nth(index_aleatorio).unwrap().clone();

        return Jogo {
            banco_de_palavras: palavras,
            palavra: PalavraComparavel::cria(palavra, palavra_exibicao),
            tentativas: vec![],
            qtde_tentativas,
        };
    }

    // loop pra catar as tentativas do infeliz que tá jogando isso
    pub fn loop_principal(&mut self) {
        let mut num_tentativa_atual = 0;
        while num_tentativa_atual < self.qtde_tentativas {
            println!("\x1B[2J\x1B[1;1H");
            println!("{}", self);

            let mut tentativa = String::new();

            io::stdin()
                .read_line(&mut tentativa)
                .expect("Deu ruim na hora de ler a tentativa");

            let ganhou = self.tenta(tentativa);

            if ganhou {
                // se acertou tudo, manda um salve, um prbs, e encerra
                println!("\x1B[2J\x1B[1;1H");
                println!("{}", self);
                println!("ae ganhooo");
                return;
            } else {
                num_tentativa_atual += 1;
            }
        }

        println!("\x1B[2J\x1B[1;1H");
        println!("{}", self);
        println!("perderdes. Era {}", self.palavra);
    }

    fn tenta(&mut self, t: String) -> bool {
        let tentativa = t.trim().to_string();
        if tentativa.len() != 5 {
            return false;
        }

        // TODO ver se é uma palavra aceita, ou seja, que está no arquivo de palavras
        let palavra = self.banco_de_palavras.get(&tentativa);
        if palavra.is_none() {
            return false;
        }

        let tentativa = self.palavra.compara(tentativa, palavra.unwrap().clone());
        let ganhou = tentativa.esta_tudo_certo();

        self.tentativas.push(tentativa);

        return ganhou;
    }
}
