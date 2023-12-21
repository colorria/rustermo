use std::fmt;

enum Precisao {
    LetraCertaPosicaoCerta,
    LetraCertaPosicaoErrada,
    TudoErrado,
}

struct Letra {
    letra: char,
    precisao: Precisao,
}

impl Letra {
    fn printa(&self) -> String {
        let mut letra_colorida = match self.precisao {
            Precisao::LetraCertaPosicaoCerta => String::from("\x1b[32m"),
            Precisao::LetraCertaPosicaoErrada => String::from("\x1b[33m"),
            Precisao::TudoErrado => String::from("\x1b[0m"),
        };
        letra_colorida.push(self.letra);
        letra_colorida.push_str("\x1b[0m");
        return letra_colorida;
    }
}

pub struct PalavraComparavel {
    palavra: String,
    letras: Vec<Letra>,
}

impl fmt::Display for PalavraComparavel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::from("");

        self.letras.iter().for_each(|l| ret.push_str(&l.printa()));

        write!(f, "{}", ret)
    }
}

impl PalavraComparavel {
    pub fn cria(palavra: String) -> Self {
        let letras: Vec<Letra> = palavra
            .chars()
            .map(|letra| Letra {
                letra,
                precisao: Precisao::LetraCertaPosicaoCerta,
            })
            .collect();
        return PalavraComparavel { palavra, letras };
    }

    pub fn compara(&self, palavra: String) -> Self {
        let chars: Vec<char> = palavra.chars().collect();
        let mut letras: Vec<Letra> = vec![];

        for i in 0..5 {
            let letra_tentativa = &chars[i];
            let letra_comp = &self.letras[i].letra;
            let precisao: Precisao;

            if letra_tentativa == letra_comp {
                precisao = Precisao::LetraCertaPosicaoCerta;
            } else {
                precisao = Precisao::TudoErrado;
            }

            letras.push(Letra {
                letra: *letra_tentativa,
                precisao,
            });
        }

        return PalavraComparavel { palavra, letras };
    }

    pub fn esta_tudo_certo(&self) -> bool {
        return self
            .letras
            .iter()
            .filter(|l| matches!(l.precisao, Precisao::LetraCertaPosicaoCerta))
            .count()
            == 5;
    }
}
