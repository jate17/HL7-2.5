use std::collections::HashMap;

#[derive(Debug, PartialEq)]

pub enum T0076 {
    ACK,
    ADR,
    ADT,
    BAR,
    BPS,
    BRP,
    BRT,
    BTS,
    CRM,
    CSU,
    DFT,
    DOC,
    DSR,
    EAC,
    EAN,
    EAR,
    EDR,
    EQQ,
    ERP,
    ESR,
    ESU,
    INR,
    INU,
    LSR,
    LSU,
    MCF,
    MDM,
    MFD,
    MFK,
    MFN,
    MFQ,
    MFR,
    NMD,
    NMQ,
    NMR,
    OMB,
    OMD,
    OMG,
    OMI,
    OML,
    OMN,
    OMP,
    OMS,
    ORB,
    ORD,
    ORF,
    ORG,
    ORI,
    ORL,
    ORM,
    ORN,
    ORP,
    ORR,
    ORS,
    ORU,
    OSQ,
    OSR,
    OUL,
    PEX,
    PGL,
    PIN,
    PMU,
    PPG,
    PPP,
    PPR,
    PPT,
    PPV,
    PRR,
    PTR,
    QBP,
    QCK,
    QCN,
    QRY,
    QSB,
    QSX,
    QVR,
    RAR,
    RAS,
    RCI,
    RCL,
    RDE,
    RDR,
    RDS,
    RDY,
    REF,
    RER,
    RGR,
    RGV,
    ROR,
    RPA,
    RPI,
    RPL,
    RPR,
    RQA,
    RQC,
    RQI,
    RQP,
    RQQ,
    RRA,
    RRD,
    RRE,
    RRG,
    RRI,
    RSP,
    RTB,
    SIU,
    SPQ,
    SQM,
    SQR,
    SRM,
    SRR,
    SSR,
    SSU,
    SUR,
    TBR,
    TCR,
    TCU,
    UDM,
    VQQ,
    VXQ,
    VXR,
    VXU,
    VXX
}

impl T0076 {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ACK" => Some(T0076::ACK),
            "ADR" => Some(T0076::ADR),
            "ADT" => Some(T0076::ADT),
            "BAR" => Some(T0076::BAR),
            "BPS" => Some(T0076::BPS),
            "BRP" => Some(T0076::BRP),
            "BRT" => Some(T0076::BRT),
            "BTS" => Some(T0076::BTS),
            "CRM" => Some(T0076::CRM),
            "CSU" => Some(T0076::CSU),
            "DFT" => Some(T0076::DFT),
            "DOC" => Some(T0076::DOC),
            "DSR" => Some(T0076::DSR),
            "EAC" => Some(T0076::EAC),
            "EAN" => Some(T0076::EAN),
            "EAR" => Some(T0076::EAR),
            "EDR" => Some(T0076::EDR),
            "EQQ" => Some(T0076::EQQ),
            "ERP" => Some(T0076::ERP),
            "ESR" => Some(T0076::ESR),
            "ESU" => Some(T0076::ESU),
            "INR" => Some(T0076::INR),
            "INU" => Some(T0076::INU),
            "LSR" => Some(T0076::LSR),
            "LSU" => Some(T0076::LSU),
            "MCF" => Some(T0076::MCF),
            "MDM" => Some(T0076::MDM),
            "MFD" => Some(T0076::MFD),
            "MFK" => Some(T0076::MFK),
            "MFN" => Some(T0076::MFN),
            "MFQ" => Some(T0076::MFQ),
            "MFR" => Some(T0076::MFR),
            "NMD" => Some(T0076::NMD),
            "NMQ" => Some(T0076::NMQ),
            "NMR" => Some(T0076::NMR),
            "OMB" => Some(T0076::OMB),
            "OMD" => Some(T0076::OMD),
            "OMG" => Some(T0076::OMG),
            "OMI" => Some(T0076::OMI),
            "OML" => Some(T0076::OML),
            "OMN" => Some(T0076::OMN),
            "OMP" => Some(T0076::OMP),
            "OMS" => Some(T0076::OMS),
            "ORB" => Some(T0076::ORB),
            "ORD" => Some(T0076::ORD),
            "ORF" => Some(T0076::ORF),
            "ORG" => Some(T0076::ORG),
            "ORI" => Some(T0076::ORI),
            "ORL" => Some(T0076::ORL),
            "ORM" => Some(T0076::ORM),
            "ORN" => Some(T0076::ORN),
            "ORP" => Some(T0076::ORP),
            "ORR" => Some(T0076::ORR),
            "ORS" => Some(T0076::ORS),
            "ORU" => Some(T0076::ORU),
            "OSQ" => Some(T0076::OSQ),
            "OSR" => Some(T0076::OSR),
            "OUL" => Some(T0076::OUL),
            "PEX" => Some(T0076::PEX),
            "PGL" => Some(T0076::PGL),
            "PIN" => Some(T0076::PIN),
            "PMU" => Some(T0076::PMU),
            "PPG" => Some(T0076::PPG),
            "PPP" => Some(T0076::PPP),
            "PPR" => Some(T0076::PPR),
            "PPT" => Some(T0076::PPT),
            "PPV" => Some(T0076::PPV),
            "PRR" => Some(T0076::PRR),
            "PTR" => Some(T0076::PTR),
            "QBP" => Some(T0076::QBP),
            "QCK" => Some(T0076::QCK),
            "QCN" => Some(T0076::QCN),
            "QRY" => Some(T0076::QRY),
            "QSB" => Some(T0076::QSB),
            "QSX" => Some(T0076::QSX),
            "QVR" => Some(T0076::QVR),
            "RAR" => Some(T0076::RAR),
            "RAS" => Some(T0076::RAS),
            "RCI" => Some(T0076::RCI),
            "RCL" => Some(T0076::RCL),
            "RDE" => Some(T0076::RDE),
            "RDR" => Some(T0076::RDR),
            "RDS" => Some(T0076::RDS),
            "RDY" => Some(T0076::RDY),
            "REF" => Some(T0076::REF),
            "RER" => Some(T0076::RER),
            "RGR" => Some(T0076::RGR),
            "RGV" => Some(T0076::RGV),
            "ROR" => Some(T0076::ROR),
            "RPA" => Some(T0076::RPA),
            "RPI" => Some(T0076::RPI),
            "RPL" => Some(T0076::RPL),
            "RPR" => Some(T0076::RPR),
            "RQA" => Some(T0076::RQA),
            "RQC" => Some(T0076::RQC),
            "RQI" => Some(T0076::RQI),
            "RQP" => Some(T0076::RQP),
            "RQQ" => Some(T0076::RQQ),
            "RRA" => Some(T0076::RRA),
            "RRD" => Some(T0076::RRD),
            "RRE" => Some(T0076::RRE),
            "RRG" => Some(T0076::RRG),
            "RRI" => Some(T0076::RRI),
            "RSP" => Some(T0076::RSP),
            "RTB" => Some(T0076::RTB),
            "SIU" => Some(T0076::SIU),
            "SPQ" => Some(T0076::SPQ),
            "SQM" => Some(T0076::SQM),
            "SQR" => Some(T0076::SQR),
            "SRM" => Some(T0076::SRM),
            "SRR" => Some(T0076::SRR),
            "SSR" => Some(T0076::SSR),
            "SSU" => Some(T0076::SSU),
            "SUR" => Some(T0076::SUR),
            "TBR" => Some(T0076::TBR),
            "TCR" => Some(T0076::TCR),
            "TCU" => Some(T0076::TCU),
            "UDM" => Some(T0076::UDM),
            "VQQ" => Some(T0076::VQQ),
            "VXQ" => Some(T0076::VXQ),
            "VXR" => Some(T0076::VXR),
            "VXU" => Some(T0076::VXU),
            "VXX" => Some(T0076::VXX),
            _ => None,
        }
    }
}



pub fn valid_triggers() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    map.insert("ACK", vec![]); // Varies, nessun trigger specifico
    map.insert("ADR", vec!["A19"]);
    map.insert("ADT", vec![
        "A01", "A04", "A08", "A13", "A02", "A03",
        "A05", "A14", "A28", "A31", "A06", "A07",
        "A09", "A10", "A11", "A12", "A15", "A16",
        "A17", "A18", "A20",
        "A21", "A22", "A23", "A25", "A26", "A27", "A29", "A32", "A33",
        "A24",
        "A30", "A34", "A35", "A36", "A46", "A47", "A48", "A49",
        "A37", "A38",
        "A39", "A40", "A41", "A42",
        "A43", "A44",
        "A45",
        "A50", "A51",
        "A52", "A53", "A55",
        "A54",
        "A60",
        "A61", "A62"
    ]);
    map.insert("BAR", vec!["P01", "P02", "P05", "P06", "P10", "P12"]);
    map.insert("BPS", vec!["O29"]);
    map.insert("BRP", vec!["O30"]);
    map.insert("BRT", vec!["O32"]);
    map.insert("BTS", vec!["O31"]);
    map.insert("CRM", vec!["C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08"]);
    map.insert("CSU", vec!["C09", "C10", "C11", "C12"]);
    map.insert("DFT", vec!["P03", "P11"]);
    map.insert("DOC", vec!["T12"]);
    map.insert("DSR", vec!["P04", "Q01", "Q03"]);
    map.insert("EAC", vec!["U07"]);
    map.insert("EAN", vec!["U09"]);
    map.insert("EAR", vec!["U08"]);
    map.insert("EDR", vec!["R07"]);
    map.insert("EQQ", vec!["Q04"]);
    map.insert("ERP", vec!["R09"]);
    map.insert("ESR", vec!["U02"]);
    map.insert("ESU", vec!["U01"]);
    map.insert("INR", vec!["U06"]);
    map.insert("INU", vec!["U05"]);
    map.insert("LSU", vec!["U12", "U13"]);
    map.insert("MDM", vec!["T01", "T03", "T05", "T07", "T09", "T11", "T02", "T04", "T06", "T08", "T10"]);
    map.insert("MFD", vec!["MFA"]);
    map.insert("MFK", vec![
        "M01", "M02", "M03", "M04", "M05", "M06", "M07", "M08", "M09", "M10", "M11"
    ]);
    map.insert("MFN", vec![
        "M01", "M02", "M03", "M04", "M05", "M06", "M07", "M08", "M09", "M10", "M11", "M12", "M13", "M15"
    ]);
    map.insert("MFQ", vec!["M01", "M02", "M03", "M04", "M05", "M06"]);
    map.insert("MFR", vec!["M01", "M02", "M03", "M04", "M05", "M06"]);
    map.insert("NMD", vec!["N02"]);
    map.insert("NMQ", vec!["N01"]);
    map.insert("NMR", vec!["N01"]);
    map.insert("OMB", vec!["O27"]);
    map.insert("OMD", vec!["O03"]);
    map.insert("OMG", vec!["O19"]);
    map.insert("OMI", vec!["O23"]);
    map.insert("OML", vec!["O21", "O33", "O35"]);
    map.insert("OMN", vec!["O07"]);
    map.insert("OMP", vec!["O09"]);
    map.insert("OMS", vec!["O05"]);
    map.insert("ORB", vec!["O28"]);
    map.insert("ORD", vec!["O04"]);
    map.insert("ORF", vec!["R04"]);
    map.insert("ORG", vec!["O20"]);
    map.insert("ORI", vec!["O24"]);
    map.insert("ORL", vec!["O22", "O34", "O36"]);
    map.insert("ORM", vec!["O01"]);
    map.insert("ORN", vec!["O08"]);
    map.insert("ORP", vec!["O10"]);
    map.insert("ORR", vec!["R02"]);
    map.insert("ORS", vec!["O06"]);
    map.insert("ORU", vec!["R01", "R30", "R31", "R32", "W01"]);
    map.insert("OSQ", vec!["Q06"]);
    map.insert("OSR", vec!["Q06"]);
    map.insert("OUL", vec!["R21", "R22", "R23", "R24"]);
    map.insert("PEX", vec!["P07", "P08"]);
    map.insert("PGL", vec!["PC6", "PC7", "PC8"]);
    map.insert("PMU", vec!["B01", "B02", "B03", "B04", "B05", "B06", "B07", "B08"]);
    map.insert("PPG", vec!["PCC", "PCG", "PCH", "PCJ"]);
    map.insert("PPP", vec!["PCB", "PCD"]);
    map.insert("PPR", vec!["PC1", "PC2", "PC3"]);
    map.insert("PPT", vec!["PCL"]);
    map.insert("PPV", vec!["PCA"]);
    map.insert("PRR", vec!["PC5"]);
    map.insert("PTR", vec!["PCF"]);
    map.insert("QBP", vec!["Q11", "Q13", "Q15", "Q21", "Q22", "Q23", "Q24", "Q25"]);
    map.insert("QCK", vec!["Q02"]);
    map.insert("QCN", vec!["J01", "J02"]);
    map.insert("QRF", vec!["W02"]);
    map.insert("QRY", vec![
        "A19", "P04", "PC4", "PC9", "PCE", "PCK",
        "Q01", "Q26", "Q27", "Q28", "Q29", "Q30",
        "Q02", "R02", "T12"
    ]);
    map.insert("QSB", vec!["Q16"]);
    map.insert("QVR", vec!["Q17"]);
    map.insert("RAR", vec!["RAR"]);
    map.insert("RAS", vec!["O17"]);
    map.insert("RCI", vec!["I05"]);
    map.insert("RCL", vec!["I06"]);
    map.insert("RDE", vec!["O01", "O11", "O25"]);
    map.insert("RDR", vec!["RDR"]);
    map.insert("RDS", vec!["O13"]);
    map.insert("RDY", vec!["K15"]);
    map.insert("REF", vec!["I12", "I13", "I14", "I15"]);
    map.insert("RER", vec!["RER"]);
    map.insert("RGR", vec!["RGR"]);
    map.insert("RGV", vec!["O15"]);
    map.insert("ROR", vec!["ROR"]);
    map.insert("RPA", vec!["I08", "I09", "I10", "I11"]);
    map.insert("RPI", vec!["I01", "I04"]);
    map.insert("RPL", vec!["I02"]);
    map.insert("RPR", vec!["I03"]);
    map.insert("RQA", vec!["I08", "I09", "I10", "I11"]);
    map.insert("RQC", vec!["I05", "I06"]);
    map.insert("RQI", vec!["I01", "I02", "I03", "I07"]);
    map.insert("RQP", vec!["I04"]);
    map.insert("RQQ", vec!["Q09"]);
    map.insert("RRA", vec!["O02", "O18"]);
    map.insert("RRD", vec!["O14"]);
    map.insert("RRE", vec!["O12", "O26"]);
    map.insert("RRG", vec!["O16"]);
    map.insert("RRI", vec!["I12", "I13", "I14", "I15"]);
    map.insert("RSP", vec!["K11", "K21", "K22", "K23", "K24"]);
    map.insert("RTB", vec!["K13"]);
    map.insert("SIU", vec![
        "S12", "S13", "S14", "S15", "S16", "S17", "S18", "S19",
        "S20", "S21", "S22", "S23", "S24", "S26"
    ]);
    map.insert("SPQ", vec!["Q08"]);
    map.insert("SQM", vec!["S25"]);
    map.insert("SQR", vec!["S25"]);
    map.insert("SRM", vec![
        "S01", "S02", "S03", "S04", "S05", "S06", "S07", "S08", "S09", "S10", "S11"
    ]);
    map.insert("SRR", vec![
        "S01", "S02", "S03", "S04", "S05", "S06", "S07", "S08", "S09", "S10", "S11"
    ]);
    map.insert("SSR", vec!["U04"]);
    map.insert("SSU", vec!["U03"]);
    map.insert("SUR", vec!["P09"]);
    map.insert("TBR", vec!["R08", "R09"]);
    map.insert("TCU", vec!["U10", "U11"]);
    map.insert("UDM", vec!["Q05"]);
    map.insert("VQQ", vec!["Q07"]);
    map.insert("VXQ", vec!["V01"]);
    map.insert("VXR", vec!["V03"]);
    map.insert("VXU", vec!["V04"]);
    map.insert("VXX", vec!["V02"]);

    map
}



pub struct MessageType {
    pub msg1: String, // Message code
    pub msg2: String, // Trigger event
}


impl MessageType {
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('^').collect();

        if parts.len() < 2 {
            // msg2 obbligatorio
            return None;
        }

        let msg1 = parts[0];
        let msg2 = parts[1];

        let valid_map = valid_triggers();
        if !valid_map.contains_key(msg1) {
            return None; // messaggio non valido
        }

        if !valid_map[msg1].contains(&msg2) {
            return None; // trigger event non valido
        }

        Some(MessageType {
            msg1: msg1.to_string(),
            msg2: msg2.to_string(),
        })
    }

    pub fn to_hl7_string(&self) -> String {
        // msg2 è obbligatorio quindi stampi sempre msg1^msg2
        format!("{}^{}", self.msg1, self.msg2)
    }
}



