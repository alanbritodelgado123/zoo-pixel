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

    // =========================================================
    //  ZONAS
    // =========================================================
    fn poblar_zonas(&self) {
        let zonas: &[(&str, &str)] = &[
            ("entrada", "Entrada Principal"),
            ("p1", "Pasillo 1"),
            ("p2", "Pasillo 2"),
            ("p3", "Pasillo 3"),
            ("p4", "Pasillo 4"),
            ("p5", "Pasillo 5 - Acuario"),
            ("z1_1", "Llanos Centrales I"),
            ("z1_2", "Llanos Centrales II"),
            ("z1_3", "Llanos Centrales III"),
            ("z1_4", "Llanos Centrales IV"),
            ("z1_5", "Llanos Occidentales"),
            ("z2_1", "Cordillera de los Andes I"),
            ("z2_2", "Cordillera de los Andes II"),
            ("z2_3", "Cordillera de la Costa"),
            ("z2_4", "Páramos de Mérida"),
            ("z2_5", "Sierra de San Luis"),
            ("z3_1", "Selva Amazónica I"),
            ("z3_2", "Selva Amazónica II"),
            ("z3_3", "Serranía de la Neblina"),
            ("z3_4", "Selva de Imataca"),
            ("z3_5", "Cerro Yapacana"),
            ("z4_1", "Sierra de Perijá I"),
            ("z4_2", "Parque Nacional Canaima"),
            ("z4_3", "Delta del Orinoco I"),
            ("z4_4", "Delta del Orinoco II"),
            ("z4_5", "Río Orinoco"),
            ("z5_1", "Península de Paria"),
            ("z5_2", "Isla de Margarita"),
            ("z5_3", "Costa Caribe Oriental"),
            ("z5_4", "Los Roques"),
            ("z5_5", "Lago de Maracaibo"),
        ];
        for (id, nombre) in zonas {
            self.conn.execute(
                "INSERT INTO zonas (id, nombre) VALUES (?1, ?2)",
                params![id, nombre],
            ).unwrap();
        }
    }

    // =========================================================
    //  ANIMALES (117 + fósiles como categoría propia)
    // =========================================================
    fn poblar_animales(&self) {
        let animales: &[(&str, &str, &str, &str, &str)] = &[

            // ── ZONA 1: Llanos ──────────────────────────────────────
            ("z1_1","Chigüire","Hydrochoerus hydrochaeris",
             "Roedor más grande del mundo. Vive en grupos de hasta 20 individuos cerca del agua. Puede pesar hasta 65 kg. Es herbívoro y semiacuático.",
             "mamiferos"),
            ("z1_1","Cunaguaro","Leopardus pardalis",
             "Ocelote, cazador nocturno solitario. Pelaje con patrón único como huellas dactilares. Excelente trepador. Pesa hasta 16 kg.",
             "mamiferos"),
            ("z1_1","Garza Real","Ardea alba",
             "Ave acuática de gran tamaño, alcanza hasta 1 metro de altura. Plumaje blanco puro y patas largas negras.",
             "aves"),
            ("z1_1","Babilla","Caiman crocodilus",
             "Caimán pequeño que habita pantanos y ríos lentos de los llanos. Mide hasta 2.5 metros.",
             "reptiles"),

            ("z1_2","Turpial","Icterus icterus",
             "Ave nacional de Venezuela. Plumaje naranja brillante y negro. Canto melodioso y complejo.",
             "aves"),
            ("z1_2","Cardenalito","Spinus cucullatus",
             "Ave pequeña endémica de Venezuela, en peligro crítico de extinción. Macho de plumaje rojo intenso.",
             "aves"),
            ("z1_2","Paraulata Llanera","Mimus gilvus",
             "Excelente imitador de cantos de otras aves. Plumaje gris pardusco. Muy común en sabanas.",
             "aves"),
            ("z1_2","Corocoro Rojo","Eudocimus ruber",
             "Ibis de plumaje rojo escarlata intenso. Color debido a carotenoides en su dieta.",
             "aves"),

            ("z1_3","Pavón Real","Cichla orinocensis",
             "Depredador de agua dulce muy apreciado en pesca deportiva. Puede pesar hasta 12 kg.",
             "peces"),
            ("z1_3","Sapara","Semaprochilodus laticeps",
             "Pez emblemático de la Feria de la Sapara en Bolívar. Migratorio, forma cardúmenes enormes.",
             "peces"),
            ("z1_3","Caribe Colorado","Pygocentrus nattereri",
             "Piraña de vientre rojo. Dientes afilados como navajas. Vive en cardúmenes.",
             "peces"),
            ("z1_3","Raya Motoro","Potamotrygon motoro",
             "Raya de agua dulce con patrón de manchas oceladas. Aguijón venenoso en la cola.",
             "peces"),

            ("z1_4","Iguana Verde","Iguana iguana",
             "Lagarto arborícola herbívoro. Puede medir 2 metros con cola. Tiene un tercer ojo parietal.",
             "reptiles"),
            ("z1_4","Morrocoy Sabanero","Chelonoidis carbonarius",
             "Tortuga terrestre de patas rojas. Caparazón negro con manchas amarillas y rojas.",
             "reptiles"),
            ("z1_4","Rana Platanera","Boana xerophylla",
             "Rana arborícola común en zonas húmedas. Discos adhesivos en dedos para trepar.",
             "anfibios"),
            ("z1_4","Tuqueque","Thecadactylus rapicauda",
             "Gecko nocturno común en viviendas. Cola en forma de nabo que puede desprender.",
             "reptiles"),

            ("z1_5","Oso Palmero","Myrmecophaga tridactyla",
             "Lengua de 60 cm para atrapar hormigas y termitas. Puede comer hasta 35000 insectos al día.",
             "mamiferos"),
            ("z1_5","Galápago Llanero","Podocnemis vogli",
             "Tortuga de agua dulce endémica de los llanos. Caparazón aplanado color oliva.",
             "reptiles"),
            ("z1_5","Zorro Cangrejero","Cerdocyon thous",
             "Cánido omnívoro de sabanas y bosques. Pelaje grisáceo. Come cangrejos y frutas.",
             "mamiferos"),
            ("z1_5","Pavón Negro","Crax alector",
             "Ave grande terrestre de bosques. Plumaje negro con cresta rizada.",
             "aves"),

            // ── ZONA 2: Andes ───────────────────────────────────────
            ("z2_1","Oso Frontino","Tremarctos ornatus",
             "Único oso de Sudamérica. Marcas faciales únicas en cada individuo. Puede pesar hasta 200 kg.",
             "mamiferos"),
            ("z2_1","Cóndor de los Andes","Vultur gryphus",
             "Envergadura de 3.3 metros, una de las aves voladoras más grandes del mundo.",
             "aves"),
            ("z2_1","Paujil de Copete de Piedra","Pauxi pauxi",
             "Ave terrestre de gran tamaño que habita bosques nublados andinos. En peligro crítico.",
             "aves"),
            ("z2_1","Venado Caramerudo","Odocoileus virginianus",
             "Habita bosques y sabanas. Los machos tienen astas ramificadas que renuevan cada año.",
             "mamiferos"),

            ("z2_2","Campanero Blanco","Procnias albus",
             "Posee el canto más fuerte registrado en aves, alcanzando 125 decibelios.",
             "aves"),
            ("z2_2","Gallito de las Rocas Andino","Rupicola peruvianus",
             "Macho de plumaje naranja intenso espectacular. Realiza danzas elaboradas.",
             "aves"),
            ("z2_2","Quetzal Dorado","Pharomachrus fulgidus",
             "Ave de plumaje iridiscente verde dorado y rojo. Cola larga en machos.",
             "aves"),
            ("z2_2","Guácharo","Steatornis caripensis",
             "Única ave frugívora nocturna que usa ecolocalización. Habita cuevas profundas.",
             "aves"),

            ("z2_3","Tigre Mariposa","Panthera onca",
             "Jaguar, denominación local venezolana. Mayor felino de América. Mordida más poderosa.",
             "mamiferos"),
            ("z2_3","Sorocua Acollarado","Trogon collaris",
             "Ave de plumaje verde metálico y pecho rojo. Cola larga con patrón blanco y negro.",
             "aves"),
            ("z2_3","Mono Capuchino","Cebus olivaceus",
             "Usa herramientas como piedras para abrir nueces. Considerado el primate más inteligente.",
             "primates"),
            ("z2_3","Trogon Grande","Trogon massena",
             "Trogon más grande de Venezuela. Plumaje verde oscuro y pecho rojo intenso.",
             "aves"),

            ("z2_4","Pato de Torrente","Merganetta armata",
             "Especializado en ríos de montaña de corriente rápida. Nada contra corrientes fuertes.",
             "aves"),
            ("z2_4","Águila Real de los Andes","Geranoaetus melanoleucus",
             "Rapaz de gran tamaño de zonas montañosas. Envergadura de 2 metros.",
             "aves"),
            ("z2_4","Conejo de Páramo","Sylvilagus varynaensis",
             "Conejo endémico de páramos venezolanos. Pelaje denso adaptado al frío.",
             "mamiferos"),
            ("z2_4","Musaraña de los Andes","Cryptotis meridensis",
             "Pequeño mamífero insectívoro endémico de los Andes venezolanos.",
             "mamiferos"),

            ("z2_5","Armadillo Gigante","Priodontes maximus",
             "Armadillo gigante, el más grande del mundo. Puede pesar hasta 60 kg.",
             "mamiferos"),
            ("z2_5","Danta","Tapirus terrestris",
             "Mayor mamífero terrestre de Sudamérica, hasta 300 kg. Tiene una pequeña trompa prensil.",
             "mamiferos"),
            ("z2_5","Puma","Puma concolor",
             "León americano. Habita desde Canadá hasta la Patagonia. Solitario y territorial.",
             "mamiferos"),
            ("z2_5","Pava Negra","Aburria aburri",
             "Ave grande terrestre de bosques nublados andinos. Plumaje negro brillante.",
             "aves"),

            // ── ZONA 3: Amazonas ────────────────────────────────────
            ("z3_1","Jaguar","Panthera onca",
             "Mayor felino de América. Mordida más poderosa de todos los felinos.",
             "mamiferos"),
            ("z3_1","Guacamaya Bandera","Ara macao",
             "Ave emblemática de los trópicos americanos. Plumaje rojo, amarillo y azul brillante.",
             "aves"),
            ("z3_1","Anaconda Verde","Eunectes murinus",
             "Serpiente más pesada del mundo, hasta 250 kg y 9 metros de longitud.",
             "reptiles"),
            ("z3_1","Mono Araguato","Alouatta seniculus",
             "Mono aullador rojo. Su grito se escucha a 5 km gracias a un hueso hioideo amplificador.",
             "primates"),

            ("z3_2","Mariposa Morpho Azul","Morpho helenor",
             "Mariposa de alas azul metálico iridiscente. Envergadura de hasta 15 cm.",
             "insectos"),
            ("z3_2","Escarabajo Hércules","Dynastes hercules",
             "Uno de los escarabajos más grandes del mundo, hasta 17 cm.",
             "insectos"),
            ("z3_2","Bachaco Culón","Atta laevigata",
             "Hormiga cortadora de hojas. Viven en colonias de millones de individuos.",
             "insectos"),
            ("z3_2","Hormiga Veinticuatro","Paraponera clavata",
             "Posee la picadura más dolorosa del reino animal. El dolor dura 24 horas.",
             "insectos"),

            ("z3_3","Gallo de Roca Guayanés","Rupicola rupicola",
             "Macho de plumaje naranja brillante espectacular. Realiza danzas en arenas comunales.",
             "aves"),
            ("z3_3","Uacarí de Cabeza Negra","Cacajao melanocephalus",
             "Primate de cara negra y pelaje largo. Vive en grupos grandes.",
             "primates"),
            ("z3_3","Saltarín de Cabeza Dorada","Ceratopipra erythrocephala",
             "Ave pequeña, macho con cabeza amarillo dorado brillante.",
             "aves"),
            ("z3_3","Mono Viuda","Cheracebus lugens",
             "Mono pequeño de pelaje denso negro. Vive en parejas monógamas.",
             "primates"),

            ("z3_4","Águila Crestada","Morphnus guianensis",
             "Rapaz poderosa de selvas tropicales. Cresta larga eréctil.",
             "aves"),
            ("z3_4","Mono Maicero","Sapajus apella",
             "Mono capuchino robusto y adaptable. Usa herramientas para abrir nueces.",
             "primates"),
            ("z3_4","Culebra Lora","Leptophis ahaetulla",
             "Serpiente arborícola delgada de color verde brillante. No venenosa.",
             "reptiles"),
            ("z3_4","Harpía","Harpia harpyja",
             "Águila más poderosa del mundo. Envergadura de 2 metros.",
             "aves"),

            ("z3_5","Sapito de Yapacana","Minyobates steyermarki",
             "Rana venenosa endémica del Cerro Yapacana. Piel negra con franjas amarillas.",
             "anfibios"),
            ("z3_5","Halcón de Monte","Micrastur semitorquatus",
             "Rapaz de bosques densos. Alas cortas y cola larga para vuelo en vegetación.",
             "aves"),
            ("z3_5","Oso Melero","Tamandua tetradactyla",
             "Oso hormiguero menor arborícola. Cola prensil. Lengua larga pegajosa.",
             "mamiferos"),
            ("z3_5","Tucancito de Pico Maculado","Selenidera culik",
             "Tucán pequeño de pico multicolor con manchas.",
             "aves"),

            // ── ZONA 4: Perijá, Canaima, Delta ─────────────────────
            ("z4_1","Marimonda de la Sierra","Ateles hybridus",
             "Mono araña café en peligro crítico. Cola prensil que funciona como quinta mano.",
             "primates"),
            ("z4_1","Tucán Real","Ramphastos sulfuratus",
             "Su pico puede medir hasta 20 cm pero es muy ligero por estructura hueca.",
             "aves"),
            ("z4_1","Puma","Puma concolor",
             "León americano, habita desde Canadá hasta la Patagonia.",
             "mamiferos"),
            ("z4_1","Cascabel","Crotalus durissus",
             "Serpiente venenosa con cascabel en la cola. Veneno hemotóxico y neurotóxico.",
             "reptiles"),

            ("z4_2","Sapito Minero","Dendrobates leucomelas",
             "Rana venenosa de piel negra con bandas amarillas brillantes.",
             "anfibios"),
            ("z4_2","Oso Hormiguero Gigante","Myrmecophaga tridactyla",
             "Lengua de 60 cm para atrapar hormigas y termitas.",
             "mamiferos"),
            ("z4_2","Tepuihyla","Tepuihyla rodriguezi",
             "Rana endémica de cimas de tepuyes. Evolucionada en aislamiento.",
             "anfibios"),
            ("z4_2","Guacamaya Azul y Amarilla","Ara ararauna",
             "Guacamaya grande de plumaje azul intenso y pecho amarillo dorado.",
             "aves"),

            ("z4_3","Manatí del Caribe","Trichechus manatus",
             "Mamífero acuático herbívoro que puede pesar hasta 500 kg.",
             "mamiferos"),
            ("z4_3","Perro de Agua","Pteronura brasiliensis",
             "Nutria gigante, la más grande del mundo, hasta 1.8 metros.",
             "mamiferos"),
            ("z4_3","Garzón Soldado","Jabiru mycteria",
             "Cigüeña enorme, la mayor ave voladora de América.",
             "aves"),
            ("z4_3","Mono Capuchino del Orinoco","Cebus albifrons",
             "Mono capuchino de frente blanca. Inteligente y adaptable.",
             "primates"),

            ("z4_4","Pava de Monte","Penelope jacquacu",
             "Ave grande terrestre de selvas inundables. Plumaje café oscuro.",
             "aves"),
            ("z4_4","Tortuga Arrau","Podocnemis expansa",
             "Mayor tortuga de río de Sudamérica, caparazón de hasta 90 cm.",
             "reptiles"),
            ("z4_4","Caimán Enano","Paleosuchus palpebrosus",
             "Caimán más pequeño, hasta 1.5 metros. Piel muy osificada.",
             "reptiles"),
            ("z4_4","Mapanare","Bothrops asper",
             "Serpiente venenosa muy peligrosa. Responsable de mayoría de mordeduras.",
             "reptiles"),

            ("z4_5","Tonina","Inia geoffrensis",
             "Delfín rosado de agua dulce, el más grande del mundo.",
             "mamiferos"),
            ("z4_5","Caimán del Orinoco","Crocodylus intermedius",
             "En peligro crítico, quedan menos de 1500 individuos.",
             "reptiles"),
            ("z4_5","Lau-lau","Brachyplatystoma filamentosum",
             "Bagre gigante de agua dulce, puede pesar hasta 200 kg.",
             "peces"),
            ("z4_5","Payara","Hydrolycus scomberoides",
             "Pez depredador con colmillos enormes que sobresalen de la mandíbula.",
             "peces"),

            // ── ZONA 5: Costas, Islas, Maracaibo ───────────────────
            ("z5_1","Carnotauro","Carnotaurus sastrei",
             "Dinosaurio terópodo del Cretácico Superior. Tenía cuernos sobre los ojos y brazos extremadamente reducidos.",
             "fosiles"),
            ("z5_1","Pteranodon","Pteranodon longiceps",
             "Reptil volador del Cretácico con envergadura de hasta 7 metros. No tenía dientes.",
             "fosiles"),
            ("z5_1","Ammonite","Ammonoidea",
             "Molusco marino extinto emparentado con el nautilus actual. Vivió desde el Devónico hasta el Cretácico.",
             "fosiles"),
            ("z5_1","Trilobite","Trilobita",
             "Artrópodo marino que vivió durante 300 millones de años. Tenía ojos de cristal de calcita.",
             "fosiles"),
            ("z5_1","Megalodon","Otodus megalodon",
             "El tiburón más grande que existió, hasta 18 metros de largo. Vivió en el Mioceno y Plioceno.",
             "fosiles"),

            ("z5_2","Venado de Margarita","Odocoileus virginianus margaritae",
             "Subespecie endémica de Isla de Margarita. Más pequeño que venados continentales.",
             "mamiferos"),
            ("z5_2","Cotorra Margariteña","Amazona barbadensis",
             "Loro endémico de zonas áridas. Plumaje verde con frente amarilla.",
             "aves"),
            ("z5_2","Ñangaro","Psittacara wagleri",
             "Perico de frente roja. Ruidoso y gregario.",
             "aves"),
            ("z5_2","Cunaguaro de Margarita","Leopardus pardalis pardalis",
             "Subespecie de ocelote de Isla de Margarita.",
             "mamiferos"),

            ("z5_3","Flamenco del Caribe","Phoenicopterus ruber",
             "Flamenco de color rosado intenso debido a carotenoides en su dieta.",
             "aves"),
            ("z5_3","Tortuga Carey","Eretmochelys imbricata",
             "Tortuga marina en peligro crítico. Caparazón con escudos superpuestos.",
             "reptiles"),
            ("z5_3","Fragata","Fregata magnificens",
             "Ave marina con envergadura de 2.3 metros. Macho tiene saco gular rojo.",
             "aves"),
            ("z5_3","Caimán de la Costa","Crocodylus acutus",
             "Cocodrilo americano, habita costas y estuarios.",
             "reptiles"),

            ("z5_4","Pez Loro","Scarus guacamaia",
             "Pez loro más grande del Atlántico, hasta 1.2 metros.",
             "peces"),
            ("z5_4","Langosta Espinosa","Panulirus argus",
             "Langosta sin pinzas con antenas largas. Importante especie comercial.",
             "insectos"),
            ("z5_4","Botuto","Lobatus gigas",
             "Caracol marino gigante, hasta 30 cm. Concha rosada por dentro.",
             "insectos"),
            ("z5_4","Pez Ángel Francés","Pomacanthus paru",
             "Pez de arrecife de color negro con bordes amarillos.",
             "peces"),

            ("z5_5","Pato Cuchara","Anas clypeata",
             "Pato con pico en forma de cuchara para filtrar alimento.",
             "aves"),
            ("z5_5","Chavarri","Chauna chavaria",
             "Ave grande semiacuática, hasta 90 cm. Grito estridente.",
             "aves"),
            ("z5_5","Pavón de Maracaibo","Cichla temensis",
             "Pez depredador más grande del género, hasta 15 kg.",
             "peces"),
            ("z5_5","Bagre de Maracaibo","Perrunichthys perruno",
             "Bagre de patrón de manchas tipo leopardo.",
             "peces"),

            // ── PECES ADICIONALES (para minijuego pesca en P5) ──────
            ("p5","Curbinata","Plagioscion squamosissimus",
             "Pez de río muy apreciado en la gastronomía venezolana. Carne blanca y firme.",
             "peces"),
            ("p5","Coporo","Prochilodus mariae",
             "Pez herbívoro migratorio del Orinoco. Forma cardúmenes enormes en época de subienda.",
             "peces"),
            ("p5","Valentón","Brachyplatystoma rousseauxii",
             "Uno de los bagres más grandes del Orinoco. Realiza migraciones de miles de kilómetros.",
             "peces"),
            ("p5","Morocoto","Colossoma macropomum",
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

    // =========================================================
    //  DIÁLOGOS
    // =========================================================
    fn poblar_dialogos(&self) {
        let dialogos: &[(&str, &str, i32, &str)] = &[
            // ── Bienvenida PC ───────────────────────────────────────
            ("bienvenida", "Guía Eli", 1,
             "¡Bienvenido al Zoológico Nacional de Venezuela! Soy Eli, tu guía personal."),
            ("bienvenida", "Guía Eli", 2,
             "Aquí conocerás la increíble fauna de Venezuela, desde los llanos hasta las montañas andinas."),
            ("bienvenida", "Guía Eli", 3,
             "Usa las flechas para moverte entre las zonas. Cada una representa un ecosistema diferente."),
            ("bienvenida_teclado", "Guía Eli", 4,
             "Presiona Z para explorar y ver los animales. Con X puedes volver atrás."),
            ("bienvenida_tactil", "Guía Eli", 4,
             "Toca el botón A para explorar y ver los animales. Con B puedes volver atrás."),
            ("bienvenida", "Guía Eli", 5,
             "Tienes una libreta de campo donde se guardan automáticamente los animales que descubras."),
            ("bienvenida", "Guía Eli", 6,
             "El mapa te ayudará a orientarte. Puedes abrirlo presionando M en cualquier momento."),
            ("bienvenida", "Guía Eli", 7,
             "¡Venezuela tiene una biodiversidad increíble: más de 1400 especies de aves!"),
            ("bienvenida", "Guía Eli", 8,
             "¡Explora las 25 zonas del parque! Buena suerte en tu expedición."),

            // ── Museo (Guía Ani) ────────────────────────────────────
            ("museo_bienvenida", "Guía Ani", 1,
             "¡Hola! Soy Ani, tu guía en el Museo Paleontológico de Paria."),
            ("museo_bienvenida", "Guía Ani", 2,
             "Aquí podrás explorar exhibiciones de fósiles venezolanos y del mundo prehistórico."),
            ("museo_bienvenida", "Guía Ani", 3,
             "También puedes excavar para encontrar fósiles escondidos en la roca."),
            ("museo_bienvenida", "Guía Ani", 4,
             "¡Y pon a prueba tu conocimiento en el quiz paleontológico! Usa Z para seleccionar."),

            // ── Callejones Zx-5 ─────────────────────────────────────
            ("callejon_z1_5", "Guía Eli", 1,
             "Has llegado al extremo de los Llanos Occidentales."),
            ("callejon_z1_5", "Guía Eli", 2,
             "Regresa al pasillo para explorar otros ecosistemas."),

            ("callejon_z2_5", "Guía Eli", 1,
             "¡Estás en la Sierra de San Luis, en los Andes venezolanos!"),
            ("callejon_z2_5", "Guía Eli", 2,
             "Desde aquí puedes volver para continuar tu recorrido."),

            ("callejon_z3_5", "Guía Eli", 1,
             "El Cerro Yapacana es uno de los tepuyes más remotos del Amazonas."),
            ("callejon_z3_5", "Guía Eli", 2,
             "Alberga especies únicas que no existen en ningún otro lugar del planeta."),

            ("callejon_z4_5", "Guía Eli", 1,
             "El Río Orinoco es uno de los más largos de Sudamérica."),
            ("callejon_z4_5", "Guía Eli", 2,
             "Hogar de la tonina rosada y el caimán del Orinoco. ¡Una joya natural!"),

            ("callejon_z5_5", "Guía Eli", 1,
             "El Lago de Maracaibo es el lago natural más grande de América Latina."),
            ("callejon_z5_5", "Guía Eli", 2,
             "Sus aguas albergan especies de peces únicas. ¡Qué lugar tan especial!"),
        ];

        for (contexto, personaje, orden, texto) in dialogos {
            self.conn.execute(
                "INSERT INTO dialogos (contexto, personaje, orden, texto)
                 VALUES (?1, ?2, ?3, ?4)",
                params![contexto, personaje, orden, texto],
            ).unwrap();
        }
    }

    // =========================================================
    //  QUIZ MUSEO (desde DB, sin hardcodeo en minijuego.rs)
    // =========================================================
    fn poblar_quiz_museo(&self) {
        let quiz: &[(&str, &str, &str, &str, &str, usize)] = &[
            ("¿En qué era vivió el Carnotauro?",
             "Jurásico", "Cretácico Superior", "Triásico", "Pérmico", 1),
            ("¿Qué característica tenía el Pteranodon?",
             "Cuernos sobre los ojos", "Cola larga", "No tenía dientes", "Plumas de colores", 2),
            ("¿Cuánto podía medir un Megalodon?",
             "5 metros", "10 metros", "18 metros", "25 metros", 2),
            ("¿Los ammonites estaban emparentados con...?",
             "Estrellas de mar", "Nautilus actual", "Corales", "Medusas", 1),
            ("¿De qué estaban hechos los ojos de los trilobites?",
             "Queratina", "Cristal de calcita", "Cartílago", "Sílice", 1),
            ("¿Cuántos millones de años vivieron los trilobites?",
             "50 millones", "100 millones", "200 millones", "300 millones", 3),
            ("¿Dónde vivía el Megalodon?",
             "Ríos tropicales", "Lagos profundos", "Océanos de todo el mundo", "Mares árticos", 2),
            ("¿Qué comía el Pteranodon?",
             "Plantas acuáticas", "Peces", "Insectos gigantes", "Otros reptiles", 1),
            ("¿Cuántos cuernos tenía el Carnotauro?",
             "Ninguno", "Uno", "Dos", "Cuatro", 2),
            ("¿En qué período vivieron los ammonites?",
             "Solo en el Jurásico", "Solo en el Cretácico",
             "Del Devónico al Cretácico", "Del Triásico al Terciario", 2),
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

    // =========================================================
    //  QUERIES PÚBLICAS
    // =========================================================
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
            if d.orden < 4 {
                resultado.push(d.clone());
            }
        }
        for e in &especificos {
            resultado.push(e.clone());
        }
        for d in &base {
            if d.orden > 4 {
                resultado.push(d.clone());
            }
        }
        resultado
    }

    /// Quiz del museo en orden aleatorio, limitado a `limite` preguntas
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