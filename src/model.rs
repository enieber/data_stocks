//    "DT_GERACAO";"HH_GERACAO";"ANO_ELEICAO";"CD_TIPO_ELEICAO";"NM_TIPO_ELEICAO"           ;"NR_TURNO"  ;"CD_ELEICAO";"DS_ELEICAO"                    ;"DT_ELEICAO"    ;"TP_ABRANGENCIA";"SG_UF";"SG_UE";"NM_UE"       ;"CD_MUNICIPIO";"NM_MUNICIPIO";"NR_ZONA"    ;"NR_SECAO";"CD_CARGO";"DS_CARGO"  ;"NR_VOTAVEL";"NM_VOTAVEL"                            ;"QT_VOTOS";"NR_LOCAL_VOTACAO"
//    "21/12/2020";"16:55:14"  ;"2020"       ;"2"              ;"ELEI<C7><C3>O ORDIN<C1>RIA";"1"         ;"426"       ;"ELEI<C7><D5>ES MUNICIPAIS 2020";"15/11/2020"    ;"M"             ;"AC"   ;"01473";"TARAUAC<C1>" ;"1473"        ;"TARAUAC<C1>" ;"5"          ;"11"      ;"11"      ;"PREFEITO"  ;"12"        ;"MARIA LUCIN<C9>IA NERY DE LIMA MENEZES";"68"      ;"1171"

pub struct Data {
    ano: String,
    cargo: String,
    local_votacao: String,
    nome: String,
    numero_votavel: String,
    quantidade_votos: i64,
}

impl ToString for Data {
    fn to_string(&self) -> String {
        return format!(
            "Candidato {} teve {} para {} em: {}",
            &self.nome, &self.quantidade_votos, &self.cargo, &self.local_votacao
        );
    }
}

pub fn convert_to_data(value_data: Vec<&str>) -> Data {
    let result_qtd_votos = value_data[21].parse::<i64>();
    let qtd_votos = match result_qtd_votos {
        Ok(votos) => votos,
        Err(_error) => 0
    };
    let data = Data {
        ano: value_data[2].to_string(),
        cargo: value_data[17].to_string(),
        local_votacao: value_data[22].to_string(),
        nome: value_data[20].to_string(),
        numero_votavel: value_data[19].to_string(),
        quantidade_votos: qtd_votos,
    };
    return data;
}

pub fn split_pattern_file() -> String {
    return String::from(";");
}
