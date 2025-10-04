use crate::{
    cartridge::cartridge::Cartridge,
    mmu::{boot_rom::BootROM, mmu::MMU},
    ppu::lcd::LCD,
};
use {
    cairo::{Context, ImageSurface},
    gtk::{
        Application, ApplicationWindow, Box, DrawingArea, FileChooserAction, FileChooserDialog,
        FileFilter, ResponseType,
        ResponseType::{Accept, Cancel},
        Window, cairo,
        gio::{ActionEntry, Menu, MenuItem},
        glib,
        glib::{ControlFlow::Continue, object::ObjectExt},
        prelude::{
            ActionMapExtManual, ApplicationExt, ApplicationExtManual, BoxExt, Cast, DialogExt,
            DrawingAreaExtManual, FileChooserExt, FileExt, GtkApplicationExt, GtkWindowExt,
            WidgetExt,
        },
    },
    std::{
        sync::{Arc, Mutex, MutexGuard},
        time::{Duration, Instant},
    },
};

pub struct Emulator {
    app: Application,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            app: Application::builder()
                .application_id("org.gtk_rs.EmulatorWindow")
                .build(),
        }
    }

    fn setup_file_chooser(window: &ApplicationWindow) -> FileChooserDialog {
        let dialog: FileChooserDialog = FileChooserDialog::new(
            Some("Ouvrir une ROM Game Boy"),
            Some(window),
            FileChooserAction::Open,
            &[("Annuler", Cancel), ("Ouvrir", Accept)],
        );

        let filter: FileFilter = FileFilter::new();
        filter.set_name(Some("ROM Game Boy"));
        filter.add_pattern("*.gb");
        dialog.add_filter(&filter);

        dialog
    }

    fn handle_file_response(dialog: &FileChooserDialog, response: ResponseType) {
        if response == Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    if let Some(path_str) = path.to_str() {
                        println!("ROM sélectionnée : {}", path_str);
                        if let Some(window) = dialog.transient_for() {
                            if let Some(mmu_ptr) = unsafe { window.data::<Arc<Mutex<MMU>>>("mmu") }
                            {
                                let mmu_ref: &Arc<Mutex<MMU>> = unsafe { mmu_ptr.as_ref() };
                                if let Ok(mut mmu) = mmu_ref.lock() {
                                    let cartridge: Cartridge = Cartridge::insert(path_str);
                                    let title: String =
                                        format!("Game Boy - {}", cartridge.get_title());
                                    window.set_title(Some(&title));
                                    mmu.set_cartridge(cartridge);
                                    mmu.boot_init_sequence();
                                }
                            }
                        }
                    }
                }
            }
        }
        dialog.close();
    }

    fn draw_lcd_area(cr: &Context, lcd: &mut LCD, mmu: &mut MMU) {
        const MARGIN: f64 = 80.0;

        let (lcd_width, lcd_height) = lcd.get_screen_dimensions();

        let window_width: f64 = cr.clip_extents().unwrap().2 as f64;
        let window_height: f64 = cr.clip_extents().unwrap().3 as f64;

        let scale_x: f64 = (window_width - 2.0 * MARGIN) / lcd_width as f64;
        let scale_y: f64 = (window_height - 2.0 * MARGIN) / lcd_height as f64;
        let scale: f64 = scale_x.min(scale_y);

        // --- Correction ici ---
        let ppu = mmu.get_ppu();
        let frame_data: ImageSurface = lcd.render_frame(ppu);

        let surface: ImageSurface = frame_data;
        let scaled_width: f64 = lcd_width as f64 * scale;
        let scaled_height: f64 = lcd_height as f64 * scale;
        let pos_x: f64 = ((window_width - scaled_width) / 2.0).max(MARGIN);
        let pos_y: f64 = ((window_height - scaled_height) / 2.0).max(MARGIN);

        cr.translate(pos_x, pos_y);
        cr.scale(scale, scale);

        cr.set_source_surface(&surface, 0.0, 0.0)
            .expect("Impossible de définir la source");
        cr.paint().expect("Impossible de dessiner");
    }

    fn draw_debug_area(cr: &cairo::Context, lcd: &mut LCD, mmu: &mut MMU) {
        const MARGIN: f64 = 20.0;
        const TILEMAP_SIZE: f64 = 256.0;

        let window_width: f64 = cr.clip_extents().unwrap().2 as f64;
        let window_height: f64 = cr.clip_extents().unwrap().3 as f64;

        let scale_x: f64 = (window_width - 2.0 * MARGIN) / TILEMAP_SIZE;
        let scale_y: f64 = (window_height - 2.0 * MARGIN) / TILEMAP_SIZE;
        let scale: f64 = scale_x.min(scale_y);

        let pos_x: f64 = (window_width - TILEMAP_SIZE * scale) / 2.0;
        let pos_y: f64 = (window_height - TILEMAP_SIZE * scale) / 2.0;

        cr.translate(pos_x, pos_y);
        cr.scale(scale, scale);

        let ppu: &mut crate::ppu::ppu::PPU = mmu.get_ppu();
        let surface: ImageSurface = lcd.render_debug_tile_map(ppu);

        cr.set_source_surface(&surface, 0.0, 0.0)
            .expect("Impossible de définir la source de débogage");
        cr.paint().expect("Impossible de dessiner le débogage");
    }

    pub fn start(&self) {
        let app_clone = self.app.clone();
        self.app.connect_startup(move |_| {
            setup_menu(&app_clone);
        });

        self.app.connect_activate(Self::create_main_window);
        self.app.run();
    }

    fn create_main_window(app: &Application) {
        let window: ApplicationWindow = ApplicationWindow::builder()
            .application(app)
            .title("Game Boy")
            .default_width(700)
            .default_height(700)
            .show_menubar(true)
            .build();

        let lcd_area: DrawingArea = DrawingArea::builder().hexpand(true).vexpand(true).build();

        let mut lcd: LCD = LCD::new();
        let mmu: Arc<Mutex<MMU>> = Arc::new(Mutex::new(MMU::new()));
        let mmu_clone_for_draw: Arc<Mutex<MMU>> = Arc::clone(&mmu);

        lcd_area.set_draw_func(move |_, cr, _, _| {
            if let Ok(mut mmu) = mmu_clone_for_draw.lock() {
                Self::draw_lcd_area(cr, &mut lcd, &mut mmu);
            }
        });

        window.set_child(Some(&lcd_area));

        unsafe {
            window.set_data("mmu", mmu.clone());
        }

        Self::create_debug_window(app, mmu.clone());

        let frame_duration: Duration = Duration::from_secs_f64(1.0 / 60.0);
        let last_frame: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
        let last_frame_clone: Arc<Mutex<Instant>> = Arc::clone(&last_frame);
        let mmu_clone_for_tick: Arc<Mutex<MMU>> = Arc::clone(&mmu);

        let tick = move || {
            let current_time: Instant = Instant::now();
            let mut last_frame: std::sync::MutexGuard<'_, Instant> =
                last_frame_clone.lock().unwrap();
            let elapsed: Duration = current_time.duration_since(*last_frame);

            if elapsed >= frame_duration {
                if let Ok(mut mmu) = mmu_clone_for_tick.lock() {
                    mmu.on_frame();
                }

                lcd_area.queue_draw();
                *last_frame = current_time;
            }

            Continue
        };

        glib::timeout_add_local(Duration::from_millis(1), tick);

        window.present();
    }

    fn create_debug_window(app: &Application, mmu: Arc<Mutex<MMU>>) {
        let debug_window = ApplicationWindow::builder()
            .application(app)
            .title("Game Boy - Debug")
            .default_width(1024)
            .default_height(512)
            .build();

        let hbox: Box = Box::new(gtk::Orientation::Horizontal, 10);
        hbox.set_homogeneous(true);

        let tilemap_area: DrawingArea = DrawingArea::builder().hexpand(true).vexpand(true).build();
        let tileset_area: DrawingArea = DrawingArea::builder().hexpand(true).vexpand(true).build();

        let mut lcd_tile_map: LCD = LCD::new();
        let mut lcd_tile_set: LCD = LCD::new();
        let mmu_clone1: Arc<Mutex<MMU>> = Arc::clone(&mmu);
        let mmu_clone2: Arc<Mutex<MMU>> = Arc::clone(&mmu);

        tilemap_area.set_draw_func(move |_, cr, width, height| {
            if let Ok(mut mmu) = mmu_clone1.lock() {
                let scale: f64 = f64::min(width as f64 / 256.0, height as f64 / 256.0);
                let x: f64 = (width as f64 - (256.0 * scale)) / 2.0;
                let y: f64 = (height as f64 - (256.0 * scale)) / 2.0;

                cr.translate(x, y);
                cr.scale(scale, scale);

                Self::draw_debug_area(cr, &mut lcd_tile_map, &mut mmu);
            }
        });

        tileset_area.set_draw_func(move |_, cr, width, height| {
            if let Ok(mut mmu) = mmu_clone2.lock() {
                let ppu = mmu.get_ppu();
                let surface: ImageSurface = lcd_tile_set.render_tile_set(ppu);
                let scale: f64 = f64::min(width as f64 / 256.0, height as f64 / 256.0);

                let x: f64 = (width as f64 - (256.0 * scale)) / 2.0;
                let y: f64 = (height as f64 - (256.0 * scale)) / 2.0;

                cr.translate(x, y);
                cr.scale(scale, scale);
                cr.set_source_surface(&surface, 0.0, 0.0).unwrap();
                cr.paint().unwrap();
            }
        });

        hbox.append(&tilemap_area);
        hbox.append(&tileset_area);

        debug_window.set_child(Some(&hbox));

        let frame_duration: Duration = Duration::from_secs_f64(1.0 / 60.0);
        let last_frame: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
        let last_frame_clone: Arc<Mutex<Instant>> = Arc::clone(&last_frame);

        let tick = move || {
            let current_time: Instant = Instant::now();
            let mut last_frame: MutexGuard<'_, Instant> = last_frame_clone.lock().unwrap();
            let elapsed: Duration = current_time.duration_since(*last_frame);

            if elapsed >= frame_duration {
                tilemap_area.queue_draw();
                tileset_area.queue_draw();
                *last_frame = current_time;
            }

            Continue
        };

        glib::timeout_add_local(Duration::from_millis(1), tick);

        debug_window.present();
    }
}

fn setup_menu(app: &Application) {
    let open: ActionEntry<Application> = ActionEntry::builder("open")
        .activate(|app: &Application, _, _| {
            let window: Window = app.active_window().unwrap();
            let app_window: &ApplicationWindow = window
                .downcast_ref::<ApplicationWindow>()
                .expect("Active window is not an ApplicationWindow");
            let dialog: FileChooserDialog = Emulator::setup_file_chooser(app_window);

            dialog.connect_response(move |dialog, response| {
                Emulator::handle_file_response(dialog, response);
            });

            dialog.show();
        })
        .build();

    let quit: ActionEntry<Application> = ActionEntry::builder("quit")
        .activate(|app: &Application, _, _| app.quit())
        .build();

    app.add_action_entries([open, quit]);

    let menu_bar: Menu = Menu::new();
    let file_menu: Menu = Menu::new();

    let open_item: MenuItem = MenuItem::new(Some("Ouvrir..."), Some("app.open"));
    let quit_item: MenuItem = MenuItem::new(Some("Quitter"), Some("app.quit"));

    file_menu.append_item(&open_item);
    file_menu.append_item(&quit_item);

    menu_bar.append_submenu(Some("Fichier"), &file_menu);
    app.set_menubar(Some(&menu_bar));
}
