// Daňové přiznání k DPH
use crate::{date_func::{first_month_day, last_month_day}, Config};
use std::fs::File;
use std::io::Read;
use xmltree::Element;

pub fn generate_dph(config: &Config) {
    // Load the XML template
    let mut xml_str = String::new();
    File::open("vzory/dph-vzor.xml")
        .expect("Template not found")
        .read_to_string(&mut xml_str)
        .unwrap();

    // Parse into xmltree::Element
    let mut root: Element = xmltree::Element::parse(xml_str.as_bytes()).unwrap();

    let zdobd_od = first_month_day(config.datum_za_obdobi.year(), config.datum_za_obdobi.month());
    let zdobd_do = last_month_day(config.datum_za_obdobi.year(), config.datum_za_obdobi.month());
    
    let round_prijata = config.prijata_zdanitelna_plneni_czk.ceil() as i64;
    let round_dph_prijata = config.dph_prijata_zdanitelna_plneni_czk.ceil() as i64;
            
    // odp_sum_nar = DPH za služby v ČR + DPH za služby v jiném státě
    let dph_celkem = round_dph_prijata + config.dph_prijeti_sluzeb_v_jinem_state_czk;

    // Navigate to <DPHDP3>
    if let Some(dphdp3) = root.get_mut_child("DPHDP3") {
        // <VetaD>
        if let Some(vetad) = dphdp3.get_mut_child("VetaD") {
            vetad.attributes.insert("d_poddp".to_string(), format!("{:02}.{:02}.{}", config.datum_podpisu.day(), config.datum_podpisu.month() as u8, config.datum_podpisu.year()));
            vetad.attributes.insert("mesic".to_string(), (config.datum_za_obdobi.month() as u8).to_string());
            vetad.attributes.insert("rok".to_string(), config.datum_za_obdobi.year().to_string());
            vetad.attributes.insert("zdobd_od".to_string(), zdobd_od);
            vetad.attributes.insert("zdobd_do".to_string(), zdobd_do);
        }
        
        // <Veta1> – USD  
        if let Some(veta1) = dphdp3.get_mut_child("Veta1") {
            veta1.attributes.insert("dan_psl23_e".to_string(), config.dph_prijeti_sluzeb_v_jinem_state_czk.to_string());
            veta1.attributes.insert("p_sl23_e".to_string(), config.prijeti_sluzeb_v_jinem_state_czk.to_string());
        }

        // <Veta2> – hodnota plnění CZK
        if let Some(veta2) = dphdp3.get_mut_child("Veta2") {
            veta2.attributes.insert("pln_sluzby".to_string(), config.hodnota_plneni_czk.to_string());
        }

        // <Veta4> – přijetí služeb v jiném státě
        if let Some(veta4) = dphdp3.get_mut_child("Veta4") {
            veta4.attributes.insert("nar_zdp23".to_string(), config.prijeti_sluzeb_v_jinem_state_czk.to_string());
            veta4.attributes.insert("od_zdp23".to_string(), config.dph_prijeti_sluzeb_v_jinem_state_czk.to_string());
            veta4.attributes.insert("odp_sum_nar".to_string(), dph_celkem.to_string());

            veta4.attributes.insert("odp_tuz23_nar".to_string(), round_dph_prijata.to_string());
            veta4.attributes.insert("pln23".to_string(), round_prijata.to_string());
        }

        // <Veta6> – celkový součet
        if let Some(veta6) = dphdp3.get_mut_child("Veta6") {
            veta6.attributes.insert("dan_zocelk".to_string(), config.dph_prijeti_sluzeb_v_jinem_state_czk.to_string());
            veta6.attributes.insert("dano_no".to_string(), round_dph_prijata.to_string());
            veta6.attributes.insert("odp_zocelk".to_string(), dph_celkem.to_string());
        }
    }

    // Build dynamic filename: DPH-YYYY-MM.xml
    let file_name = format!(
        "DPH-{}-{:02}.xml",
        config.datum_za_obdobi.year(),
        config.datum_za_obdobi.month() as u8
    );

    let mut out = File::create(&file_name).unwrap();
    root.write(&mut out).unwrap();

    println!("✅ DPH XML generated -> {file_name}");
}
