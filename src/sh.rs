// Souhrné hlášení 
use crate::Config;
use std::fs::File;
use std::io::Read;
use xmltree::Element;

pub fn generate_sh(config: &Config) {
    
    // Load the XML template
    let mut xml_str = String::new();
    File::open("vzory/sh-vzor.xml").expect("Template not found").read_to_string(&mut xml_str).unwrap();

    // Parse into xmltree::Element
    let mut root: Element = xmltree::Element::parse(xml_str.as_bytes()).unwrap();

    // Navigate to <VetaD>
    if let Some(dphshv) = root.get_mut_child("DPHSHV") {
        if let Some(vetad) = dphshv.get_mut_child("VetaD") {
            vetad.attributes.insert("d_poddp".to_string(), format!("{:02}.{:02}.{}", config.datum_podpisu.day(), config.datum_podpisu.month() as u8, config.datum_podpisu.year()));
            vetad.attributes.insert("mesic".to_string(), (config.datum_za_obdobi.month() as u8).to_string());
            vetad.attributes.insert("rok".to_string(), config.datum_za_obdobi.year().to_string());
        }
        if let Some(vetar) = dphshv.get_mut_child("VetaR") {
            vetar.attributes.insert("pln_hodnota".to_string(), config.hodnota_plneni_czk.to_string());
        }
    }

    // Build dynamic filename: SH-YYYY-MM.xml
    let file_name = format!(
        "SH-{}-{:02}.xml",
        config.datum_za_obdobi.year(),
        config.datum_za_obdobi.month() as u8
    );

    let mut out = File::create(&file_name).unwrap();
    root.write(&mut out).unwrap();

    println!("✅ SH XML generated -> {file_name}");
}
