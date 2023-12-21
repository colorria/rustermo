use std::{
    fmt::{self, Formatter},
    io,
};

use super::palavra::PalavraComparavel;

pub struct Jogo {
    palavra_comp: PalavraComparavel,
    palavra_disp: String,
    tentativas: Vec<PalavraComparavel>,
    qtde_tentativas: usize,
}

impl fmt::Display for Jogo {
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

impl Jogo {
    pub fn cria(palavra: String, qtde_tentativas: usize) -> Self {
        let palavra_comp;
        let palavra_disp;

        if palavra.contains(",") {
            let mut parts = palavra.split(",");
            palavra_comp = PalavraComparavel::cria(parts.next().unwrap().to_string());
            palavra_disp = parts.next().unwrap().to_string();
        } else {
            palavra_comp = PalavraComparavel::cria(palavra.clone());
            palavra_disp = palavra.clone();
        }

        return Jogo {
            palavra_comp,
            palavra_disp,
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
        println!("perderdes. Era {}", self.palavra_disp);
    }

    fn tenta(&mut self, t: String) -> bool {
        let tentativa = t.trim().to_string();
        if tentativa.len() != 5 {
            return false;
        }

        // TODO ver se é uma palavra aceita, ou seja, que está no arquivo de palavras
        let tentativa = self.palavra_comp.compara(tentativa);
        let ganhou = tentativa.esta_tudo_certo();

        self.tentativas.push(tentativa);

        return ganhou;
    }
}
