use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    io,
};

use rand::Rng;

use super::palavra::PalavraComparavel;

const LIMPA: &str = "\x1B[2J\x1B[1;1H";

pub struct Jogo<'a> {
    banco_de_palavras: &'a HashMap<String, String>,
    palavra: PalavraComparavel,
    tentativas: Vec<PalavraComparavel>,
    qtde_tentativas: usize,
    msg: String,
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

        ret.push_str(&self.msg);
        ret.push_str("\n");

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
            msg: String::from(""),
        };
    }

    // loop pra catar as tentativas do infeliz que tá jogando isso
    pub fn loop_principal(&mut self) {
        let mut num_tentativa_atual = 0;
        while num_tentativa_atual < self.qtde_tentativas {
            println!("{}", LIMPA);
            println!("{}", self);

            let mut tentativa = String::new();

            io::stdin()
                .read_line(&mut tentativa)
                .expect("Deu ruim na hora de ler a tentativa");

            let resultado = self.tenta(tentativa);

            if let Ok(ganhou) = resultado {
                if ganhou {
                    // se acertou tudo, manda um salve, um prbs, e encerra
                    self.msg = String::from("ae ganhooo");
                    println!("{}", LIMPA);
                    println!("{}", self);
                    return;
                } else {
                    self.msg = String::from("");
                    num_tentativa_atual += 1;
                }
            } else if let Err(msg) = resultado {
                self.msg = msg.to_string();
            }
        }

        self.msg = format!("perderdes. Era {}", self.palavra);
        println!("{}", LIMPA);
        println!("{}", self);
    }

    fn tenta(&mut self, t: String) -> Result<bool, &str> {
        let tentativa = t.trim().to_string();
        if tentativa.len() != 5 {
            return Err("tem que ter 5 letras!!!");
        }

        // TODO ver se é uma palavra aceita, ou seja, que está no arquivo de palavras
        let palavra = self.banco_de_palavras.get(&tentativa);
        if palavra.is_none() {
            return Err("essa palavra não existe");
        }

        let tentativa = self.palavra.compara(tentativa, palavra.unwrap().clone());
        let ganhou = tentativa.esta_tudo_certo();

        self.tentativas.push(tentativa);

        return Ok(ganhou);
    }
}
