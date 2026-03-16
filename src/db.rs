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
                FOREIGN KEY (zona_id) REFERENCES zonas(id)
            );
        ").expect("Error creando tablas");

        let db = Self { conn };
        db.poblar();
        db
    }

    fn poblar(&self) {
        let zonas = [
            ("entrada",     "Entrada"),
            ("sabana",      "Sabana"),
            ("laguna",      "Laguna"),
            ("aviario",     "Aviario"),
            ("felinos",     "Felinos"),
            ("reptiliario", "Reptiliario"),
            ("primates",    "Primates"),
            ("montana",     "Montaña"),
            ("humedal",     "Humedal"),
            ("nocturario",  "Nocturario"),
        ];

        for (id, nombre) in &zonas {
            self.conn.execute(
                "INSERT INTO zonas (id, nombre) VALUES (?1, ?2)",
                params![id, nombre],
            ).unwrap();
        }

        let animales: &[(&str, &str, &str, &str)] = &[
            // Sabana
            ("sabana", "Chigüire", "Hydrochoerus hydrochaeris",
             "Roedor más grande del mundo. Vive en grupos de hasta 20 individuos cerca del agua. Puede pesar hasta 65kg. Es herbívoro y semiacuático."),
            ("sabana", "Venado Caramerudo", "Odocoileus virginianus",
             "Habita bosques y sabanas de Venezuela. Los machos tienen astas ramificadas que renuevan cada año. Muy ágil, puede saltar hasta 3 metros de alto."),
            ("sabana", "Oso Palmero", "Myrmecophaga tridactyla",
             "Lengua de 60cm para atrapar hormigas y termitas. Puede comer hasta 35.000 insectos al día. No tiene dientes. Sus garras miden 10cm."),
            ("sabana", "Báquiro", "Pecari tajacu",
             "Jabalí sudamericano que vive en manadas de 5 a 15 individuos. Tiene una glándula en el lomo que produce un olor fuerte. Pesa hasta 30kg."),

            // Laguna
            ("laguna", "Tonina", "Inia geoffrensis",
             "Delfín rosado de agua dulce, el más grande del mundo. Puede medir hasta 2.5 metros. Su color rosado se intensifica con la edad. Usa ecolocalización para navegar en aguas turbias."),
            ("laguna", "Manatí", "Trichechus manatus",
             "Herbívoro acuático que puede pesar hasta 500kg. Come hasta 50kg de plantas al día. Puede vivir 60 años. Nada a 8km/h y necesita respirar cada 20 minutos."),
            ("laguna", "Danta", "Tapirus terrestris",
             "Mayor mamífero terrestre de Sudamérica, hasta 300kg. Tiene una pequeña trompa prénsil. Excelente nadadora. Es solitaria y nocturna."),
            ("laguna", "Raya de Río", "Potamotrygon motoro",
             "Se camufla en el fondo arenoso de los ríos. Su cola tiene un aguijón venenoso. El disco puede medir 50cm. Es ovovivípara."),

            // Aviario
            ("aviario", "Guacamaya Bandera", "Ara macao",
             "Ave emblemática de los trópicos americanos. Plumaje rojo, amarillo y azul. Puede vivir hasta 75 años. Pareja de por vida. Muy inteligente."),
            ("aviario", "Tucán Piapoco", "Ramphastos tucanus",
             "Su pico puede medir hasta 20cm pero es muy ligero. Anida en huecos de árboles. Se alimenta de frutas, insectos y pequeños reptiles."),
            ("aviario", "Flamenco del Caribe", "Phoenicopterus ruber",
             "Color rosado por los carotenoides en su dieta de crustáceos. Filtra el alimento con su pico curvo. Vive en colonias de miles. Duerme sobre una pata."),
            ("aviario", "Gallito de las Rocas", "Rupicola rupicola",
             "Macho de plumaje naranja intenso. Realiza danzas elaboradas para cortejar. Anida en cuevas y paredes rocosas. Ave nacional de Perú."),
            ("aviario", "Colibrí Coliazul", "Amazilia lactea",
             "Aletea hasta 80 veces por segundo. Puede volar hacia atrás y mantenerse suspendido. Consume néctar de hasta 2000 flores al día. Pesa solo 5 gramos."),

            // Felinos
            ("felinos", "Jaguar", "Panthera onca",
             "Mayor felino de América. Mordida más poderosa de todos los felinos: puede perforar caparazones de tortuga. Excelente nadador. Cazador solitario. Puede pesar hasta 150kg."),
            ("felinos", "Puma", "Puma concolor",
             "León americano, habita desde Canadá hasta Patagonia. Puede saltar 5 metros de alto. Solitario y territorial. No puede rugir, ronronea como gato doméstico."),
            ("felinos", "Cunaguaro", "Leopardus pardalis",
             "Ocelote, cazador nocturno solitario. Pelaje con patrón único como huellas dactilares. Excelente trepador. Pesa hasta 16kg. En peligro por pérdida de hábitat."),
            ("felinos", "Onza", "Herpailurus yagouaroundi",
             "Felino más pequeño de Venezuela, hasta 9kg. Color uniforme marrón o gris. Activo de día, raro entre felinos. Muy ágil y veloz en distancias cortas."),

            // Reptiliario
            ("reptiliario", "Anaconda Verde", "Eunectes murinus",
             "Serpiente más pesada del mundo, hasta 250kg y 9 metros. Constrictora: asfixia a sus presas. Semiacuática. Puede pasar meses sin comer tras una presa grande."),
            ("reptiliario", "Iguana Verde", "Iguana iguana",
             "Herbívora de sangre fría, puede medir 2 metros con cola. Tiene un tercer ojo parietal para detectar depredadores aéreos. Puede caer de 15 metros sin dañarse."),
            ("reptiliario", "Baba", "Caiman crocodilus",
             "Caimán común venezolano. Habita pantanos, lagunas y ríos lentos. Puede medir hasta 2.5 metros. Regula la temperatura tomando sol con la boca abierta."),
            ("reptiliario", "Tortuga Arrau", "Podocnemis expansa",
             "Mayor tortuga de río de Sudamérica, caparazón de hasta 90cm. Puede pesar 60kg. En peligro por recolección de huevos. Anida en playas fluviales."),

            // Primates
            ("primates", "Araguato", "Alouatta seniculus",
             "Mono aullador rojo. Su grito se escucha a 5km gracias a un hueso hioideo amplificador. Vive en grupos de 3 a 9. Pasa el 80% del día descansando."),
            ("primates", "Mono Araña", "Ateles belzebuth",
             "Cola prénsil que funciona como quinta mano, más fuerte que sus brazos. Puede balancearse a 15km/h entre árboles. Vive en grupos de hasta 30. En peligro crítico."),
            ("primates", "Capuchino", "Cebus olivaceus",
             "Usa herramientas como piedras para abrir nueces. Considerado el primate más inteligente de América. Vive en grupos de 8 a 15. Puede vivir 50 años en cautiverio."),

            // Montaña
            ("montana", "Oso Frontino", "Tremarctos ornatus",
             "Único oso de Sudamérica. Marcas faciales únicas en cada individuo. Puede pesar hasta 200kg. Excelente trepador de árboles. En peligro de extinción. Quedan menos de 10.000."),
            ("montana", "Cóndor Andino", "Vultur gryphus",
             "Envergadura de 3.3 metros, una de las aves voladoras más grandes. Puede planear durante horas sin aletear. Vive a más de 3000 metros de altitud. Puede vivir 75 años."),

            // Humedal
            ("humedal", "Caimán del Orinoco", "Crocodylus intermedius",
             "En peligro crítico, quedan menos de 1500. Puede medir hasta 5 metros. Endémico de la cuenca del Orinoco. Fue cazado casi hasta la extinción por su piel."),
            ("humedal", "Pereza", "Bradypus variegatus",
             "Duerme hasta 20 horas al día. Baja de los árboles solo una vez por semana. Algas crecen en su pelaje como camuflaje. Puede girar la cabeza 270 grados."),
            ("humedal", "Cachicamo", "Dasypus novemcinctus",
             "Armadillo de 9 bandas. Se enrolla en bola para defenderse. Siempre tiene cuatrillizos idénticos. Puede contener la respiración 6 minutos bajo agua."),

            // Nocturario
            ("nocturario", "Cunaguaro Nocturno", "Leopardus pardalis",
             "De noche sus pupilas se dilatan al máximo para captar luz. Caza ratones, ranas y aves dormidas. Patrón de manchas único en cada individuo. Camina hasta 10km por noche."),
            ("nocturario", "Murciélago Vampiro", "Desmodus rotundus",
             "Se alimenta exclusivamente de sangre. Su saliva tiene anticoagulante usado en medicina. Puede detectar calor corporal con sensores en su nariz. Vuela solo de noche."),
            ("nocturario", "Víbora de Pestañas", "Bothriechis schlegelii",
             "Escamas modificadas sobre los ojos parecen pestañas. Viene en colores verde, amarillo o rojo. Arbórícola, caza al acecho. Fosetas loreales detectan calor de presas."),
        ];

        for (zona, comun, cientifico, desc) in animales {
            self.conn.execute(
                "INSERT INTO animales (zona_id, nombre_comun, nombre_cientifico, descripcion)
                 VALUES (?1, ?2, ?3, ?4)",
                params![zona, comun, cientifico, desc],
            ).unwrap();
        }
    }

    pub fn animales_zona(&self, escena: &Escena) -> Vec<Animal> {
        let mut stmt = self.conn.prepare(
            "SELECT a.id, a.zona_id, a.nombre_comun, a.nombre_cientifico, a.descripcion
             FROM animales a
             JOIN zonas z ON a.zona_id = z.id
             WHERE a.zona_id = ?1
             ORDER BY a.id"
        ).unwrap();

        stmt.query_map(params![escena.db_id()], |row| {
            Ok(Animal {
                id: row.get(0)?,
                zona_id: row.get(1)?,
                nombre_comun: row.get(2)?,
                nombre_cientifico: row.get(3)?,
                descripcion: row.get(4)?,
            })
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    #[allow(dead_code)]
    pub fn animal_por_id(&self, id: i64) -> Option<Animal> {
        self.conn.query_row(
            "SELECT id, zona_id, nombre_comun, nombre_cientifico, descripcion
             FROM animales WHERE id = ?1",
            params![id],
            |row| Ok(Animal {
                id: row.get(0)?,
                zona_id: row.get(1)?,
                nombre_comun: row.get(2)?,
                nombre_cientifico: row.get(3)?,
                descripcion: row.get(4)?,
            }),
        ).ok()
    }

    #[allow(dead_code)]
    pub fn total_animales(&self) -> usize {
        self.conn.query_row("SELECT COUNT(*) FROM animales", [], |row| row.get::<_, i64>(0))
            .unwrap_or(0) as usize
    }

    #[allow(dead_code)]
    pub fn animales_por_zona(&self) -> Vec<(String, i64)> {
        let mut stmt = self.conn.prepare(
            "SELECT z.nombre, COUNT(a.id) FROM zonas z
             LEFT JOIN animales a ON z.id = a.zona_id
             GROUP BY z.id ORDER BY z.nombre"
        ).unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }
}