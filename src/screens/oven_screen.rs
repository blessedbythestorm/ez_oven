use std::{fs, fs::File, io::{BufReader, BufRead, Write}, path::{Path, PathBuf}, process::{Command, Stdio}, thread, sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}}};
use std::io::{stdin, stdout, Stdout};
use eframe::{epi, epi::{App, Frame}, egui::Context};
use egui::{Label, ScrollArea, TextStyle};
use crate::screens::Screen;

pub struct Oven {
    engine_directory: String,
    project_directory: String,
    content_modules: Vec<String>,
    content_module: usize,
    is_cooking: Arc<AtomicBool>,
    log: Arc<RwLock<String>>,
}

impl Screen for Oven {
    fn create() -> Box<dyn Screen> where Self: Sized {
        let mut oven = Box::new(Oven {
            engine_directory: "".to_string(),
            project_directory: "".to_string(),
            content_modules: Vec::new(),
            content_module: 0,
            is_cooking: Arc::new(AtomicBool::new(false)),
            log: Arc::new(RwLock::new(String::new())),
        });
        oven.initialize();
        oven
    }

    fn initialize(&mut self) {
        let config = ini::Ini::load_from_file("config.ini").unwrap();
        if let Some(settings_section) = config.section(Some("settings")) {
            if let Some(engine_location) = settings_section.get("engine_directory") {
                self.engine_directory = engine_location.to_string();
            }
            if let Some(project_directory) = settings_section.get("project_directory") {
                self.project_directory = project_directory.to_string();
                self.update_modules();
            }
            if let Some(content_module) = settings_section.get("content_module") {
                self.content_module = content_module.parse::<usize>().unwrap();
            }
        }
    }

    fn draw(&mut self, ctx: &Context, _frame: &Frame) -> Option<Box<dyn Screen>> {
        egui::Window::new("Oven")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2 { x: 0.0, y: 0.0 })
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {


                    let valid_engine = is_engine_directory(&self.engine_directory);
                    let project_name = PathBuf::from(self.project_directory.clone()).file_name().unwrap().to_str().unwrap().to_string();
                    let valid_project = is_project_directory(&self.project_directory, &project_name);

                    // Handle persistence of Engine Directory
                    if !valid_engine {
                        ui.colored_label(egui::Color32::RED, "Engine Directory");
                    } else {
                        ui.colored_label(egui::Color32::GREEN, "Engine Directory");
                    }

                    if ui.text_edit_singleline(&mut self.engine_directory).changed() {
                        Oven::save_settings_value("engine_directory", self.engine_directory.clone());
                    }
                    ui.separator();

                    // Handle persistence of Project Directory
                    if !valid_project {
                        ui.colored_label(egui::Color32::RED, "Project Directory");
                    } else {
                        ui.colored_label(egui::Color32::GREEN, "Project Directory");
                    }

                    if ui.text_edit_singleline(&mut self.project_directory).changed() {
                        Oven::save_settings_value("project_directory", self.project_directory.clone());
                        self.update_modules();
                        self.content_module = 0;
                    }
                    ui.separator();

                    // Handle persistence of Content Directory
                    if self.content_modules.len() > 0 {
                        if egui::ComboBox::from_label("Modules")
                            .width(300.0)
                            .show_index(
                                ui,
                                &mut self.content_module,
                                self.content_modules.len(),
                                |i| { self.content_modules[i].to_owned() },
                            ).changed() {
                            Oven::save_settings_value("content_module", self.content_module.to_string());
                        }
                        ui.separator();
                    }

                    // Control cooking state with an atomic boolean shared between threads
                    if !self.is_cooking.load(Ordering::Relaxed) {
                        if !valid_project || !valid_engine {
                            ui.set_enabled(false);
                        }

                        let cook_button = ui.button("Cook");

                        if cook_button.clicked() {
                            let clear_bat = std::env::current_dir().unwrap().to_str().unwrap().to_string() + "\\scripts\\clear_cache.bat";
                            let cook_bat = std::env::current_dir().unwrap().to_str().unwrap().to_string() + "\\scripts\\cook_folder.bat";
                            let engine_directory = self.engine_directory.clone();
                            let project_directory = self.project_directory.clone();
                            let project_name = PathBuf::from(self.project_directory.clone()).file_name().unwrap().to_str().unwrap().to_string();
                            let content_directory = self.project_directory.clone() + "\\Content\\";
                            let content_name = self.content_modules[self.content_module].to_owned();
                            let pak_list = std::env::current_dir().unwrap().to_str().unwrap().to_string() + "\\scripts\\pak_list.txt";
                            let is_cooking = self.is_cooking.clone();
                            let log = self.log.clone();

                            self.prepare_project_config();
                            self.generate_pak_files_list(&project_name, &content_name);

                            // Launch the cook script in a separate thread
                            thread::spawn(move || {
                                is_cooking.store(true, Ordering::Relaxed);

                                let output = Command::new("cmd")
                                    .args(&[
                                        "/C",
                                        clear_bat.as_str(),
                                        project_directory.as_str(),
                                    ])
                                    //.stdout(Stdio::inherit())
                                    .output()
                                    .expect("failed to execute process");

                                if let Ok(mut log) = log.write() {
                                    log.push_str(&String::from_utf8_lossy(&output.stdout).to_string());
                                    println!("{}", log);
                                }

                                let output = Command::new("cmd")
                                    .args(&[
                                        "/C",
                                        cook_bat.as_str(),
                                        engine_directory.as_str(),
                                        project_directory.as_str(),
                                        project_name.as_str(),
                                        content_directory.as_str(),
                                        content_name.as_str(),
                                        pak_list.as_str()
                                    ])
                                    //.stdout(Stdio::inherit())
                                    .output()
                                    .expect("failed to execute process");

                                if let Ok(mut log) = log.write() {
                                    log.push_str(&String::from_utf8_lossy(&output.stdout).to_string());
                                    println!("{}", log);
                                }

                                is_cooking.store(false, Ordering::Relaxed);
                            });
                        }
                    } else {
                        // Show spinner while command is running
                        let spinner = egui::Spinner::new();
                        ui.add(spinner);
                    }

                    ui.separator();
                    ui.colored_label(egui::Color32::GREEN, "Log");
                    ScrollArea::new([true; 2])
                        .auto_shrink([false; 2])
                        .show(
                            ui,
                            |ui| {
                                if let Ok(log) = self.log.read() {
                                    for line in log.lines() {
                                        let line = Label::new(line)
                                            .wrap(false);
                                        ui.add(line);
                                    }
                                }
                            });
                })
            });
        None
    }
}

impl Oven {
    // Gather content folder module data and update the list
    fn update_modules(&mut self) {
        self.content_modules.clear();
        let content_directory = self.project_directory.clone() + "\\Content\\";

        if let Ok(content_directories) = fs::read_dir(content_directory) {
            for content_directory in content_directories {
                if let Ok(content_directory) = content_directory {
                    let folder_name = content_directory.path().file_name().unwrap().to_str().unwrap().to_string();
                    if folder_name.contains("Assets_") {
                        self.content_modules.push(folder_name);
                    }
                }
            }
        }
    }

    fn save_ini_value<'a>(file: &'a str, section: &'static str, key: &'static str, value: String) {
        if let Ok(mut config) = ini::Ini::load_from_file(file) {
            config.set_to(
                Some(section),
                key.to_string(),
                value,
            );
            config.write_to_file(file);
        }
    }

    // Save value into config.ini file
    fn save_settings_value(key: &'static str, value: String) {
        Oven::save_ini_value("config.ini", "settings", key, value);
    }

    // Prepare project ini for cooking
    fn prepare_project_config(&mut self) {
        let game_ini = self.project_directory.clone() + "\\Config\\DefaultGame.ini";
        Oven::save_ini_value(
            game_ini.as_str(),
            "/Script/UnrealEd.ProjectPackagingSettings",
            "bShareMaterialShaderCode",
            "False".to_string(),
        );
        let engine_ini = self.project_directory.clone() + "\\Config\\DefaultEngine.ini";
        Oven::save_ini_value(
            engine_ini.as_str(),
            "/Script/EngineSettings.GameMapsSettings",
            "GameDefaultMap",
            "None".to_string(),
        );
        Oven::save_ini_value(
            engine_ini.as_str(),
            "/Script/EngineSettings.GameMapsSettings",
            "EditorStartupMap",
            "None".to_string(),
        );
    }

    // Generate file to create a pak file
    fn generate_pak_files_list(&mut self, project_name: &String, content_name: &String) {
        let pak_list = std::env::current_dir().unwrap().to_str().unwrap().to_string() + "\\scripts\\pak_list.txt";
        let pak_files = self.project_directory.clone() + "\\Saved\\Cooked\\Android_ASTC\\" + project_name + "\\Content\\" + content_name + "\\*";
        if let Ok(mut file_list) = File::create(&pak_list) {
            write!(&mut file_list, "{}", &pak_files);
        }
    }
}

fn is_project_directory(path: &String, project_name: &String) -> bool {
    let uproject = path.clone() + "\\" + project_name + ".uproject";
    let content_folder = path.clone() + "\\Content\\";

    if !Path::new(&uproject).exists() {
        return false;
    }

    if !Path::new(&content_folder).exists() {
        return false;
    }

    true
}

fn is_engine_directory(path: &String) -> bool {
    let engine_executable = path.clone() + "\\Engine\\Binaries\\Win64\\UE4Editor.exe";
    let engine_pak_executable = path.clone() + "\\Engine\\Binaries\\Win64\\UnrealPak.exe";
    let engine_build_bat = path.clone() + "\\Engine\\Build\\BatchFiles\\RunUAT.bat";

    if !Path::new(&engine_executable).exists() {
        return false;
    }

    if !Path::new(&engine_pak_executable).exists() {
        return false;
    }

    if !Path::new(&engine_build_bat).exists() {
        return false;
    }

    true
}