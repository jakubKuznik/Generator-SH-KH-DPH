# Generator-SH-KH-DPH

Nástroj pro automatické generování tří XML souborů pro podání finanční správě ČR:
- **SH** – Souhrnné hlášení
- **KH** – Kontrolní hlášení
- **DPH** – Daňové přiznání k DPH

## Požadavky
- !!! Před prvním použítím je potřeba doplnit své údaje do xml šablon !!! 
- Rust (aktuální stable verze)
- Připojení k internetu (pro stažení kurzů z ČNB)

## Příklad Použití
```shell
./target/release/generator-sh-kh-dph --hodnota-plneni 2773.47 --prijata-zdanitelna-plneni 2045.45 --prijeti-sluzeb-v-jinem-state 20.0 


cargo run -- --hodnota-plneni 2773.47 --prijata-zdanitelna-plneni 2045.45 --prijeti-sluzeb-v-jinem-state 20.0

# OpenAI faktura vystavená rovnou v Kč
cargo run -- --hodnota-plneni 2773.47 --prijata-zdanitelna-plneni 2045.45 --prijeti-sluzeb-v-jinem-state-czk 412.40

# Tuzemské daňové doklady nad 10 000 Kč včetně DPH
cargo run -- --hodnota-plneni 2773.47 --prijata-zdanitelna-plneni 2045.45 --prijeti-sluzeb-v-jinem-state-czk 412.40 --prijata-zdanitelna-plneni-nad-10000 19865.29 dic 27082440 danovydoklad 589791468 datum 25.3.2026
```

## Build  
```shell
cargo build --release
```

## Použití
```shell
./generator-sh-kh-dph --hodnota-plneni <EUR> --prijata-zdanitelna-plneni <CZK> (--prijeti-sluzeb-v-jinem-state <USD> | --prijeti-sluzeb-v-jinem-state-czk <CZK>) [--datum-za-obdobi <YYYY-MM-DD>]
```

### Parametry
- `--hodnota-plneni <EUR>`  
  Hodnota plnění v EUR (např. podnájem kanceláře, nákup mobilu).  
  Program automaticky přepočítá na CZK dle průměrného kurzu ČNB a zaokrouhlí nahoru na celé Kč.

- `--prijata-zdanitelna-plneni <CZK>`  
  Přijatá zdanitelná plnění v CZK **bez DPH** (např. faktura za software v ČR).  
  Maximálně položky do 10 000czk, ale suma může být větší 
  DPH se dopočítá automaticky sazbou 21 %.

- `--prijeti-sluzeb-v-jinem-state <USD>` 
  Přijetí služeb z jiného členského státu (např. licence ChatGPT) v USD **bez_dph**.  
  Přepočet na CZK dle průměrného kurzu ČNB, dopočítané DPH 21 %.

- `--prijeti-sluzeb-v-jinem-state-czk <CZK>`
  Přijetí služeb z jiného členského státu (např. licence ChatGPT) už fakturované v CZK **bez_dph**.
  Nepřepočítává se kurzem ČNB, jen se zaokrouhlí základ nahoru na celé Kč a dopočítá DPH 21 %.

- `--prijata-zdanitelna-plneni-nad-10000 <CZK>`
  Souhrn tuzemských daňových dokladů nad 10 000 Kč včetně DPH, bez DPH.
  DPH se dopočítá automaticky sazbou 21 %.
  Tyto částky se zahrnou do DPH ř. 40, do kontrolního řádku KH `VetaC/pln23` a do KH B.2.
  Vyžaduje také `dic <DIC>`, `danovydoklad <TEXT>` a `datum <D.M.YYYY>`.
  Hodnoty `pomer` a `zdph_44` se berou ze `vzory/KH-vzor-nad10.xml`.

- `--datum-za-obdobi <YYYY-MM-DD>` nebo `--datum <YYYY-MM-DD>`
  Volitelný parametr pro zpětné generování.
  Přepíše výchozí období (jinak se bere minulý měsíc).
  Zadaný měsíc/rok se použije i pro načtení měsíčního kurzu ČNB.
  

### Výstup
Program vygeneruje tři soubory:
- `SH-YYYY-MM.xml`
- `KH-YYYY-MM.xml`
- `DPH-YYYY-MM.xml`

Všechny šablony XML jsou uloženy ve složce `vzory/`.

## Struktura projektu
```shell
.
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── sh.rs
│ ├── kh.rs
│ ├── dph.rs
│ ├── date_func.rs # Funkce pro čas 
│ └── rates.rs # API volání na ČNB
├── vzory/ # Vzorove xml 
│ ├── sh-vzor.xml
│ ├── kh-vzor.xml
│ └── dph-vzor.xml
├── SH-YYYY-MM.xml
├── KH-YYYY-MM.xml
└── DPH-YYYY-MM.xml
```

## Poznámky
- Program používá **průměrný měsíční kurz ČNB** (`https://api.cnb.cz/cnbapi/exrates/monthly-averages-currency`) pro přepočet měn.  
- Hodnoty v CZK se zaokrouhlují **nahoru na celé koruny**.  
- Vygenerované XML odpovídá vzorům EPO (EPO MF ČR).  

## Todo 
- prijeti sluzeb v zahranici libovolna mena 

## Licence
MIT
