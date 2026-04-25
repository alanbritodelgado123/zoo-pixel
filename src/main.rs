// src/main.rs
use macroquad::prelude::*;
use std::collections::HashMap;

mod animacion;
mod audio;
mod ciclo_dia;
mod config;
mod db;
mod escena;
mod estado;
mod eventos;
mod fondo;
mod guia;
mod input;
mod libreta;
mod minijuego;
mod plataforma;
mod save;
mod ui;

use audio::AudioManager;
use db::ZooDB;
use estado::{Estado, Pantalla};
use fondo::Fondos;
use ui::UiRenderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Zoo Pixel".to_owned(),
        window_width: 800,
        window_height: 480,
        window_resizable: !cfg!(target_os = "android"),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let db = ZooDB::new();

    // ═══════════════════════════════════════════════════════════════
    //  ✅ FUENTE
    // ═══════════════════════════════════════════════════════════════
    let font_bytes = include_bytes!("../assets/fonts/PressStart2P.ttf");
    let font = load_ttf_font_from_bytes(font_bytes)
        .expect("No se pudo cargar la fuente");
    println!("✅ Fuente cargada");

    // ═══════════════════════════════════════════════════════════════
    //  ✅ SPRITESHEET DE FONDOS (31 frames verticales para escenas)
    // ═══════════════════════════════════════════════════════════════
    let spritesheet_bytes = include_bytes!("../assets/fondos/spritesheet_vertical.png");
    let fondos = Fondos::new(spritesheet_bytes, 640.0, 480.0);
    println!("✅ Spritesheet de fondos cargado ({} frames)", fondos.total_frames());

    // ═══════════════════════════════════════════════════════════════
    //  ✅ ÍCONOS DE CATEGORÍA (pequeños, para pantalla de info)
    // ═══════════════════════════════════════════════════════════════
    let mut iconos_categoria: HashMap<String, Texture2D> = HashMap::new();

    {
        let bytes = include_bytes!("../assets/categorias/anfibios_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("anfibios".to_string(), tex);
        println!("  ✓ Ícono: anfibios");
    }
    {
        let bytes = include_bytes!("../assets/categorias/aves_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("aves".to_string(), tex);
        println!("  ✓ Ícono: aves");
    }
    {
        let bytes = include_bytes!("../assets/categorias/fosiles_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("fosiles".to_string(), tex);
        println!("  ✓ Ícono: fosiles");
    }
    {
        let bytes = include_bytes!("../assets/categorias/insectos_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("insectos".to_string(), tex);
        println!("  ✓ Ícono: insectos");
    }
    {
        let bytes = include_bytes!("../assets/categorias/mamiferos_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("mamiferos".to_string(), tex);
        println!("  ✓ Ícono: mamiferos");
    }
    {
        let bytes = include_bytes!("../assets/categorias/peces_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("peces".to_string(), tex);
        println!("  ✓ Ícono: peces");
    }
    {
        let bytes = include_bytes!("../assets/categorias/primates_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("primates".to_string(), tex);
        println!("  ✓ Ícono: primates");
    }
    {
        let bytes = include_bytes!("../assets/categorias/reptiles_inspyrenet.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        iconos_categoria.insert("reptiles".to_string(), tex);
        println!("  ✓ Ícono: reptiles");
    }
    println!("✅ {} íconos de categoría cargados", iconos_categoria.len());

    // ═══════════════════════════════════════════════════════════════
    //  ✅ FONDOS DE CATEGORÍA (para pantalla de info, oscurecidos)
    // ═══════════════════════════════════════════════════════════════
    let mut fondos_categoria: HashMap<String, Texture2D> = HashMap::new();

    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/anfibios_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("anfibios".to_string(), tex);
        println!("  ✓ Fondo: anfibios ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/Aves_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("aves".to_string(), tex);
        println!("  ✓ Fondo: aves ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/fosiles_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("fosiles".to_string(), tex);
        println!("  ✓ Fondo: fosiles ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/insectos_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("insectos".to_string(), tex);
        println!("  ✓ Fondo: insectos ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/mamiferos_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("mamiferos".to_string(), tex);
        println!("  ✓ Fondo: mamiferos ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/peces_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("peces".to_string(), tex);
        println!("  ✓ Fondo: peces ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/primates_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("primates".to_string(), tex);
        println!("  ✓ Fondo: primates ({}x{})", w, h);
    }
    {
        let bytes = include_bytes!("../assets/fondos/categorias_pixel/reptiles_pixel.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        let w = tex.width();
        let h = tex.height();
        fondos_categoria.insert("reptiles".to_string(), tex);
        println!("  ✓ Fondo: reptiles ({}x{})", w, h);
    }
    println!("✅ {} fondos de categoría cargados", fondos_categoria.len());

    // ═══════════════════════════════════════════════════════════════
    //  ✅ TEXTURAS DE ANIMALES (imágenes pixel art)
    //  Clave: nombre_cientifico en minúsculas
    // ═══════════════════════════════════════════════════════════════
    let mut texturas_animales: HashMap<String, Texture2D> = HashMap::new();

    macro_rules! cargar_animal {
        ($map:expr, $path:expr, $clave:expr) => {{
            let bytes = include_bytes!(concat!("../assets/animales_pixel/", $path));
            let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
            tex.set_filter(FilterMode::Nearest);
            $map.insert($clave.to_string(), tex);
        }};
    }

    // ── Z1 ──────────────────────────────────────────────────────────
    cargar_animal!(texturas_animales, "Z1/Z1-1_babilla_caiman_crocodilus.png", "caiman crocodilus");
    cargar_animal!(texturas_animales, "Z1/Z1-1_chigüire_hydrochoerus_hydrochaeris.png", "hydrochoerus hydrochaeris");
    cargar_animal!(texturas_animales, "Z1/Z1-1_cunaguaro_leopardus_pardalis.png", "leopardus pardalis");
    cargar_animal!(texturas_animales, "Z1/Z1-1_garza_real_ardea_alba.png", "ardea alba");
    cargar_animal!(texturas_animales, "Z1/Z1-2_coondor_de_los_andes_vultur_gryphus.png", "vultur gryphus");
    cargar_animal!(texturas_animales, "Z1/Z1-2_oso_frontino_tremarctos_ornatus.png", "tremarctos ornatus");
    cargar_animal!(texturas_animales, "Z1/Z1-2_paujii_de_copete_de_piedra_pauxi_pauxi.png", "pauxi pauxi");
    cargar_animal!(texturas_animales, "Z1/Z1-2_venado_caramerudo_odocoileus_virginianus.png", "odocoileus virginianus");
    cargar_animal!(texturas_animales, "Z1/Z1-3_anaconda_verde_eunectes_murinus.png", "eunectes murinus");
    cargar_animal!(texturas_animales, "Z1/Z1-3_guacamaya_bandera_ara_macao.png", "ara macao");
    cargar_animal!(texturas_animales, "Z1/Z1-3_jaguar_panthera_onca.png", "panthera onca");
    cargar_animal!(texturas_animales, "Z1/Z1-3_mono_araguato_alouatta_seniculus.png", "alouatta seniculus");
    cargar_animal!(texturas_animales, "Z1/Z1-5_marimonda_de_la_sierra_ateles_hybridus.png", "ateles hybridus");
    cargar_animal!(texturas_animales, "Z1/Z1-5_pava_negra_aburria_aburri.png", "aburria aburri");
    cargar_animal!(texturas_animales, "Z1/Z1-5_puma_puma_concolor.png", "puma concolor");
    cargar_animal!(texturas_animales, "Z1/Z1-5_tucaan_real_ramphastos_sulfuratus.png", "ramphastos sulfuratus");

    // ── Z2 ──────────────────────────────────────────────────────────
    cargar_animal!(texturas_animales, "Z2/Z2-1_aangaro_psittacara_wagleri.png", "psittacara wagleri");
    cargar_animal!(texturas_animales, "Z2/Z2-1_cotorra_margaritenna_amazona_barbadensis.png", "amazona barbadensis");
    cargar_animal!(texturas_animales, "Z2/Z2-1_cunaguaro_de_margarita_leopardus_pardalis_pardalis.png", "leopardus pardalis pardalis");
    cargar_animal!(texturas_animales, "Z2/Z2-1_venado_de_margarita_odocoileus_virginianus_margari.png", "odocoileus virginianus margaritae");
    cargar_animal!(texturas_animales, "Z2/Z2-2_caimaan_de_la_costa_crocodylus_acutus.png", "crocodylus acutus");
    cargar_animal!(texturas_animales, "Z2/Z2-2_corocoro_rojo_eudocimus_ruber.png", "eudocimus ruber");
    cargar_animal!(texturas_animales, "Z2/Z2-2_flamenco_del_caribe_phoenicopterus_ruber.png", "phoenicopterus ruber");
    cargar_animal!(texturas_animales, "Z2/Z2-2_tortuga_carey_eretmochelys_imbricata.png", "eretmochelys imbricata");
    cargar_animal!(texturas_animales, "Z2/Z2-3_garzoon_soldado_jabiru_mycteria.png", "jabiru mycteria");
    cargar_animal!(texturas_animales, "Z2/Z2-3_manatii_del_caribe_trichechus_manatus.png", "trichechus manatus");
    cargar_animal!(texturas_animales, "Z2/Z2-3_mono_capuchino_del_orinoco_cebus_albifrons.png", "cebus albifrons");
    cargar_animal!(texturas_animales, "Z2/Z2-3_perro_de_agua_pteronura_brasiliensis.png", "pteronura brasiliensis");
    cargar_animal!(texturas_animales, "Z2/Z2-4_harpiia_harpia_harpyja.png", "harpia harpyja");
    cargar_animal!(texturas_animales, "Z2/Z2-4_oso_hormiguero_gigante_myrmecophaga_tridactyla.png", "myrmecophaga tridactyla");
    cargar_animal!(texturas_animales, "Z2/Z2-4_sapito_minero_dendrobates_leucomelas.png", "dendrobates leucomelas");
    cargar_animal!(texturas_animales, "Z2/Z2-4_tepuihyla_tepuihyla_rodriguezi.png", "tepuihyla rodriguezi");
    cargar_animal!(texturas_animales, "Z2/Z2-5_gallo_de_roca_guayanees_rupicola_rupicola.png", "rupicola rupicola");
    cargar_animal!(texturas_animales, "Z2/Z2-5_mono_viuda_cheracebus_lugens.png", "cheracebus lugens");
    cargar_animal!(texturas_animales, "Z2/Z2-5_saltariin_de_cabeza_dorada_ceratopipra_erythroceph.png", "ceratopipra erythrocephala");
    cargar_animal!(texturas_animales, "Z2/Z2-5_uacarii_de_cabeza_negra_cacajao_melanocephalus.png", "cacajao melanocephalus");

    // ── Z3 ──────────────────────────────────────────────────────────
    cargar_animal!(texturas_animales, "Z3/Z3-1_quetzal_dorado_pharomachrus_fulgidus.png", "pharomachrus fulgidus");
    cargar_animal!(texturas_animales, "Z3/Z3-1_sorocuaa_acollarado_trogon_collaris.png", "trogon collaris");
    cargar_animal!(texturas_animales, "Z3/Z3-2_bagre_de_maracaibo_perrunichthys_perruno.png", "perrunichthys perruno");
    cargar_animal!(texturas_animales, "Z3/Z3-2_chavarrii_chauna_chavaria.png", "chauna chavaria");
    cargar_animal!(texturas_animales, "Z3/Z3-2_pato_cuchara_anas_clypeata.png", "anas clypeata");
    cargar_animal!(texturas_animales, "Z3/Z3-2_pavoon_de_maracaibo_cichla_temensis.png", "cichla temensis");
    cargar_animal!(texturas_animales, "Z3/Z3-3_cuspa_cachicamo_priodontes_maximus.png", "priodontes maximus");
    cargar_animal!(texturas_animales, "Z3/Z3-3_guaacharo_steatornis_caripensis.png", "steatornis caripensis");
    cargar_animal!(texturas_animales, "Z3/Z3-3_mono_capuchino_cebus_olivaceus.png", "cebus olivaceus");
    cargar_animal!(texturas_animales, "Z3/Z3-3_trogoon_grande_trogon_massena.png", "trogon massena");
    cargar_animal!(texturas_animales, "Z3/Z3-4_botuto_lobatus_gigas.png", "lobatus gigas");
    cargar_animal!(texturas_animales, "Z3/Z3-4_langosta_espinosa_panulirus_argus.png", "panulirus argus");
    cargar_animal!(texturas_animales, "Z3/Z3-4_pez_loro_scarus_guacamaia.png", "scarus guacamaia");
    cargar_animal!(texturas_animales, "Z3/Z3-4_pez_ngel_francees_pomacanthus_paru.png", "pomacanthus paru");
    cargar_animal!(texturas_animales, "Z3/Z3-5_caimaan_del_orinoco_crocodylus_intermedius.png", "crocodylus intermedius");
    cargar_animal!(texturas_animales, "Z3/Z3-5_caribe_colorado_pygocentrus_nattereri.png", "pygocentrus nattereri");
    cargar_animal!(texturas_animales, "Z3/Z3-5_raya_motoro_potamotrygon_motoro.png", "potamotrygon motoro");
    cargar_animal!(texturas_animales, "Z3/Z3-5_tonina_inia_geoffrensis.png", "inia geoffrensis");

    // ── Z4 ──────────────────────────────────────────────────────────
    cargar_animal!(texturas_animales, "Z4/Z4-1_culebra_lora_leptophis_ahaetulla.png", "leptophis ahaetulla");
    cargar_animal!(texturas_animales, "Z4/Z4-1_danta_tapirus_terrestris.png", "tapirus terrestris");
    cargar_animal!(texturas_animales, "Z4/Z4-1_guila_crestada_morphnus_guianensis.png", "morphnus guianensis");
    cargar_animal!(texturas_animales, "Z4/Z4-1_mono_maicero_sapajus_apella.png", "sapajus apella");
    cargar_animal!(texturas_animales, "Z4/Z4-2_conejo_de_paaramo_sylvilagus_varynaensis.png", "sylvilagus varynaensis");
    cargar_animal!(texturas_animales, "Z4/Z4-2_guila_real_de_los_andes_geranoaetus_melanoleucus.png", "geranoaetus melanoleucus");
    cargar_animal!(texturas_animales, "Z4/Z4-2_musaranna_de_los_andes_cryptotis_meridensis.png", "cryptotis meridensis");
    cargar_animal!(texturas_animales, "Z4/Z4-2_pato_de_torrente_merganetta_armata.png", "merganetta armata");
    cargar_animal!(texturas_animales, "Z4/Z4-3_galaapago_llanero_podocnemis_vogli.png", "podocnemis vogli");
    cargar_animal!(texturas_animales, "Z4/Z4-3_zorro_cangrejero_cerdocyon_thous.png", "cerdocyon thous");
    cargar_animal!(texturas_animales, "Z4/Z4-3_pavo_real_o_pavoon_negro_crax_alector.png", "crax alector");
    cargar_animal!(texturas_animales, "Z4/Z4-4_cardenal_guajiro_cardinalis_phoeniceus.png", "cardinalis phoeniceus");
    cargar_animal!(texturas_animales, "Z4/Z4-4_cascabel_crotalus_durissus.png", "crotalus durissus");
    cargar_animal!(texturas_animales, "Z4/Z4-4_conejo_sabanero_sylvilagus_floridanus.png", "sylvilagus floridanus");
    cargar_animal!(texturas_animales, "Z4/Z4-4_turpial_guajiro_icterus_icterus_ridgwayi.png", "icterus icterus ridgwayi");
    cargar_animal!(texturas_animales, "Z4/Z4-5_halcoon_de_monte_micrastur_semitorquatus.png", "micrastur semitorquatus");
    cargar_animal!(texturas_animales, "Z4/Z4-5_oso_melero_tamandua_tetradactyla.png", "tamandua tetradactyla");
    cargar_animal!(texturas_animales, "Z4/Z4-5_sapito_de_yapacana_minyobates_steyermarki.png", "minyobates steyermarki");
    cargar_animal!(texturas_animales, "Z4/Z4-5_tucancito_de_pico_maculado_selenidera_culik.png", "selenidera culik");

    // ── Z5 ──────────────────────────────────────────────────────────
    cargar_animal!(texturas_animales, "Z5/z5-1_glyptotherium_venezuelensis.png", "glyptotherium venezuelensis");
    cargar_animal!(texturas_animales, "Z5/z5-1_megatherium_americanum.png", "megatherium americanum");
    cargar_animal!(texturas_animales, "Z5/z5-1_notiomastodon_platensis.png", "notiomastodon platensis");
    cargar_animal!(texturas_animales, "Z5/z5-1_smilodon_populator.png", "smilodon populator");
    cargar_animal!(texturas_animales, "Z5/z5-2_lutjanus_campechanus.png", "lutjanus campechanus");
    cargar_animal!(texturas_animales, "Z5/z5-2_megalops_atlanticus.png", "megalops atlanticus");
    cargar_animal!(texturas_animales, "Z5/z5-2_scomberomorus_cavalla.png", "scomberomorus cavalla");
    cargar_animal!(texturas_animales, "Z5/z5-2_semaprochilodus_laticeps.png", "semaprochilodus laticeps");
    cargar_animal!(texturas_animales, "Z5/z5-3_chelonoidis_carbonarius.png", "chelonoidis carbonarius");
    cargar_animal!(texturas_animales, "Z5/z5-3_crotalus_durissus.png", "crotalus durissus cumanensis");
    cargar_animal!(texturas_animales, "Z5/z5-3_rhinella_marina.png", "rhinella marina");
    cargar_animal!(texturas_animales, "Z5/z5-3_thecadactylus_rapicauda.png", "thecadactylus rapicauda");
    cargar_animal!(texturas_animales, "Z5/z5-4_atta_laevigata.png", "atta laevigata");
    cargar_animal!(texturas_animales, "Z5/z5-4_dynastes_hercules.png", "dynastes hercules");
    cargar_animal!(texturas_animales, "Z5/z5-4_fulgora_laternaria.png", "fulgora laternaria");
    cargar_animal!(texturas_animales, "Z5/z5-5_ara_ararauna.png", "ara ararauna");
    cargar_animal!(texturas_animales, "Z5/z5-5_icterus_icterus.png", "icterus icterus");
    cargar_animal!(texturas_animales, "Z5/z5-5_procnias_albus.png", "procnias albus");
    cargar_animal!(texturas_animales, "Z5/z5-5_spinus_cucullatus.png", "spinus cucullatus");

    println!("✅ {} texturas de animales cargadas", texturas_animales.len());

    // ═══════════════════════════════════════════════════════════════
    //  ✅ AUDIO - Fallback de ambiente
    // ═══════════════════════════════════════════════════════════════
    let mut audio = AudioManager::new();
    audio.set_fallback(
        include_bytes!("../assets/audio/ambiente/amb_entrada.ogg")
    ).await;

    // ═══════════════════════════════════════════════════════════════
    //  ✅ EFECTOS DE UI
    // ═══════════════════════════════════════════════════════════════
    audio.agregar_efecto(
        "transicion",
        include_bytes!("../assets/audio/efectos/fx_transicion.wav"),
    ).await;
    audio.agregar_efecto(
        "boton",
        include_bytes!("../assets/audio/efectos/fx_boton.wav"),
    ).await;
    println!("✅ Efectos de UI cargados");

    // ═══════════════════════════════════════════════════════════════
    //  ✅ GRITOS POR CATEGORÍA (8 archivos WAV)
    // ═══════════════════════════════════════════════════════════════
    audio.agregar_efecto(
        "grito_anfibios",
        include_bytes!("../assets/audio/categorias/grito_anfibios.wav"),
    ).await;
    println!("  ✓ Grito: anfibios");

    audio.agregar_efecto(
        "grito_aves",
        include_bytes!("../assets/audio/categorias/grito_aves.wav"),
    ).await;
    println!("  ✓ Grito: aves");

    audio.agregar_efecto(
        "grito_fosiles",
        include_bytes!("../assets/audio/categorias/grito_fosiles.wav"),
    ).await;
    println!("  ✓ Grito: fosiles");

    audio.agregar_efecto(
        "grito_insectos",
        include_bytes!("../assets/audio/categorias/grito_insectos.wav"),
    ).await;
    println!("  ✓ Grito: insectos");

    audio.agregar_efecto(
        "grito_mamiferos",
        include_bytes!("../assets/audio/categorias/grito_mamiferos.wav"),
    ).await;
    println!("  ✓ Grito: mamiferos");

    audio.agregar_efecto(
        "grito_peces",
        include_bytes!("../assets/audio/categorias/grito_peces.wav"),
    ).await;
    println!("  ✓ Grito: peces");

    audio.agregar_efecto(
        "grito_primates",
        include_bytes!("../assets/audio/categorias/grito_primates.wav"),
    ).await;
    println!("  ✓ Grito: primates");

    audio.agregar_efecto(
        "grito_reptiles",
        include_bytes!("../assets/audio/categorias/grito_reptiles.wav"),
    ).await;
    println!("  ✓ Grito: reptiles");

    println!("✅ 8 gritos de categoría cargados");

    // ═══════════════════════════════════════════════════════════════
    //  ✅ AMBIENTES POR ESCENA
    // ═══════════════════════════════════════════════════════════════
    for escena in escena::Escena::TODAS {
        audio.agregar_ambiente(
            *escena,
            include_bytes!("../assets/audio/ambiente/amb_entrada.ogg"),
        ).await;
    }
    println!("✅ {} ambientes de escena cargados", escena::Escena::TODAS.len());

    // ═══════════════════════════════════════════════════════════════
    //  ✅ TEXTURAS DE GUÍAS
    // ═══════════════════════════════════════════════════════════════
    let textura_eli = {
        let bytes = include_bytes!("../assets/guias/guiaEli.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        println!("✅ Guía Eli cargada ({}x{})", tex.width(), tex.height());
        Some(tex)
    };

    let textura_ani = {
        let bytes = include_bytes!("../assets/guias/guiaAni.png");
        let tex = Texture2D::from_file_with_format(&bytes[..], Some(ImageFormat::Png));
        tex.set_filter(FilterMode::Nearest);
        println!("✅ Guía Ani cargada ({}x{})", tex.width(), tex.height());
        Some(tex)
    };

    // ═══════════════════════════════════════════════════════════════
    //  ✅ INICIALIZAR UI Y ESTADO
    // ═══════════════════════════════════════════════════════════════
    let iconos_count = iconos_categoria.len();
    let fondos_count = fondos_categoria.len();
    let animales_count = texturas_animales.len();

    let ui = UiRenderer::new(
        font,
        textura_eli,
        textura_ani,
        iconos_categoria,
        fondos_categoria,
        texturas_animales,
    );
    let mut estado = Estado::new(&db);
    estado.duracion_transicion = (audio.duracion_transicion() + 0.2)
        .max(config::TRANSITION_MIN);
    audio.iniciar_ambiente(estado.escena);

    println!("");
    println!("🎮 ═══════════════════════════════════════════════════════");
    println!("🎮   Zoo Pixel v0.7.0 - Fauna Venezolana Interactiva");
    println!("🎮 ═══════════════════════════════════════════════════════");
    println!("📍 Escena inicial: {:?}", estado.escena);
    println!("📐 Resolución: {}x{}", screen_width(), screen_height());
    println!("🎨 Fondos: {} frames en spritesheet", fondos.total_frames());
    println!("🖼️  Íconos: {} categorías", iconos_count);
    println!("🌄 Fondos cat: {} categorías", fondos_count);
    println!("🦎 Animales: {} texturas", animales_count);
    println!("🔊 Audio: 8 gritos + {} ambientes", escena::Escena::TODAS.len());
    println!("🎮 ═══════════════════════════════════════════════════════");
    println!("");

    // ═══════════════════════════════════════════════════════════════
    //  ✅ GAME LOOP
    // ═══════════════════════════════════════════════════════════════
    loop {
        let dt = get_frame_time().min(0.1);

        // ── Input ────────────────────────────────────────────────────
        for accion in input::leer_teclado() {
            estado.procesar_accion(accion, &db);
        }
        for accion in input::leer_tactil(&estado) {
            estado.procesar_accion(accion, &db);
        }

        // ── Update ───────────────────────────────────────────────────
        estado.update(dt, &db);
        audio.update(dt);

        // ── Audio: Volumen desde config activa ───────────────────────
        let (vol_m, vol_e) = if matches!(estado.pantalla, Pantalla::Config) {
            (estado.menu_config.volumen_musica, estado.menu_config.volumen_efectos)
        } else {
            (estado.save.config.volumen_musica, estado.save.config.volumen_efectos)
        };
        audio.set_volumen_musica(vol_m);
        audio.set_volumen_efectos(vol_e);

        // Transición de audio entre escenas
        if let Some(destino) = estado.necesita_transicion_audio.take() {
            audio.transicionar_a(destino);
        }

        // ── Gritos de categoría ──────────────────────────────────────
        if estado.necesita_sonido_animal {
            let categoria = match &estado.modo {
                estado::ModoVista::ViendoAnimal { animal, .. } => {
                    Some(&animal.categoria)
                }
                estado::ModoVista::Foto { animales, indice_actual, foto_tomada, .. } => {
                    if *foto_tomada {
                        Some(&animales[*indice_actual].categoria)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(cat) = categoria {
                audio.reproducir_grito_categoria(cat);
            }

            estado.necesita_sonido_animal = false;
        }

        // ── Render ───────────────────────────────────────────────────
        ui.render(&estado, &fondos, &audio);

        // Overlay PC (indicadores de botones)
        let mostrar_overlay_pc = estado.mostrar_overlay
            && !cfg!(target_os = "android")
            && !estado.en_pantalla_info()
            && !estado.dialogo.activo
            && !estado.en_transicion();
        if mostrar_overlay_pc {
            ui::render_pc_overlay(&estado, &ui.font);
        }

        // Filtro CRT
        let crt_activo = if matches!(estado.pantalla, Pantalla::Config) {
            estado.menu_config.crt
        } else {
            estado.save.config.crt
        };
        if crt_activo {
            ui::render_crt();
        }

        next_frame().await;
    }
}