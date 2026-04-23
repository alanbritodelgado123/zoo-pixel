use rusqlite::{Connection, params};
use crate::escena::Escena;

#[derive(Debug, Clone)]
pub struct Animal {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub zona_id: String,
    pub nombre_comun: String,
    pub nombre_cientifico: String,
    pub descripcion: String,
    pub categoria: String,
}

#[derive(Debug, Clone)]
pub struct DialogoDB {
    pub id: i64,
    pub contexto: String,
    pub personaje: String,
    pub orden: i32,
    pub texto: String,
}

#[derive(Debug, Clone)]
pub struct QuizPreguntaDB {
    pub id: i64,
    pub pregunta: String,
    pub opcion_a: String,
    pub opcion_b: String,
    pub opcion_c: String,
    pub opcion_d: String,
    pub correcta: usize,
}

pub struct ZooDB {
    conn: Connection,
}

impl ZooDB {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory()
            .expect("No se pudo crear la base de datos");
        conn.execute_batch("
        CREATE TABLE zonas (
            id TEXT PRIMARY KEY,
            nombre TEXT NOT NULL
        );
        CREATE TABLE animales (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            zona_id TEXT NOT NULL,
            nombre_comun TEXT NOT NULL,
            nombre_cientifico TEXT NOT NULL,
            descripcion TEXT NOT NULL,
            categoria TEXT NOT NULL,
            FOREIGN KEY (zona_id) REFERENCES zonas(id)
        );
        CREATE TABLE dialogos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            contexto TEXT NOT NULL,
            personaje TEXT NOT NULL,
            orden INTEGER NOT NULL,
            texto TEXT NOT NULL
        );
        CREATE TABLE quiz_museo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pregunta TEXT NOT NULL,
            opcion_a TEXT NOT NULL,
            opcion_b TEXT NOT NULL,
            opcion_c TEXT NOT NULL,
            opcion_d TEXT NOT NULL,
            correcta INTEGER NOT NULL
        );
        ").expect("Error creando tablas");

        let db = Self { conn };
        db.poblar_zonas();
        db.poblar_animales();
        db.poblar_dialogos();
        db.poblar_quiz_museo();
        db
    }

    fn poblar_zonas(&self) {
        let zonas: &[(&str, &str)] = &[
            ("entrada",     "Entrada Principal"),
            ("pasillo_01",  "Pasillo 1"),
            ("pasillo_02",  "Pasillo 2"),
            ("pasillo_03",  "Pasillo 3"),
            ("pasillo_04",  "Pasillo 4"),
            ("pasillo_05",  "Pasillo 5 - Acuario"),
            ("zona_01",  "Zona 1: Llanos Centrales"),
            ("zona_02",  "Zona 2: Cordillera de los Andes"),
            ("zona_03",  "Zona 3: Selva Amazonica"),
            ("zona_04",  "Zona 4: Sierra de Perija"),
            ("zona_05",  "Zona 5: Peninsula de Paria"),
            ("zona_06",  "Zona 6: Isla de Margarita"),
            ("zona_07",  "Zona 7: Costa Caribe Oriental"),
            ("zona_08",  "Zona 8: Delta del Orinoco"),
            ("zona_09",  "Zona 9: Parque Nacional Canaima"),
            ("zona_10",  "Zona 10: Serrania de la Neblina"),
            ("zona_11",  "Zona 11: Cordillera de la Costa"),
            ("zona_12",  "Zona 12: Lago de Maracaibo"),
            ("zona_13",  "Zona 13: Sierra de San Luis"),
            ("zona_14",  "Zona 14: Los Roques"),
            ("zona_15",  "Zona 15: Rio Orinoco"),
            ("zona_16",  "Zona 16: Selva de Imataca"),
            ("zona_17",  "Zona 17: Paramos de Merida"),
            ("zona_18",  "Zona 18: Llanos Occidentales"),
            ("zona_19",  "Zona 19: Peninsula de la Guajira"),
            ("zona_20",  "Zona 20: Cerro Yapacana"),
        ("zona_21",  "Zona 21: Museo Paleontologia"),
        ("zona_22",  "Zona 22: Peces de Pesca"),        // ✅ CORREGIDO
        ("zona_23",  "Zona 23: Reptiles y Anfibios"),   // ✅ CORREGIDO
        ("zona_24",  "Zona 24: Insectos"),
        ("zona_25",  "Zona 25: Aves Llamativas"),       // ✅ CORREGIDO
        ];
        for (id, nombre) in zonas {
            self.conn.execute(
                "INSERT INTO zonas (id, nombre) VALUES (?1, ?2)",
                params![id, nombre],
            ).unwrap();
        }
    }

    fn poblar_animales(&self) {
        let animales: &[(&str, &str, &str, &str, &str)] = &[

            // ── ZONA 1: Llanos Centrales ─────────────────────────────
            ("zona_01","Chigüire","Hydrochoerus hydrochaeris",
             "Roedor más grande del mundo. Vive en grupos de hasta 20 individuos cerca del agua. Puede pesar hasta 65 kg. Es herbívoro y semiacuático. También conocido como Capibara en Colombia y Brasil, o Carpincho en Argentina y Uruguay.",
             "mamiferos"),
            ("zona_01","Cunaguaro","Leopardus pardalis",
             "Ocelote, cazador nocturno solitario. Pelaje con patrón único como huellas dactilares. Excelente trepador. Pesa hasta 16 kg. Conocido como Manigordo en Costa Rica y Panamá.",
             "mamiferos"),
            ("zona_01","Garza Real","Ardea alba",
             "Ave acuática de gran tamaño, alcanza hasta 1 metro de altura. Plumaje blanco puro y patas largas negras. Llamada Garceta Grande en España y Garza Blanca en México y Argentina.",
             "aves"),
            ("zona_01","Babilla","Caiman crocodilus",
             "Caimán pequeño que habita pantanos y ríos lentos de los llanos. Mide hasta 2.5 metros. Conocido como Cachirre en Colombia y Jacare-tinga en Brasil.",
             "reptiles"),

            // ── ZONA 2: Cordillera de los Andes ─────────────────────
            ("zona_02","Oso Frontino","Tremarctos ornatus",
             "Único oso de Sudamérica. Marcas faciales únicas en cada individuo. Puede pesar hasta 200 kg. Conocido como Oso de Anteojos en Perú y Ecuador, y Oso Andino en Bolivia y Chile.",
             "mamiferos"),
            ("zona_02","Condor de los Andes","Vultur gryphus",
             "Envergadura de 3.3 metros, una de las aves voladoras más grandes del mundo. Llamado Kuntur en la región andina en quechua, y Cóndor en Argentina y Chile.",
             "aves"),
            ("zona_02","Paujil de Copete de Piedra","Pauxi pauxi",
             "Ave terrestre de gran tamaño que habita bosques nublados andinos. En peligro crítico de extinción. Conocido como Mutún en Bolivia y Pavón en Colombia.",
             "aves"),
            ("zona_02","Venado Caramerudo","Odocoileus virginianus",
             "Habita bosques y sabanas. Los machos tienen astas ramificadas que renuevan cada año. Llamado Venado de cola blanca en México y EE.UU., y Guazú Virá en Uruguay.",
             "mamiferos"),

            // ── ZONA 3: Selva Amazónica ──────────────────────────────
            ("zona_03","Jaguar","Panthera onca",
             "Mayor felino de América. Mordida más poderosa de todos los felinos. Conocido como Yaguareté en Argentina y Paraguay, y Onça-pintada en Brasil.",
             "mamiferos"),
            ("zona_03","Guacamaya Bandera","Ara macao",
             "Ave emblemática de los trópicos americanos. Plumaje rojo, amarillo y azul brillante. Llamada Lapa roja en Costa Rica y Nicaragua, y Guacamayo rojo en Perú y Bolivia.",
             "aves"),
            ("zona_03","Anaconda Verde","Eunectes murinus",
             "Serpiente más pesada del mundo, hasta 250 kg y 9 metros de longitud. Conocida como Sucurí en Brasil y Yacu-mama en Perú y Ecuador.",
             "reptiles"),
            ("zona_03","Mono Araguato","Alouatta seniculus",
             "Mono aullador rojo. Su grito se escucha a 5 km gracias a un hueso hioideo amplificador. Conocido como Mono aullador rojo en Colombia y Guariba en Brasil.",
             "primates"),

            // ── ZONA 4: Sierra de Perijá ─────────────────────────────
            ("zona_04","Marimonda de la Sierra","Ateles hybridus",
             "Mono araña café en peligro crítico. Cola prensil que funciona como quinta mano. Conocido como Choiba en Colombia.",
             "primates"),
            ("zona_04","Tucan Real","Ramphastos sulfuratus",
             "Su pico puede medir hasta 20 cm pero es muy ligero por estructura hueca. Conocido como Tucán de pico canoa en México y Piapoco en Colombia.",
             "aves"),
            ("zona_04","Puma","Puma concolor",
             "León americano. Habita desde Canadá hasta la Patagonia. Solitario y territorial. Conocido como Trapial en la cultura Mapuche de Chile.",
             "mamiferos"),
            ("zona_04","Pava Negra","Aburria aburri",
             "Ave grande terrestre de bosques nublados andinos. Plumaje negro brillante. Conocida como Pava aburria en Colombia y Ecuador.",
             "aves"),

            // ── ZONA 5: Península de Paria ───────────────────────────
            ("zona_05","Colibri Tijereta de Paria","Hylonympha macrocerca",
             "Colibrí endémico de la Península de Paria. Cola larga bifurcada en el macho. Conocido como Scissor-tailed Hummingbird en Trinidad y Guyana.",
             "aves"),
            ("zona_05","Mono Capuchino","Cebus olivaceus",
             "Usa herramientas como piedras para abrir nueces. Considerado el primate más inteligente de Venezuela. Conocido como Mono maicero en Colombia y Caiarara en Brasil.",
             "primates"),
            ("zona_05","Perezoso de Tres Dedos","Bradypus variegatus",
             "Mamífero arborícola de movimientos lentos. Pasa la mayor parte de su vida colgado de ramas. Conocido como Perico ligero en Panamá y Bicho-preguiça en Brasil.",
             "mamiferos"),
            ("zona_05","Cuchicuchi","Potos flavus",
             "Mamífero nocturno arborícola con cola prensil. Se alimenta principalmente de frutas y néctar. Conocido como Kinkajú en México y Centroamérica, y Martucha en México.",
             "mamiferos"),

            // ── ZONA 6: Isla de Margarita ────────────────────────────
            ("zona_06","Venado de Margarita","Odocoileus virginianus margaritae",
             "Subespecie endémica de Isla de Margarita. Más pequeño que venados continentales. En peligro crítico de extinción por pérdida de hábitat.",
             "mamiferos"),
            ("zona_06","Cotorra Margariteña","Amazona barbadensis",
             "Loro endémico de zonas áridas. Plumaje verde con frente amarilla. Conocido como Cotorra cabeciamarilla en Bonaire y Lora de hombros amarillos en Aruba.",
             "aves"),
            ("zona_06","Nangaro","Psittacara wagleri",
             "Perico de frente roja. Ruidoso y gregario. Conocido como Perico frentirrojo en Colombia y Aratinga frentirroja en Perú.",
             "aves"),
            ("zona_06","Cunaguaro de Margarita","Leopardus pardalis pardalis",
             "Subespecie de ocelote de Isla de Margarita. Pelaje con manchas características. Conocido como Ocelot en EE.UU. y Tigrillo en Ecuador.",
             "mamiferos"),

            // ── ZONA 7: Costa Caribe Oriental ────────────────────────
            ("zona_07","Flamenco del Caribe","Phoenicopterus ruber",
             "Flamenco de color rosado intenso debido a carotenoides en su dieta. Conocido como Flamenco rosado en México y Parina grande en Chile.",
             "aves"),
            ("zona_07","Tortuga Carey","Eretmochelys imbricata",
             "Tortuga marina en peligro crítico. Caparazón con escudos superpuestos. Conocida como Tartaruga-de-pente en Brasil y Tortuga de concha en México.",
             "reptiles"),
            ("zona_07","Corocoro Rojo","Eudocimus ruber",
             "Ibis de plumaje rojo escarlata intenso. Color debido a carotenoides en su dieta. Conocido como Ibis escarlata en Trinidad y Tobago, y Guará en Brasil.",
             "aves"),
            ("zona_07","Mero Guasa","Epinephelus itajara",
             "Mero gigante de arrecifes. Puede pesar hasta 400 kg y medir 2.5 metros. Conocido como Goliath Grouper en EE.UU. y Mero gigante en México.",
             "peces"),
            ("zona_07","Fragata","Fregata magnificens",
             "Ave marina con envergadura de 2.3 metros. Macho tiene saco gular rojo que infla para atraer hembras. Conocida como Tijereta de mar en México y Tesourão en Brasil.",
             "aves"),
            ("zona_07","Caiman de la Costa","Crocodylus acutus",
             "Cocodrilo americano, habita costas y estuarios. Conocido como Cocodrilo americano en México y Centroamérica, y Lagarto amarillo en Cuba.",
             "reptiles"),

            // ── ZONA 8: Delta del Orinoco ────────────────────────────
            ("zona_08","Manati del Caribe","Trichechus manatus",
             "Mamífero acuático herbívoro que puede pesar hasta 500 kg. Conocido como Vaca marina en México y Peixe-boi en Brasil.",
             "mamiferos"),
            ("zona_08","Perro de Agua","Pteronura brasiliensis",
             "Nutria gigante, la más grande del mundo, hasta 1.8 metros. Conocida como Nutria gigante en Perú y Colombia, y Ariranha en Brasil.",
             "mamiferos"),
            ("zona_08","Garzon Soldado","Jabiru mycteria",
             "Cigüeña enorme, la mayor ave voladora de América. Conocido como Tuyuyú en Argentina y Paraguay, y Tuiuiu en Brasil.",
             "aves"),
            ("zona_08","Pavon Real","Cichla orinocensis",
             "Depredador de agua dulce muy apreciado en pesca deportiva. Puede pesar hasta 12 kg. Conocido como Tucunaré en Brasil y Peacock Bass en EE.UU.",
             "peces"),
            ("zona_08","Mono Capuchino del Orinoco","Cebus albifrons",
             "Mono capuchino de frente blanca. Inteligente y adaptable. Conocido como Machín blanco en Perú y Mono blanco en Ecuador.",
             "primates"),
            ("zona_08","Pava de Monte","Penelope jacquacu",
             "Ave grande terrestre de selvas inundables. Plumaje café oscuro. Conocida como Pava amazónica en Bolivia y Jacu en Brasil.",
             "aves"),

            // ── ZONA 9: Parque Nacional Canaima ─────────────────────
            ("zona_09","Sapito Minero","Dendrobates leucomelas",
             "Rana venenosa de piel negra con bandas amarillas brillantes. Conocida como Rana flecha moteada en Colombia y Bumblebee poison frog en EE.UU. y Guyana.",
             "anfibios"),
            ("zona_09","Harpia","Harpia harpyja",
             "Águila más poderosa del mundo. Envergadura de 2 metros. Conocida como Águila monera en México y Uiraçu-verdadeiro en Brasil.",
             "aves"),
            ("zona_09","Oso Hormiguero Gigante","Myrmecophaga tridactyla",
             "Lengua de 60 cm para atrapar hormigas y termitas. Puede comer hasta 35000 insectos al día. Conocido como Tamanduá-bandeira en Brasil y Yurumí en Argentina y Paraguay.",
             "mamiferos"),
            ("zona_09","Tepuihyla","Tepuihyla rodriguezi",
             "Rana endémica de cimas de tepuyes. Evolucionada en aislamiento durante millones de años. Conocida como Rana de los tepuyes en Guyana y Rodriguez's treefrog internacionalmente.",
             "anfibios"),

            // ── ZONA 10: Serranía de la Neblina ─────────────────────
            ("zona_10","Gallo de Roca Guayanes","Rupicola rupicola",
             "Macho de plumaje naranja brillante espectacular. Realiza danzas en arenas comunales. Conocido como Gallito de las rocas en Guyana y Galo-da-serra en Brasil.",
             "aves"),
            ("zona_10","Uacari de Cabeza Negra","Cacajao melanocephalus",
             "Primate de cara negra y pelaje largo. Vive en grupos grandes en la selva amazónica. Conocido como Mono uacarí en Brasil y Uakari internacionalmente.",
             "primates"),
            ("zona_10","Saltarin de Cabeza Dorada","Ceratopipra erythrocephala",
             "Ave pequeña, macho con cabeza amarillo dorado brillante. Conocida como Manakin cabecidorado en Colombia y Tangará-de-cabeça-amarela en Brasil.",
             "aves"),
            ("zona_10","Mono Viuda","Cheracebus lugens",
             "Mono pequeño de pelaje denso negro. Vive en parejas monógamas en selvas densas. Conocido como Zogue-zogue en Brasil y Tití de collar en Colombia.",
             "primates"),

            // ── ZONA 11: Cordillera de la Costa ─────────────────────
            ("zona_11","Paují de Copete","Pauxi pauxi",
             "Ave grande terrestre con casco óseo en la cabeza. En peligro crítico de extinción. Conocido como Helmeted Curassow en EE.UU. y Mutún pava en Bolivia.",
             "aves"),
            ("zona_11","Quetzal Dorado","Pharomachrus fulgidus",
             "Ave de plumaje iridiscente verde dorado y rojo. Cola larga en machos. Conocido como White-tipped Quetzal internacionalmente.",
             "aves"),
            ("zona_11","Sorocua Acollarado","Trogon collaris",
             "Ave de plumaje verde metálico y pecho rojo. Cola larga con patrón blanco y negro. Conocido como Trogón de collar en México y Surucuá-de-coleira en Brasil.",
             "aves"),
            ("zona_11","Tigre Mariposa","Panthera onca",
             "Jaguar, denominación local venezolana. Mayor felino de América. Mordida más poderosa de todos los felinos. Conocido como Yaguareté en el Cono Sur.",
             "mamiferos"),

            // ── ZONA 12: Lago de Maracaibo ───────────────────────────
            ("zona_12","Pato Cuchara","Anas clypeata",
             "Pato con pico en forma de cuchara para filtrar alimento del agua. Conocido como Pato cucharón en México y Cuchareta en España.",
             "aves"),
            ("zona_12","Chavarri","Chauna chavaria",
             "Ave grande semiacuática, hasta 90 cm. Grito estridente que se escucha a larga distancia. Conocido como Northern Screamer internacionalmente y Gritón en Colombia.",
             "aves"),
            ("zona_12","Pavon de Maracaibo","Cichla temensis",
             "Pez depredador más grande del género, puede alcanzar 15 kg. Conocido como Tucunaré açu en Brasil y Speckled peacock bass en EE.UU.",
             "peces"),
            ("zona_12","Bagre de Maracaibo","Perrunichthys perruno",
             "Bagre de patrón de manchas tipo leopardo. Especie endémica del Lago de Maracaibo. Conocido como Reticulated Catfish en EE.UU. y Bagre leopardo en Colombia.",
             "peces"),

            // ── ZONA 13: Sierra de San Luis ──────────────────────────
            ("zona_13","Guacharo","Steatornis caripensis",
             "Única ave frugívora nocturna que usa ecolocalización. Habita cuevas profundas. Conocido como Ave de las cavernas en Perú y Huácharo en Colombia.",
             "aves"),
            ("zona_13","Trogon Grande","Trogon massena",
             "Trogón más grande de Venezuela. Plumaje verde oscuro y pecho rojo intenso. Conocido como Trogón colioscuro en México y Sorocuá en Brasil.",
             "aves"),
            ("zona_13","Armadillo Gigante","Priodontes maximus",
             "Armadillo más grande del mundo. Puede pesar hasta 60 kg. Conocido como Cuspa Cachicamo en Venezuela, Tatu-canastra en Brasil.",
             "mamiferos"),
            ("zona_13","Mono Silvador","Cebus olivaceus",
             "Mono capuchino adaptable y social. Usa herramientas para obtener alimento. Conocido como Mono silvador en Bolivia y Caí en Argentina.",
             "primates"),

            // ── ZONA 14: Los Roques ──────────────────────────────────
            ("zona_14","Pez Loro","Scarus guacamaia",
             "Pez loro más grande del Atlántico, hasta 1.2 metros. Conocido como Rainbow Parrotfish en EE.UU. y Peixe-papagaio en Brasil.",
             "peces"),
            ("zona_14","Langosta Espinosa","Panulirus argus",
             "Langosta sin pinzas con antenas largas. Importante especie comercial. Conocida como Caribbean Spiny Lobster en EE.UU. y Lagosta-pintada en Brasil.",
             "insectos"),
            ("zona_14","Botuto","Lobatus gigas",
             "Caracol marino gigante, hasta 30 cm. Concha rosada por dentro. Conocido como Caracol pala en Colombia y Queen Conch en Bahamas.",
             "insectos"),
            ("zona_14","Pez Angel Frances","Pomacanthus paru",
             "Pez de arrecife de color negro con bordes amarillos. Conocido como French Angelfish en EE.UU. y Frade en Brasil.",
             "peces"),
            ("zona_14","Tortuga Verde","Chelonia mydas",
             "Tortuga marina herbívora que anida en playas venezolanas. Conocida como Tartaruga-verde en Brasil y Tortuga blanca en México.",
             "reptiles"),
            ("zona_14","Barracuda","Sphyraena barracuda",
             "Pez depredador de arrecifes de gran velocidad. Dientes afilados como cuchillas. Conocida como Picúa en Cuba y Puerto Rico.",
             "peces"),

            // ── ZONA 15: Río Orinoco ─────────────────────────────────
            ("zona_15","Tonina","Inia geoffrensis",
             "Delfín rosado de agua dulce, el más grande del mundo. Conocido como Boto en Brasil y Delfín rosado en Colombia y Perú.",
             "mamiferos"),
            ("zona_15","Caiman del Orinoco","Crocodylus intermedius",
             "En peligro crítico, quedan menos de 1500 individuos. Conocido como Orinoco Crocodile en EE.UU. y Cocodrilo del Orinoco en Colombia.",
             "reptiles"),
            ("zona_15","Lau-lau","Brachyplatystoma filamentosum",
             "Bagre gigante de agua dulce, puede pesar hasta 200 kg. Conocido como Piraíba en Brasil y Pirabuton en Perú.",
             "peces"),
            ("zona_15","Raya Motoro","Potamotrygon motoro",
             "Raya de agua dulce con patrón de manchas oceladas. Aguijón venenoso en la cola. Conocida como Ocellated river stingray en EE.UU. y Arraia-limão en Brasil.",
             "peces"),
            ("zona_15","Payara","Hydrolycus scomberoides",
             "Pez depredador con colmillos enormes que sobresalen de la mandíbula. Conocido como Pez perro en Colombia y Peixe-cachorro en Brasil.",
             "peces"),
            ("zona_15","Caribe Colorado","Pygocentrus nattereri",
             "Piraña de vientre rojo. Dientes afilados como navajas. Vive en cardúmenes. Conocido como Piranha-caju en Brasil y Piraña de vientre rojo en Argentina.",
             "peces"),

            // ── ZONA 16: Selva de Imataca ────────────────────────────
            ("zona_16","Aguila Crestada","Morphnus guianensis",
             "Rapaz poderosa de selvas tropicales. Cresta larga eréctil. Conocida como Crested Eagle en EE.UU. y Gavião-real-falso en Brasil.",
             "aves"),
            ("zona_16","Mono Maicero","Sapajus apella",
             "Mono capuchino robusto y adaptable. Usa herramientas para abrir nueces. Conocido como Macaco-prego en Brasil.",
             "primates"),
            ("zona_16","Danta","Tapirus terrestris",
             "Mayor mamífero terrestre de Sudamérica, hasta 300 kg. Tiene una pequeña trompa prensil. Conocida como Tapir en México y Ecuador, y Anta en Brasil.",
             "mamiferos"),
            ("zona_16","Culebra Lora","Leptophis ahaetulla",
             "Serpiente arborícola delgada de color verde brillante. No venenosa. Conocida como Bejuquilla verde en México y Cobra-cipó en Brasil.",
             "reptiles"),

            // ── ZONA 17: Páramos de Mérida ───────────────────────────
            ("zona_17","Pato de Torrente","Merganetta armata",
             "Especializado en ríos de montaña de corriente rápida. Nada contra corrientes fuertes. Conocido como Pato de los torrentes en Chile y Argentina.",
             "aves"),
            ("zona_17","Aguila Real de los Andes","Geranoaetus melanoleucus",
             "Rapaz de gran tamaño de zonas montañosas. Envergadura de 2 metros. Conocida como Águila mora en Argentina y Chile, y Águila escudada en Perú.",
             "aves"),
            ("zona_17","Conejo de Paramo","Sylvilagus varynaensis",
             "Conejo endémico de páramos venezolanos. Pelaje denso adaptado al frío. Conocido como Venezuelan Lowland Rabbit en EE.UU.",
             "mamiferos"),
            ("zona_17","Musarana de los Andes","Cryptotis meridensis",
             "Pequeño mamífero insectívoro endémico de los Andes venezolanos. Conocida como Merida Shrew en EE.UU.",
             "mamiferos"),

            // ── ZONA 18: Llanos Occidentales ────────────────────────
            ("zona_18","Oso Palmero","Myrmecophaga tridactyla",
             "Lengua de 60 cm para atrapar hormigas y termitas. Puede comer hasta 35000 insectos al día. Conocido como Gran Hormiguero en Argentina y Tamanduá-bandeira en Brasil.",
             "mamiferos"),
            ("zona_18","Galapago Llanero","Podocnemis vogli",
             "Tortuga de agua dulce endémica de los llanos. Caparazón aplanado color oliva. Conocida como Sabanera en Colombia y Savannah Side-necked Turtle internacionalmente.",
             "reptiles"),
            ("zona_18","Zorro Cangrejero","Cerdocyon thous",
             "Cánido omnívoro de sabanas y bosques. Pelaje grisáceo. Come cangrejos y frutas. Conocido como Zorro de monte en Argentina y Cachorro-do-mato en Brasil.",
             "mamiferos"),
            ("zona_18","Pavon Negro","Crax alector",
             "Ave grande terrestre de bosques. Plumaje negro con cresta rizada. Conocido como Mutum-do-norte en Brasil.",
             "aves"),

            // ── ZONA 19: Península de la Guajira ────────────────────
            ("zona_19","Cardenal Guajiro","Cardinalis phoeniceus",
             "Ave de plumaje rojo intenso en el macho. Endémica de la región Caribe. Conocida como Vermilion Cardinal en EE.UU. y Cardenal de la Guajira en Colombia.",
             "aves"),
            ("zona_19","Conejo Sabanero","Sylvilagus floridanus",
             "Conejo común de sabanas y matorrales. Muy adaptable. Conocido como Conejo de Florida en México y Tapeti en Brasil.",
             "mamiferos"),
            ("zona_19","Cascabel","Crotalus durissus",
             "Serpiente venenosa con cascabel en la cola. Veneno hemotóxico y neurotóxico. Conocida como Boicininga en Brasil y Víbora de cascabel en México.",
             "reptiles"),
            ("zona_19","Turpial Guajiro","Icterus icterus ridgwayi",
             "Subespecie del turpial nacional venezolano. Plumaje naranja y negro brillante. Conocido como Troupial en Aruba y Curaçao.",
             "aves"),

            // ── ZONA 20: Cerro Yapacana ──────────────────────────────
            ("zona_20","Sapito de Yapacana","Minyobates steyermarki",
             "Rana venenosa endémica del Cerro Yapacana. Piel negra con franjas amarillas. Conocida como Demonic Poison Frog internacionalmente.",
             "anfibios"),
            ("zona_20","Halcon de Monte","Micrastur semitorquatus",
             "Rapaz de bosques densos. Alas cortas y cola larga para vuelo en vegetación. Conocido como Gavião-relógio en Brasil.",
             "aves"),
            ("zona_20","Oso Melero","Tamandua tetradactyla",
             "Oso hormiguero menor arborícola. Cola prensil. Lengua larga pegajosa. Conocido como Tamandúa en Argentina y Uruguay, y Mixila en Brasil.",
             "mamiferos"),
            ("zona_20","Tucancito de Pico Maculado","Selenidera culik",
             "Tucán pequeño de pico multicolor con manchas. Conocido como Tucancinho en Brasil y Guyanan Toucanet en EE.UU.",
             "aves"),

            // ── ZONA 21: Museo Paleontología ─────────────────────────
            ("zona_21","Pereza Gigante","Megatherium americanum",
             "Perezoso gigante del Pleistoceno, del tamaño de un elefante. Podía alcanzar 6 metros de altura parado. Sus restos han sido hallados en Taima-Taima, Falcón. Hallazgos similares en Argentina, Uruguay, Brasil y Chile.",
             "fosiles"),
            ("zona_21","Tigre Dientes de Sable","Smilodon populator",
             "Felino prehistórico con colmillos de hasta 30 cm. Habitó América del Sur durante el Pleistoceno. Restos hallados en Venezuela. Hallazgos similares en Brasil, Argentina, Bolivia y Uruguay.",
             "fosiles"),
            ("zona_21","Mastodonte Sudamericano","Notiomastodon platensis",
             "Elefante prehistórico de América del Sur. Pariente del mamut. Sus restos aparecen en el yacimiento de El Breal de Orocual, Monagas. Hallazgos similares en Colombia, Ecuador, Brasil y Argentina.",
             "fosiles"),
            ("zona_21","Gliptodonte","Glyptotherium venezuelensis",
             "Armadillo gigante prehistórico del Pleistoceno. Caparazón óseo completo de hasta 3 metros. Especie específica de Venezuela. Géneros afines en EE.UU., México y Brasil.",
             "fosiles"),

// ── ZONA 22: Peces de Pesca (Z5-2) ───────────────────────────
("zona_22","Carite Rey","Scomberomorus cavalla",
 "Pez marino muy apreciado en la gastronomía venezolana costera. Nada en cardúmenes a gran velocidad. Conocido como King Mackerel en EE.UU., Sierra en Panamá y Colombia, y Cavala en Brasil.",
 "peces"),
("zona_22","Pargo Rojo","Lutjanus campechanus",
 "Pez de arrecife y fondos rocosos. Carne blanca y firme muy apreciada. Conocido como Huachinango en México, Red Snapper en EE.UU. y Chillo en Puerto Rico.",
 "peces"),
("zona_22","Sapara","Semaprochilodus laticeps",
 "Pez emblemático de la Feria de la Sapara en el estado Bolívar. Migratorio, forma cardúmenes enormes. Conocido como Jaraqui en Brasil y Boquichico en Perú y Colombia.",
 "peces"),
("zona_22","Sabalo","Megalops atlanticus",
 "Pez deportivo de gran tamaño y saltos espectaculares. Puede vivir en agua dulce y salada. Conocido como Tarpon en EE.UU. y Tarpão en Brasil.",
 "peces"),

// ── ZONA 23: Reptiles y Anfibios (Z5-3) ──────────────────────
("zona_23","Iguana Verde","Iguana iguana",
 "Lagarto arborícola herbívoro. Puede medir 2 metros con cola. Tiene un tercer ojo parietal. Conocida como Gallina de palo en Panamá y Puerto Rico, y Teiú-verde en Brasil.",
 "reptiles"),
("zona_23","Tuqueque","Thecadactylus rapicauda",
 "Gecko nocturno común en viviendas. Cola en forma de nabo que puede desprender. Conocido como Geco de cola de nabo en México y Salamanquesa en España y Colombia.",
 "reptiles"),
("zona_23","Mapanare","Bothrops asper",
 "Serpiente venenosa muy peligrosa. Responsable de mayoría de mordeduras en Venezuela. Conocida como Terciopelo en Costa Rica y Barba amarilla en Guatemala y Honduras.",
 "reptiles"),
("zona_23","Rana Platanera","Boana xerophylla",
 "Rana arborícola común en zonas húmedas. Discos adhesivos en dedos para trepar. Conocida como Rana de lluvia en Colombia y Rana arborícola común en Trinidad.",
 "anfibios"),
("zona_23","Morrocoy Sabanero","Chelonoidis carbonarius",
 "Tortuga terrestre de patas rojas. Caparazón negro con manchas amarillas y rojas. Conocido como Tortuga de patas rojas en Argentina y Jabuti-piranga en Brasil.",
 "reptiles"),
("zona_23","Cascabel de Venezuela","Crotalus durissus cumanensis",
 "Subespecie de cascabel endémica de Venezuela. Veneno hemotóxico y neurotóxico muy potente. Conocida como Boicininga en Brasil y Víbora de cascabel en México.",
 "reptiles"),
("zona_23","Sapito Amarillo de la Carbonera","Atelopus carbonerensis",
 "Rana arlequín endémica de Venezuela en peligro crítico. Coloración de advertencia amarilla. Conocida como Jambato en Ecuador.",
 "anfibios"),
("zona_23","Tortuga Arrau","Podocnemis expansa",
 "Mayor tortuga de río de Sudamérica, caparazón de hasta 90 cm. Conocida como Charapa en Perú y Tartaruga-da-amazônia en Brasil.",
 "reptiles"),
("zona_23","Caiman Enano","Paleosuchus palpebrosus",
 "Caimán más pequeño, hasta 1.5 metros. Piel muy osificada. Conocido como Jacaré-pagua en Brasil y Cachirre en Colombia.",
 "reptiles"),
("zona_23","Coral de Bandas","Micrurus isozonus",
 "Serpiente venenosa de colores rojo, negro y amarillo. Veneno neurotóxico muy potente. Conocida como Serpiente de coral en México y Cobra-coral en Brasil.",
 "reptiles"),
("zona_23","Lagartija de Jardin","Ameiva ameiva",
 "Lagartija diurna muy ágil y territorial. Lengua bífida y movimientos rápidos. Conocida como Borrego en Panamá y Calango en Brasil.",
 "reptiles"),
("zona_23","Sapo Comun Cururu","Rhinella marina",
 "Sapo de gran tamaño con glándulas venenosas parotoides. Muy adaptable a entornos humanos. Conocido como Sapo de caña en Australia y Sapo buey en Centroamérica.",
 "anfibios"),

            // ── ZONA 24: Insectos ────────────────────────────────────
            ("zona_24","Mariposa Morpho Azul","Morpho helenor",
             "Mariposa de alas azul metálico iridiscente. Envergadura de hasta 15 cm. Conocida como Morpho azul en México y Panapaná en Brasil.",
             "insectos"),
            ("zona_24","Escarabajo Hercules","Dynastes hercules",
             "Uno de los escarabajos más grandes del mundo, hasta 17 cm. Conocido como Escarabajo rinoceronte en Colombia y Cornizuelo en Centroamérica.",
             "insectos"),
            ("zona_24","Bachaco Culon","Atta laevigata",
             "Hormiga cortadora de hojas. Viven en colonias de millones de individuos. Conocida como Hormiga arriera en Colombia y Hormiga cortadora en Argentina y Uruguay.",
             "insectos"),
            ("zona_24","Mariposa Monarca","Danaus plexippus",
             "Mariposa migratoria que recorre miles de kilómetros. Colores naranja y negro advierten toxicidad. Conocida como Monarca en México y EE.UU., y Mariposa del algodoncillo en España.",
             "insectos"),
            ("zona_24","Hormiga Veinticuatro","Paraponera clavata",
             "Posee la picadura más dolorosa del reino animal. El dolor dura 24 horas. Conocida como Hormiga bala en Brasil y Costa Rica, y Hormiga tocantera en Bolivia.",
             "insectos"),
            ("zona_24","Escarabajo Arlequin","Acrocinus longimanus",
             "Escarabajo de colores vivos rojo, negro y amarillo. Sus patas delanteras son extremadamente largas. Conocido como Arlequín de la madera en México y Serrucho en Colombia.",
             "insectos"),
            ("zona_24","Machaca","Fulgora laternaria",
             "Insecto homóptero con cabeza alargada en forma de maní. Existe el mito de que su mordedura es mortal. Conocido como Insecto linterna en Brasil y Víbora voladora en Centroamérica.",
             "insectos"),
            ("zona_24","Mariposa Buho","Caligo idomeneus",
             "Mariposa nocturna con manchas oculares en las alas que imitan ojos de búho. Conocida como Ojo de búho en Costa Rica y Borboleta-coruja en Brasil.",
             "insectos"),
            ("zona_24","Libelula Gigante","Megaloprepus caerulatus",
             "La libélula más grande del mundo con envergadura de hasta 19 cm. Conocida como Helicóptero en Panamá y Caballito del diablo gigante en México.",
             "insectos"),
            ("zona_24","Escarabajo Metalico","Euchroma gigantea",
             "Escarabajo de colores metálicos brillantes verde y rojo. Sus élitros se usan en artesanías tradicionales. Conocido como Joya del bosque en Colombia.",
             "insectos"),
            ("zona_24","Mantis Religiosa de Bosque","Stagmatoptera septentrionalis",
             "Mantis de gran tamaño adaptada a bosques tropicales. Camuflaje perfecto entre hojas y ramas. Conocida como Santateresa en España y Rezadora en Colombia y México.",
             "insectos"),
            ("zona_24","Abejorro Carpintero","Xylocopa latipes",
             "Abejorro negro de gran tamaño que excava nidos en madera. Polinizador importante. Conocido como Abejorro negro en Argentina y Mamangava en Brasil.",
             "insectos"),

// ── ZONA 25: Aves Llamativas (Z5-5) ──────────────────────────
("zona_25","Turpial","Icterus icterus",
 "Ave nacional de Venezuela. Plumaje naranja brillante y negro. Canto melodioso y complejo. Conocido como Troupial en EE.UU. y las Islas del Caribe.",
 "aves"),
("zona_25","Cardenalito","Spinus cucullatus",
 "Ave pequeña endémica de Venezuela, en peligro crítico de extinción. Macho de plumaje rojo intenso. Conocido como Red Siskin internacionalmente y Jilguero rojo en Colombia.",
 "aves"),
("zona_25","Campanero Blanco","Procnias albus",
 "Posee el canto más fuerte registrado en aves, alcanzando 125 decibelios. Conocido como White Bellbird en Guyana y Surinam, y Araponga-da-amazônia en Brasil.",
 "aves"),
("zona_25","Guacamaya Azul y Amarilla","Ara ararauna",
 "Guacamaya grande de plumaje azul intenso y pecho amarillo dorado. Conocida como Canindé en Brasil y Guacamayo azulamarillo en Perú, Bolivia y Ecuador.",
 "aves"),
("zona_25","Gallito de las Rocas Andino","Rupicola peruvianus",
 "Macho de plumaje naranja intenso espectacular. Realiza danzas elaboradas. Conocido como Tunqui en Perú y Gallo de la peña en Colombia y Ecuador.",
 "aves"),
("zona_25","Paraulata Llanera","Mimus gilvus",
 "Excelente imitador de cantos de otras aves. Plumaje gris pardusco. Muy común en sabanas. Conocida como Tropical Mockingbird internacionalmente.",
 "aves"),

            // ── PESCA P5 (Acuario) ────────────────────────────────────
            ("pasillo_05","Curbinata","Plagioscion squamosissimus",
             "Pez de río muy apreciado en la gastronomía venezolana. Carne blanca y firme.",
             "peces"),
            ("pasillo_05","Coporo","Prochilodus mariae",
             "Pez herbívoro migratorio del Orinoco. Forma cardúmenes enormes en época de subienda.",
             "peces"),
            ("pasillo_05","Valenton","Brachyplatystoma rousseauxii",
             "Uno de los bagres más grandes del Orinoco. Realiza migraciones de miles de kilómetros.",
             "peces"),
            ("pasillo_05","Morocoto","Colossoma macropomum",
             "Pez grande herbívoro. Se alimenta de frutos que caen al río durante las inundaciones.",
             "peces"),
        ];

        for (zona, comun, cientifico, desc, categoria) in animales {
            self.conn.execute(
                "INSERT INTO animales
                 (zona_id, nombre_comun, nombre_cientifico, descripcion, categoria)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![zona, comun, cientifico, desc, categoria],
            ).unwrap();
        }
    }

    fn poblar_dialogos(&self) {
        let dialogos: &[(&str, &str, i32, &str)] = &[
            // ── Bienvenida General ───────────────────────────────────
            ("bienvenida", "Guia Eli", 1,
             "¡Bienvenido al Zoológico Nacional de Venezuela! Soy Eli, tu guía personal."),
            ("bienvenida", "Guia Eli", 2,
             "Aquí conocerás la increíble fauna de Venezuela, desde los llanos hasta las montañas andinas."),
            ("bienvenida", "Guia Eli", 3,
             "Usa las flechas para moverte entre las zonas. Cada una representa un ecosistema diferente."),
            ("bienvenida_teclado", "Guia Eli", 4,
             "Presiona Z para explorar y ver los animales. Con X puedes volver atrás."),
            ("bienvenida_tactil", "Guia Eli", 4,
             "Toca el botón A para explorar y ver los animales. Con B puedes volver atrás."),
            ("bienvenida", "Guia Eli", 5,
             "Tienes una libreta de campo donde se guardan automáticamente los animales que descubras."),
            ("bienvenida", "Guia Eli", 6,
             "El mapa te ayudará a orientarte. Puedes abrirlo presionando M en cualquier momento."),
            ("bienvenida", "Guia Eli", 7,
             "¡Venezuela tiene una biodiversidad increíble: más de 1400 especies de aves!"),
            ("bienvenida", "Guia Eli", 8,
             "¡Explora las 25 zonas del parque! Buena suerte en tu expedición."),

            // ── Museo (Guía Ani) ─────────────────────────────────────
            ("museo_bienvenida", "Guia Ani", 1,
             "¡Hola! Soy Ani, tu guía en el Museo Paleontológico de Paria."),
            ("museo_bienvenida", "Guia Ani", 2,
             "Aquí podrás explorar fósiles de la megafauna del Pleistoceno venezolano."),
            ("museo_bienvenida", "Guia Ani", 3,
             "También puedes excavar para encontrar fósiles escondidos en la roca."),
            ("museo_bienvenida", "Guia Ani", 4,
             "¡Y pon a prueba tu conocimiento en el quiz paleontológico! Usa Z para seleccionar."),

            // ── Pesca P5 ─────────────────────────────────────────────
            ("pesca_bienvenida", "Guia Eli", 1,
             "¡Bienvenido al Acuario del Parque! Aquí puedes pescar especies de agua dulce venezolanas."),
            ("pesca_bienvenida", "Guia Eli", 2,
             "Espera con paciencia hasta que el pez pique. Cuando veas !! PICA !! presiona Z rápido."),
            ("pesca_bienvenida", "Guia Eli", 3,
             "Cada pez capturado se registra automáticamente en tu libreta de campo. ¡Buena pesca!"),

            // ── Foto Z5_5 ────────────────────────────────────────────
            ("foto_bienvenida", "Guia Eli", 1,
             "¡Bienvenido a la Zona de Aves muy llamativas! Esta es la zona más increíble del parque."),
            ("foto_bienvenida", "Guia Eli", 2,
             "Venezuela alberga las especies más hermosas"),
            ("foto_bienvenida", "Guia Eli", 3,
             "Presiona Z para tomar fotografías, y terminar el recorrido con broche de oro"),

            // ── Callejones Zx_5 ──────────────────────────────────────
            ("callejon_zona_05", "Guia Eli", 1,
             "¡Estás en la Península de Paria, hogar de especies únicas endémicas!"),
            ("callejon_zona_05", "Guia Eli", 2,
             "El Colibrí Tijereta de Paria solo existe aquí."),
            ("callejon_zona_05", "Guia Eli", 3,
             "Regresa a los pasillos para explorar otras zonas."),

            ("callejon_zona_10", "Guia Eli", 1,
             "La Serranía de la Neblina alberga especies que no existen en ningún otro lugar del planeta."),
            ("callejon_zona_10", "Guia Eli", 2,
             "Sus bosques neblinosos son uno de los ecosistemas más biodiversos de Venezuela."),
            ("callejon_zona_10", "Guia Eli", 3,
             "Regresa a los pasillos para explorar otras zonas."),

            ("callejon_zona_15", "Guia Eli", 1,
             "El Río Orinoco es uno de los ríos más largos y biodiversos de Sudamérica."),
            ("callejon_zona_15", "Guia Eli", 2,
             "Hogar de la tonina rosada y el caimán del Orinoco. ¡Una joya natural única!"),
                         ("callejon_zona_15", "Guia Eli", 3,
             "Regresa a los pasillos para explorar otras zonas."),

            ("callejon_zona_20", "Guia Eli", 1,
             "El Cerro Yapacana es uno de los tepuyes más remotos del Amazonas venezolano."),
            ("callejon_zona_20", "Guia Eli", 2,
             "Alberga especies únicas que evolucionaron en aislamiento durante millones de años."),
            ("callejon_zona_20", "Guia Eli", 3,
             "Regresa a los pasillos para explorar otras zonas."),

            ("callejon_zona_25", "Guia Eli", 1,
             "¡Has llegado al final de esta sección! Increíble biodiversidad, ¿verdad?"),
            ("callejon_zona_25", "Guia Eli", 2,
             "Regresa a los pasillos para seguir explorando otros ecosistemas del parque."),
        ];

        for (contexto, personaje, orden, texto) in dialogos {
            self.conn.execute(
                "INSERT INTO dialogos (contexto, personaje, orden, texto)
                 VALUES (?1, ?2, ?3, ?4)",
                params![contexto, personaje, orden, texto],
            ).unwrap();
        }
    }

    fn poblar_quiz_museo(&self) {
        let quiz: &[(&str, &str, &str, &str, &str, usize)] = &[
            ("¿En qué era vivió el Tigre Dientes de Sable?",
             "Jurásico", "Pleistoceno", "Triásico", "Cretácico", 1),
            ("¿Cuánto podía medir la Pereza Gigante parada?",
             "2 metros", "4 metros", "6 metros", "8 metros", 2),
            ("¿Cuál es el nombre científico del Gliptodonte venezolano?",
             "Glyptodon clavipes", "Glyptotherium venezuelensis",
             "Doedicurus clavicaudatus", "Panochthus tuberculatus", 1),
            ("¿Dónde se hallaron restos del Mastodonte en Venezuela?",
             "Taima-Taima, Falcón", "El Breal de Orocual, Monagas",
             "Cueva del Guácharo, Monagas", "Valle de Caracas", 1),
            ("¿A qué animal actual está emparentado el Mastodonte?",
             "Rinoceronte", "Hipopótamo", "Elefante", "Tapir", 2),
            ("¿Cuántos cm medían los colmillos del Tigre Dientes de Sable?",
             "10 cm", "20 cm", "30 cm", "50 cm", 2),
            ("¿Cuándo se extinguió la megafauna del Pleistoceno?",
             "Hace 65 millones de años", "Hace 10 millones de años",
             "Hace 10000 años aprox.", "Hace 1 millón de años", 2),
            ("¿Qué tan grande era el Gliptodonte?",
             "Como un perro", "Como un oso",
             "Como un auto pequeño", "Como un elefante", 2),
            ("¿La Pereza Gigante era...?",
             "Carnívora", "Herbívora", "Omnívora", "Insectívora", 1),
            ("¿En qué yacimiento venezolano se hallaron restos de la megafauna?",
             "El Ávila", "Taima-Taima", "Los Roques", "Canaima", 1),
        ];

        for (preg, a, b, c, d, correcta) in quiz {
            self.conn.execute(
                "INSERT INTO quiz_museo
                 (pregunta, opcion_a, opcion_b, opcion_c, opcion_d, correcta)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![preg, a, b, c, d, correcta],
            ).unwrap();
        }
    }

    // ── Queries Públicas ─────────────────────────────────────────────

    pub fn animales_zona(&self, escena: &Escena) -> Vec<Animal> {
        let mut stmt = self.conn.prepare(
            "SELECT id, zona_id, nombre_comun, nombre_cientifico, descripcion, categoria
             FROM animales WHERE zona_id = ?1 ORDER BY id"
        ).unwrap();
        stmt.query_map(params![escena.db_id()], |row| {
            Ok(Animal {
                id: row.get(0)?,
                zona_id: row.get(1)?,
                nombre_comun: row.get(2)?,
                nombre_cientifico: row.get(3)?,
                descripcion: row.get(4)?,
                categoria: row.get(5)?,
            })
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    pub fn animales_por_categoria(&self, categoria: &str) -> Vec<Animal> {
        let mut stmt = self.conn.prepare(
            "SELECT id, zona_id, nombre_comun, nombre_cientifico, descripcion, categoria
             FROM animales WHERE categoria = ?1 ORDER BY id"
        ).unwrap();
        stmt.query_map(params![categoria], |row| {
            Ok(Animal {
                id: row.get(0)?,
                zona_id: row.get(1)?,
                nombre_comun: row.get(2)?,
                nombre_cientifico: row.get(3)?,
                descripcion: row.get(4)?,
                categoria: row.get(5)?,
            })
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    pub fn dialogos_por_contexto(&self, contexto: &str) -> Vec<DialogoDB> {
        let mut stmt = self.conn.prepare(
            "SELECT id, contexto, personaje, orden, texto
             FROM dialogos WHERE contexto = ?1 ORDER BY orden"
        ).unwrap();
        stmt.query_map(params![contexto], |row| {
            Ok(DialogoDB {
                id: row.get(0)?,
                contexto: row.get(1)?,
                personaje: row.get(2)?,
                orden: row.get(3)?,
                texto: row.get(4)?,
            })
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    pub fn dialogos_bienvenida(&self, es_tactil: bool) -> Vec<DialogoDB> {
        let plataforma_ctx = if es_tactil {
            "bienvenida_tactil"
        } else {
            "bienvenida_teclado"
        };
        let base = self.dialogos_por_contexto("bienvenida");
        let especificos = self.dialogos_por_contexto(plataforma_ctx);
        let mut resultado = Vec::new();
        for d in &base {
            if d.orden < 4 { resultado.push(d.clone()); }
        }
        for e in &especificos {
            resultado.push(e.clone());
        }
        for d in &base {
            if d.orden > 4 { resultado.push(d.clone()); }
        }
        resultado
    }

    pub fn quiz_museo_preguntas(&self) -> Vec<QuizPreguntaDB> {
        let mut stmt = self.conn.prepare(
            "SELECT id, pregunta, opcion_a, opcion_b, opcion_c, opcion_d, correcta
             FROM quiz_museo ORDER BY RANDOM()"
        ).unwrap();
        stmt.query_map([], |row| {
            Ok(QuizPreguntaDB {
                id: row.get(0)?,
                pregunta: row.get(1)?,
                opcion_a: row.get(2)?,
                opcion_b: row.get(3)?,
                opcion_c: row.get(4)?,
                opcion_d: row.get(5)?,
                correcta: row.get(6)?,
            })
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    pub fn animal_por_nombre(&self, nombre: &str) -> Option<Animal> {
        self.conn.query_row(
            "SELECT id, zona_id, nombre_comun, nombre_cientifico, descripcion, categoria
             FROM animales WHERE nombre_comun = ?1",
            params![nombre],
            |row| Ok(Animal {
                id: row.get(0)?,
                zona_id: row.get(1)?,
                nombre_comun: row.get(2)?,
                nombre_cientifico: row.get(3)?,
                descripcion: row.get(4)?,
                categoria: row.get(5)?,
            }),
        ).ok()
    }
}