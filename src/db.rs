// src/db.rs
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

#[derive(Debug, Clone)]
pub struct DialogoDB {
    pub id: i64,
    pub contexto: String,
    pub personaje: String,
    pub orden: i32,
    pub texto: String,
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
            CREATE TABLE dialogos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                contexto TEXT NOT NULL,
                personaje TEXT NOT NULL,
                orden INTEGER NOT NULL,
                texto TEXT NOT NULL
            );
        ").expect("Error creando tablas");

        let db = Self { conn };
        db.poblar();
        db.poblar_dialogos();
        db
    }

    fn poblar(&self) {
        let zonas: &[(&str, &str)] = &[
            ("entrada_principal",  "Entrada Principal"),
            ("entrada_secundaria", "Entrada Secundaria"),
            ("area_central",       "Área Central"),
            ("a1", "Zona A1 - Sabana"),
            ("a2", "Zona A2 - Felinos"),
            ("a3", "Zona A3 - Reptiliario"),
            ("b1", "Zona B1 - Primates"),
            ("b2", "Zona B2 - Montaña"),
            ("b3", "Zona B3 - Humedal"),
            ("b4", "Zona B4 - Laguna"),
            ("b5", "Zona B5 - Nocturario"),
            ("c1", "Zona C1"),
            ("c2", "Zona C2"),
            ("c3", "Zona C3"),
            ("acuario", "Acuario"),
            ("museo",   "Museo Paleontológico"),
            ("aviario",  "Aviario"),
            ("d1", "Zona D1"),
            ("d2", "Zona D2"),
            ("d3", "Zona D3"),
            ("d4", "Zona D4"),
        ];

        for (id, nombre) in zonas {
            self.conn.execute(
                "INSERT INTO zonas (id, nombre) VALUES (?1, ?2)",
                params![id, nombre],
            ).unwrap();
        }

        let animales: &[(&str, &str, &str, &str)] = &[
            // A1 — Sabana
            ("a1", "Chigüire", "Hydrochoerus hydrochaeris",
             "Roedor más grande del mundo. Vive en grupos de hasta 20 individuos cerca del agua. Puede pesar hasta 65 kg. Es herbívoro y semiacuático."),
            ("a1", "Venado Caramerudo", "Odocoileus virginianus",
             "Habita bosques y sabanas de Venezuela. Los machos tienen astas ramificadas que renuevan cada año. Muy ágil, puede saltar hasta 3 metros de alto."),
            ("a1", "Oso Palmero", "Myrmecophaga tridactyla",
             "Lengua de 60 cm para atrapar hormigas y termitas. Puede comer hasta 35.000 insectos al día. No tiene dientes. Sus garras miden 10 cm."),
            ("a1", "Báquiro", "Pecari tajacu",
             "Jabalí sudamericano que vive en manadas de 5 a 15 individuos. Tiene una glándula en el lomo que produce un olor fuerte. Pesa hasta 30 kg."),

            // B4 — Laguna
            ("b4", "Tonina", "Inia geoffrensis",
             "Delfín rosado de agua dulce, el más grande del mundo. Puede medir hasta 2,5 metros. Su color rosado se intensifica con la edad. Usa ecolocalización para navegar en aguas turbias."),
            ("b4", "Manatí", "Trichechus manatus",
             "Herbívoro acuático que puede pesar hasta 500 kg. Come hasta 50 kg de plantas al día. Puede vivir 60 años. Nada a 8 km/h y necesita respirar cada 20 minutos."),
            ("b4", "Danta", "Tapirus terrestris",
             "Mayor mamífero terrestre de Sudamérica, hasta 300 kg. Tiene una pequeña trompa prénsil. Excelente nadadora. Es solitaria y nocturna."),
            ("b4", "Raya de Río", "Potamotrygon motoro",
             "Se camufla en el fondo arenoso de los ríos. Su cola tiene un aguijón venenoso. El disco puede medir 50 cm. Es ovovivípara."),

            // Aviario
            ("aviario", "Guacamaya Bandera", "Ara macao",
             "Ave emblemática de los trópicos americanos. Plumaje rojo, amarillo y azul. Puede vivir hasta 75 años. Pareja de por vida. Muy inteligente."),
            ("aviario", "Tucán Piapoco", "Ramphastos tucanus",
             "Su pico puede medir hasta 20 cm pero es muy ligero. Anida en huecos de árboles. Se alimenta de frutas, insectos y pequeños reptiles."),
            ("aviario", "Flamenco del Caribe", "Phoenicopterus ruber",
             "Color rosado por los carotenoides en su dieta de crustáceos. Filtra el alimento con su pico curvo. Vive en colonias de miles. Duerme sobre una pata."),
            ("aviario", "Gallito de las Rocas", "Rupicola rupicola",
             "Macho de plumaje naranja intenso. Realiza danzas elaboradas para cortejar. Anida en cuevas y paredes rocosas. Ave nacional del Perú."),
            ("aviario", "Colibrí Coliazul", "Amazilia lactea",
             "Aletea hasta 80 veces por segundo. Puede volar hacia atrás y mantenerse suspendido. Consume néctar de hasta 2000 flores al día. Pesa solo 5 gramos."),

            // A2 — Felinos
            ("a2", "Jaguar", "Panthera onca",
             "Mayor felino de América. Mordida más poderosa de todos los felinos: puede perforar caparazones de tortuga. Excelente nadador. Cazador solitario. Puede pesar hasta 150 kg."),
            ("a2", "Puma", "Puma concolor",
             "León americano, habita desde Canadá hasta la Patagonia. Puede saltar 5 metros de alto. Solitario y territorial. No puede rugir, ronronea como gato doméstico."),
            ("a2", "Cunaguaro", "Leopardus pardalis",
             "Ocelote, cazador nocturno solitario. Pelaje con patrón único como huellas dactilares. Excelente trepador. Pesa hasta 16 kg. En peligro por pérdida de hábitat."),
            ("a2", "Onza", "Herpailurus yagouaroundi",
             "Felino más pequeño de Venezuela, hasta 9 kg. Color uniforme marrón o gris. Activo de día, raro entre felinos. Muy ágil y veloz en distancias cortas."),

            // A3 — Reptiliario
            ("a3", "Anaconda Verde", "Eunectes murinus",
             "Serpiente más pesada del mundo, hasta 250 kg y 9 metros. Constrictora: asfixia a sus presas. Semiacuática. Puede pasar meses sin comer tras una presa grande."),
            ("a3", "Iguana Verde", "Iguana iguana",
             "Herbívora de sangre fría, puede medir 2 metros con cola. Tiene un tercer ojo parietal para detectar depredadores aéreos. Puede caer de 15 metros sin dañarse."),
            ("a3", "Baba", "Caiman crocodilus",
             "Caimán común venezolano. Habita pantanos, lagunas y ríos lentos. Puede medir hasta 2,5 metros. Regula la temperatura tomando sol con la boca abierta."),
            ("a3", "Tortuga Arrau", "Podocnemis expansa",
             "Mayor tortuga de río de Sudamérica, caparazón de hasta 90 cm. Puede pesar 60 kg. En peligro por recolección de huevos. Anida en playas fluviales."),

            // B1 — Primates
            ("b1", "Araguato", "Alouatta seniculus",
             "Mono aullador rojo. Su grito se escucha a 5 km gracias a un hueso hioideo amplificador. Vive en grupos de 3 a 9. Pasa el 80% del día descansando."),
            ("b1", "Mono Araña", "Ateles belzebuth",
             "Cola prénsil que funciona como quinta mano, más fuerte que sus brazos. Puede balancearse a 15 km/h entre árboles. Vive en grupos de hasta 30. En peligro crítico."),
            ("b1", "Capuchino", "Cebus olivaceus",
             "Usa herramientas como piedras para abrir nueces. Considerado el primate más inteligente de América. Vive en grupos de 8 a 15. Puede vivir 50 años en cautiverio."),

            // B2 — Montaña
            ("b2", "Oso Frontino", "Tremarctos ornatus",
             "Único oso de Sudamérica. Marcas faciales únicas en cada individuo. Puede pesar hasta 200 kg. Excelente trepador de árboles. En peligro de extinción. Quedan menos de 10.000."),
            ("b2", "Cóndor Andino", "Vultur gryphus",
             "Envergadura de 3,3 metros, una de las aves voladoras más grandes. Puede planear durante horas sin aletear. Vive a más de 3000 metros de altitud. Puede vivir 75 años."),

            // B3 — Humedal
            ("b3", "Caimán del Orinoco", "Crocodylus intermedius",
             "En peligro crítico, quedan menos de 1500. Puede medir hasta 5 metros. Endémico de la cuenca del Orinoco. Fue cazado casi hasta la extinción por su piel."),
            ("b3", "Pereza", "Bradypus variegatus",
             "Duerme hasta 20 horas al día. Baja de los árboles solo una vez por semana. Algas crecen en su pelaje como camuflaje. Puede girar la cabeza 270 grados."),
            ("b3", "Cachicamo", "Dasypus novemcinctus",
             "Armadillo de 9 bandas. Se enrolla en bola para defenderse. Siempre tiene cuatrillizos idénticos. Puede contener la respiración 6 minutos bajo el agua."),

            // B5 — Nocturario
            ("b5", "Cunaguaro Nocturno", "Leopardus pardalis",
             "De noche sus pupilas se dilatan al máximo para captar luz. Caza ratones, ranas y aves dormidas. Patrón de manchas único en cada individuo. Camina hasta 10 km por noche."),
            ("b5", "Murciélago Vampiro", "Desmodus rotundus",
             "Se alimenta exclusivamente de sangre. Su saliva tiene anticoagulante usado en medicina. Puede detectar calor corporal con sensores en su nariz. Vuela solo de noche."),
            ("b5", "Víbora de Pestañas", "Bothriechis schlegelii",
             "Escamas modificadas sobre los ojos parecen pestañas. Viene en colores verde, amarillo o rojo. Arborícola, caza al acecho. Fosetas loreales detectan calor de presas."),

            // Acuario
            ("acuario", "Pez Ángel", "Pterophyllum scalare",
             "Originario de la cuenca del Amazonas y Orinoco. Elegante pez de agua dulce con aletas alargadas. Puede vivir hasta 10 años. Muy popular en acuarismo."),
            ("acuario", "Pavón", "Cichla temensis",
             "Depredador de agua dulce, puede pesar hasta 12 kg. Muy combativo al ser pescado. Originario de la cuenca del Orinoco. Color verde-dorado con manchas."),
            ("acuario", "Piranha", "Pygocentrus nattereri",
             "Dientes afilados como navajas. Vive en cardúmenes. Pese a su fama, rara vez ataca humanos. Su mandíbula ejerce una fuerza enorme para su tamaño."),
            ("acuario", "Bagre Rayado", "Pseudoplatystoma fasciatum",
             "Pez gato grande de ríos venezolanos. Puede medir hasta 1 metro. Nocturno y depredador. Su carne es muy apreciada en gastronomía."),

            // Museo
            ("museo", "Carnotauro", "Carnotaurus sastrei",
             "Dinosaurio terópodo del Cretácico Superior. Tenía cuernos sobre los ojos y brazos extremadamente reducidos. Bípedo y carnívoro. Descubierto en Argentina en 1984."),
            ("museo", "Pteranodon", "Pteranodon longiceps",
             "Reptil volador del Cretácico con envergadura de hasta 7 metros. No tenía dientes. Su cresta ósea servía como timón en vuelo. Se alimentaba de peces."),
            ("museo", "Ammonite", "Ammonoidea",
             "Molusco marino extinto emparentado con el nautilus actual. Su concha espiral podía medir desde 1 cm hasta 2 metros. Vivió desde el Devónico hasta el Cretácico."),
        ];

        for (zona, comun, cientifico, desc) in animales {
            self.conn.execute(
                "INSERT INTO animales (zona_id, nombre_comun, nombre_cientifico, descripcion)
                 VALUES (?1, ?2, ?3, ?4)",
                params![zona, comun, cientifico, desc],
            ).unwrap();
        }
    }

    fn poblar_dialogos(&self) {
        let dialogos: &[(&str, &str, i32, &str)] = &[
            // Bienvenida - guía principal
            ("bienvenida", "Guía Carlos", 1,
             "¡Bienvenido al Zoológico Nacional! Soy Carlos, tu guía. Te acompañaré en este recorrido."),
            ("bienvenida", "Guía Carlos", 2,
             "Aquí conocerás animales increíbles de Venezuela y el mundo. ¡Cada zona tiene sorpresas!"),
            ("bienvenida", "Guía Carlos", 3,
             "Usa las flechas para moverte entre zonas. Explora todo el parque a tu ritmo."),
            ("bienvenida_teclado", "Guía Carlos", 4,
             "Presiona Z para interactuar y ver animales. Con X puedes volver atrás."),
            ("bienvenida_tactil", "Guía Carlos", 4,
             "Toca el botón A para interactuar y ver animales. Con B puedes volver atrás."),
            ("bienvenida", "Guía Carlos", 5,
             "También tienes una libreta donde se guardan los animales que descubras. ¡Intenta encontrarlos todos!"),
            ("bienvenida", "Guía Carlos", 6,
             "El mapa te ayudará a orientarte. Puedes abrirlo en cualquier momento."),
            ("bienvenida", "Guía Carlos", 7,
             "¡Ah! Y no olvides visitar el Acuario para pescar, y el Museo para desenterrar fósiles. ¡Buena suerte!"),

            // Guía del Acuario
            ("acuario_entrada", "Bióloga Marina", 1,
             "¡Hola! Bienvenido al Acuario. Aquí puedes pescar especies de agua dulce de Venezuela."),
            ("acuario_entrada", "Bióloga Marina", 2,
             "Espera a que un pez pique el anzuelo... ¡y tira con fuerza! Cada especie tiene su propio comportamiento."),

            // Guía del Museo
            ("museo_entrada", "Paleontólogo", 1,
             "¡Bienvenido al Museo Paleontológico! Aquí descubrirás criaturas del pasado remoto."),
            ("museo_entrada", "Paleontólogo", 2,
             "Tenemos un sitio de excavación donde puedes desenterrar fósiles reales. ¡Con cuidado y paciencia!"),
            ("museo_entrada", "Paleontólogo", 3,
             "Limpia la roca capa por capa hasta revelar el fósil escondido. Cada excavación es única."),

            // Guía del Aviario
            ("aviario_entrada", "Ornitóloga", 1,
             "¡Bienvenida al Aviario! Este es un espacio especial para observar aves en su hábitat."),
            ("aviario_entrada", "Ornitóloga", 2,
             "Las aves se mueven libremente. Tendrás que buscarlas y fotografiarlas. ¡Buena cacería fotográfica!"),

            // Eventos aleatorios
            ("evento_cria", "Cuidador", 1,
             "¡Mira! Ha nacido una cría en esta zona. La madre la protege con mucho cuidado."),
            ("evento_alimentacion", "Cuidador", 1,
             "Es hora de la alimentación. Los animales ya saben que es su momento favorito del día."),
            ("evento_enriquecimiento", "Veterinaria", 1,
             "Estamos haciendo enriquecimiento ambiental. Les damos juguetes y retos para mantenerlos activos."),
            ("evento_dato", "Guía Carlos", 1,
             "¿Sabías que Venezuela tiene más de 1.400 especies de aves? ¡Es uno de los países con mayor biodiversidad!"),
            ("evento_lluvia", "Guía Carlos", 1,
             "Parece que va a llover. Muchos animales se refugian, pero otros disfrutan la lluvia."),
        ];

        for (contexto, personaje, orden, texto) in dialogos {
            self.conn.execute(
                "INSERT INTO dialogos (contexto, personaje, orden, texto)
                 VALUES (?1, ?2, ?3, ?4)",
                params![contexto, personaje, orden, texto],
            ).unwrap();
        }
    }

    pub fn animales_zona(&self, escena: &Escena) -> Vec<Animal> {
        let mut stmt = self.conn.prepare(
            "SELECT a.id, a.zona_id, a.nombre_comun, a.nombre_cientifico, a.descripcion
             FROM animales a WHERE a.zona_id = ?1 ORDER BY a.id"
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

    /// Obtener diálogos de bienvenida, intercalando el específico de plataforma
    pub fn dialogos_bienvenida(&self, es_tactil: bool) -> Vec<DialogoDB> {
        let mut base = self.dialogos_por_contexto("bienvenida");
        let plataforma_ctx = if es_tactil { "bienvenida_tactil" } else { "bienvenida_teclado" };
        let especificos = self.dialogos_por_contexto(plataforma_ctx);

        // Insertar los específicos de plataforma en orden 4
        let mut resultado = Vec::new();
        let mut insertado = false;
        for d in &base {
            if d.orden >= 4 && !insertado {
                for e in &especificos {
                    resultado.push(e.clone());
                }
                insertado = true;
            }
            if d.orden != 4 || insertado {
                resultado.push(d.clone());
            }
        }
        if !insertado {
            for e in &especificos {
                resultado.push(e.clone());
            }
        }
        // Quitar duplicados de orden 4 base
        resultado.retain(|d| !(d.contexto == "bienvenida" && d.orden == 4));

        // Re-filtrar: solo los que no son de bienvenida con orden 4
        // Más simple: reconstruir
        let mut final_list = Vec::new();
        for d in self.dialogos_por_contexto("bienvenida") {
            if d.orden < 4 {
                final_list.push(d);
            }
        }
        for e in &especificos {
            final_list.push(e.clone());
        }
        for d in self.dialogos_por_contexto("bienvenida") {
            if d.orden > 4 {
                final_list.push(d);
            }
        }
        final_list
    }

    pub fn dialogos_evento(&self, tipo: &str) -> Vec<DialogoDB> {
        let ctx = format!("evento_{}", tipo);
        self.dialogos_por_contexto(&ctx)
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