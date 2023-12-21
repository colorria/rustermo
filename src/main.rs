use core::fmt;
use rand::Rng;
use std::{fmt::Formatter, fs, io, path::Path};

struct Jogo {
    palavra_comp: String,
    palavra_disp: String,
    tentativas: Vec<String>,
}

impl From<String> for Jogo {
    fn from(value: String) -> Self {
        let palavra_comp;
        let palavra_disp;

        if value.contains(",") {
            let mut parts = value.split(",");
            palavra_comp = parts.next().unwrap().to_string();
            palavra_disp = parts.next().unwrap().to_string();
        } else {
            palavra_comp = value.clone();
            palavra_disp = value.clone();
        }

        return Jogo {
            palavra_comp: palavra_comp,
            palavra_disp: palavra_disp,
            tentativas: vec![],
        };
    }
}

impl fmt::Display for Jogo {
    // printa tudo bonitinho ou algo do tipo
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut ret = String::from("");
        for i in 0..6 {
            let tentativa = self.tentativas.get(i);
            match tentativa {
                Some(text) => {
                    ret.push_str(text);
                    ret.push_str("\n");
                }
                None => ret.push_str("_____\n"),
            }
        }
        write!(f, "{}", ret)
    }
}

impl Jogo {
    fn adiciona_tentativa(&mut self, t: String) {
        // TODO se a pessoa passar mais ou menos que 5 letras, ou vier com qq outra que não seja [a-z], explode
        self.tentativas.push(t.trim().to_string());
    }

    fn acertou(&self) -> bool {
        let ultima = self.tentativas.last();
        match ultima {
            Some(t) => *t == self.palavra_comp,
            None => false,
        }
    }
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
    let palavras: Vec<String> = palavras.lines().map(String::from).collect();
    let index_aleatorio = rand::thread_rng().gen_range(0..palavras.len());
    let mut jogo: Jogo = palavras[index_aleatorio].clone().into();
    //println!("{}", jogo.palavra_comp);

    // começa loop pra catar até 6 tentativas do infeliz que tá jogando isso
    for _i in 0..6 {
        println!("\x1B[2J\x1B[1;1H");

        println!("{}", jogo);

        // confere cada tentativa pra ver se acertou algo
        if jogo.acertou() {
            // se acertou tudo, manda um salve, um prbs, e encerra
            println!("ae ganhooo");
            break;
        }

        let mut tentativa = String::new();

        io::stdin()
            .read_line(&mut tentativa)
            .expect("Deu ruim na hora de ler a tentativa");

        jogo.adiciona_tentativa(tentativa);
    }

    println!("\x1B[2J\x1B[1;1H");
    println!("{}", jogo);
    if jogo.acertou() {
        println!("ae ganhoooooo");
    } else {
        println!("perdeste. Era {}", jogo.palavra_disp);
    }
}
