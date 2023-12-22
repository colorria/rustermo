use std::{collections::HashMap, fmt};

const VERDE: &str = "\x1b[32m";
const AMARELO: &str = "\x1b[33m";
const NORMAL: &str = "\x1b[0m";

#[derive(Clone)]
enum Precisao {
    LetraCertaPosicaoCerta,
    LetraCertaPosicaoErrada,
    TudoErradoOuExcesso,
}

impl Precisao {
    fn colore(&self, letra: char) -> String {
        let mut letra_colorida = match &self {
            Precisao::LetraCertaPosicaoCerta => String::from(VERDE),
            Precisao::LetraCertaPosicaoErrada => String::from(AMARELO),
            Precisao::TudoErradoOuExcesso => String::from(NORMAL),
        };
        letra_colorida.push(letra);
        letra_colorida.push_str(NORMAL);
        return letra_colorida;
    }
}

pub struct PalavraComparavel {
    palavra: String,
    palavra_exibicao: String,
    acertos: Vec<Precisao>,
}

impl fmt::Display for PalavraComparavel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::from("");
        let mut chars_exibicao = self.palavra_exibicao.chars();

        for a in self.acertos.iter() {
            ret.push_str(&a.colore(chars_exibicao.next().unwrap()));
        }

        write!(f, "{}", ret)
    }
}

impl PalavraComparavel {
    pub fn cria(palavra: String, palavra_exibicao: String) -> Self {
        let acertos: Vec<Precisao> = vec![Precisao::LetraCertaPosicaoCerta; 5];
        return PalavraComparavel {
            palavra,
            palavra_exibicao,
            acertos,
        };
    }

    fn pega_saldo_comparacao(&self) -> HashMap<char, isize> {
        let mut saldo = HashMap::new();
        for c in self.palavra.chars() {
            saldo.entry(c).and_modify(|q| *q += 1).or_insert(1);
        }
        return saldo;
    }

    pub fn compara(&self, palavra: String, palavra_exibicao: String) -> Self {
        let chars_tentativa: Vec<char> = palavra.chars().collect();
        let chars_comp: Vec<char> = self.palavra.chars().collect();
        let mut saldo = self.pega_saldo_comparacao();
        let mut acertos: Vec<Precisao> = vec![];

        // primeiro vê o que está certo, e popula uns placeholders
        for i in 0..5 {
            let letra_tentativa = &chars_tentativa[i];
            let letra_comp = &chars_comp[i];
            let precisao: Precisao;

            if letra_tentativa == letra_comp {
                precisao = Precisao::LetraCertaPosicaoCerta;
                saldo.entry(*letra_tentativa).and_modify(|q| *q -= 1);
            } else {
                precisao = Precisao::TudoErradoOuExcesso;
            }

            acertos.push(precisao);
        }

        // depois vê o que tá meio certo, ou absolutamente errado
        for i in 0..5 {
            let letra_tentativa = &chars_tentativa[i];
            let letra_comp = &chars_comp[i];
            let saldo_restante = saldo.entry(*letra_tentativa).or_insert(0);

            if letra_tentativa != letra_comp && *saldo_restante > 0 {
                acertos[i] = Precisao::LetraCertaPosicaoErrada;
                saldo.entry(*letra_tentativa).and_modify(|q| *q -= 1);
            }
        }

        return PalavraComparavel {
            palavra,
            palavra_exibicao,
            acertos,
        };
    }

    pub fn esta_tudo_certo(&self) -> bool {
        return self
            .acertos
            .iter()
            .filter(|l| matches!(l, Precisao::LetraCertaPosicaoCerta))
            .count()
            == 5;
    }
}
