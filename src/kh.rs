// Kontrolní hlášení
use crate::{date_func::first_month_day, date_func::last_month_day, Config};
use std::fs::File;
use std::io::Read;
use xmltree::{Element, XMLNode};

fn load_veta_b2_template() -> Element {
    let mut xml_str = String::new();
    File::open("vzory/KH-vzor-nad10.xml")
        .expect("Template vzory/KH-vzor-nad10.xml not found")
        .read_to_string(&mut xml_str)
        .unwrap();

    let root: Element = Element::parse(xml_str.as_bytes()).unwrap();
    root.get_child("DPHKH1")
        .and_then(|dphkh1| dphkh1.get_child("VetaB2"))
        .expect("Template vzory/KH-vzor-nad10.xml must contain VetaB2")
        .clone()
}

pub fn generate_kh(config: &Config) {
    // Load the XML template
    let mut xml_str = String::new();
    File::open("vzory/kh-vzor.xml")
        .expect("Template not found")
        .read_to_string(&mut xml_str)
        .unwrap();

    // Parse into xmltree::Element
    let mut root: Element = xmltree::Element::parse(xml_str.as_bytes()).unwrap();

    let zdobd_od = first_month_day(
        config.datum_za_obdobi.year(),
        config.datum_za_obdobi.month(),
    );
    let zdobd_do = last_month_day(
        config.datum_za_obdobi.year(),
        config.datum_za_obdobi.month(),
    );

    // Navigate to <DPHKH1>
    if let Some(dphkh1) = root.get_mut_child("DPHKH1") {
        // <VetaD>
        if let Some(vetad) = dphkh1.get_mut_child("VetaD") {
            vetad.attributes.insert(
                "d_poddp".to_string(),
                format!(
                    "{:02}.{:02}.{}",
                    config.datum_podpisu.day(),
                    config.datum_podpisu.month() as u8,
                    config.datum_podpisu.year()
                ),
            );
            vetad.attributes.insert(
                "mesic".to_string(),
                (config.datum_za_obdobi.month() as u8).to_string(),
            );
            vetad
                .attributes
                .insert("rok".to_string(), config.datum_za_obdobi.year().to_string());
            vetad.attributes.insert("zdobd_od".to_string(), zdobd_od);
            vetad.attributes.insert("zdobd_do".to_string(), zdobd_do);
        }

        if config.prijata_zdanitelna_plneni_nad_limit_czk > 0.0 {
            let mut vetab2 = load_veta_b2_template();
            let dppd = config
                .prijata_zdanitelna_plneni_nad_limit_dppd
                .expect("DPPD is required for VetaB2");
            vetab2.attributes.insert(
                "dic_dod".to_string(),
                config
                    .prijata_zdanitelna_plneni_nad_limit_dic
                    .clone()
                    .expect("DIC is required for VetaB2"),
            );
            vetab2.attributes.insert(
                "c_evid_dd".to_string(),
                config
                    .prijata_zdanitelna_plneni_nad_limit_doklad
                    .clone()
                    .expect("Danovy doklad is required for VetaB2"),
            );
            vetab2.attributes.insert(
                "dppd".to_string(),
                format!(
                    "{:02}.{:02}.{}",
                    dppd.day(),
                    dppd.month() as u8,
                    dppd.year()
                ),
            );
            vetab2.attributes.insert(
                "zakl_dane1".to_string(),
                format!("{:.2}", config.prijata_zdanitelna_plneni_nad_limit_czk),
            );
            vetab2.attributes.insert(
                "dan1".to_string(),
                format!(
                    "{:.2}",
                    config.prijata_zdanitelna_plneni_nad_limit_czk * 0.21
                ),
            );

            let insert_at = dphkh1
                .children
                .iter()
                .position(|node| match node {
                    XMLNode::Element(element) => element.name == "VetaB3",
                    _ => false,
                })
                .unwrap_or(dphkh1.children.len());
            dphkh1.children.insert(insert_at, XMLNode::Element(vetab2));
        }

        // <VetaB3>
        if let Some(vetab3) = dphkh1.get_mut_child("VetaB3") {
            vetab3.attributes.insert(
                "zakl_dane1".to_string(),
                format!("{:.2}", config.prijata_zdanitelna_plneni_czk),
            );
            vetab3.attributes.insert(
                "dan1".to_string(),
                format!("{:.2}", config.dph_prijata_zdanitelna_plneni_czk),
            );
        }

        // <VetaC>
        if let Some(vetac) = dphkh1.get_mut_child("VetaC") {
            vetac.attributes.insert(
                "pln23".to_string(),
                format!(
                    "{:.2}",
                    config.prijata_zdanitelna_plneni_czk
                        + config.prijata_zdanitelna_plneni_nad_limit_czk
                ),
            );
        }
    }

    // Build dynamic filename: KH-YYYY-MM.xml
    let file_name = format!(
        "KH-{}-{:02}.xml",
        config.datum_za_obdobi.year(),
        config.datum_za_obdobi.month() as u8
    );

    let mut out = File::create(&file_name).unwrap();
    root.write(&mut out).unwrap();

    println!("✅ KH XML generated -> {file_name}");
}
