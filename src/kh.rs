// Kontrolní hlášení
use crate::{date_func::last_month_day, Config, date_func::first_month_day};
use std::fs::File;
use std::io::Read;
use xmltree::Element;


pub fn generate_kh(config: &Config) {
    // Load the XML template
    let mut xml_str = String::new();
    File::open("vzory/kh-vzor.xml")
        .expect("Template not found")
        .read_to_string(&mut xml_str)
        .unwrap();

    // Parse into xmltree::Element
    let mut root: Element = xmltree::Element::parse(xml_str.as_bytes()).unwrap();

    let zdobd_od = first_month_day(config.datum_za_obdobi.year(), config.datum_za_obdobi.month());
    let zdobd_do = last_month_day(config.datum_za_obdobi.year(), config.datum_za_obdobi.month());

    // Navigate to <DPHKH1>
    if let Some(dphkh1) = root.get_mut_child("DPHKH1") {
        // <VetaD>
        if let Some(vetad) = dphkh1.get_mut_child("VetaD") {
            vetad.attributes.insert("d_poddp".to_string(), format!("{:02}.{:02}.{}", config.datum_podpisu.day(), config.datum_podpisu.month() as u8, config.datum_podpisu.year()));
            vetad.attributes.insert("mesic".to_string(), (config.datum_za_obdobi.month() as u8).to_string());
            vetad.attributes.insert("rok".to_string(), config.datum_za_obdobi.year().to_string());
            vetad.attributes.insert("zdobd_od".to_string(), zdobd_od);
            vetad.attributes.insert("zdobd_do".to_string(), zdobd_do);
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
                format!("{:.2}", config.prijata_zdanitelna_plneni_czk),
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
