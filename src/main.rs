//! Interactive Resume

use eframe::egui;
use egui_dock::TabStyle;

// ------------------------------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let _ = eframe::run_native(
        "website",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some(egui::vec2(1280.0, 720.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Box::new(Resume::new(cc))),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                eframe::WebOptions::default(),
                Box::new(|cc| Box::new(Resume::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

// ------------------------------------------------------------------------------------------------

pub struct Resume {
    pages: egui_dock::DockState<Page>,
    viewer: PageViewer,
}

impl eframe::App for Resume {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top-bar").exact_height(41.0).show(ctx, |ui| {
            ui.visuals_mut().button_frame = false;
            ui.style_mut().spacing.item_spacing = egui::vec2(19.0, 3.0);
            ui.horizontal_centered(|ui| {
                if ui.button("Home").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    self.viewer.requests.push(Request::OpenPage(Page::Home));
                }
                if ui.button("Help").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    self.viewer.requests.push(Request::OpenPage(Page::Help));
                }
                if ui.button("Contact").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    self.viewer.requests.push(Request::OpenPage(Page::Contact));
                }
            });
        });
        egui_dock::DockArea::new(&mut self.pages)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.viewer);

        for req in self.viewer.requests.drain(..).collect::<Vec<_>>() {
            match req {
                Request::OpenPage(page) => {
                    if let Some(ids) = self.pages.find_tab(&page) {
                        self.pages.set_active_tab(ids);
                        self.viewer.highlight_page = Some(page);
                    } else {
                        self.pages.push_to_focused_leaf(page);
                    }
                }
            }
        }
        // ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}

impl Resume {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        use egui::{FontFamily::*, FontId, TextStyle};

        // let mut fonts = egui::FontDefinitions::default();
        // fonts.font_data.insert(
        //     "JetBrainsMono Regular".to_owned(),
        //     egui::FontData::from_static(include_bytes!("../res/JetBrainsMono_Regular.ttf"))
        // );
        // cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_visuals(egui::Visuals {
            interact_cursor: Some(egui::CursorIcon::PointingHand),
            ..Default::default()
        });
        cc.egui_ctx.set_style(egui::Style {
            text_styles: [
                (TextStyle::Small, FontId::new(13.0, Proportional)),
                (TextStyle::Body, FontId::new(17.0, Proportional)),
                (TextStyle::Button, FontId::new(17.0, Proportional)),
                (TextStyle::Heading, FontId::new(23.0, Proportional)),
                (TextStyle::Monospace, FontId::new(17.0, Monospace)),
            ].into(),
            ..Default::default()
        });
        
        let mut pages = egui_dock::DockState::new(vec![Page::Home]);

        // You can modify the tree before constructing the dock
        let [_a, b] = pages.main_surface_mut()
            .split_left(egui_dock::NodeIndex::root(), 0.3, vec![Page::Contact]);
        let [_a, _b] = pages.main_surface_mut()
            .split_below(b, 0.5, vec![Page::Help]);

        Self {
            pages,
            viewer: PageViewer {
                requests: vec![],
                highlight_page: Some(Page::Help),
            },
        }
    }
}

pub struct PageViewer {
    requests: Vec<Request>,
    highlight_page: Option<Page>,
}

impl egui_dock::TabViewer for PageViewer {
    type Tab = Page;
    
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Page::Home => "Home".into(),
            Page::Help => "Help".into(),
            Page::Contact => "Contact".into(),
            Page::Skills => "Skills".into(),
            Page::Portfolio => "Portfolio".into(),
            Page::Education => "Education".into(),
            Page::WorkHistory => "Work History".into(),
            Page::Goals => "Goals".into(),
        }
    }
    
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Page::Help => {
                ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 7.0);
                ui.weak("✱ Try clicking and dragging pages around!");
                ui.collapsing(egui::RichText::new("Navigation").heading(), |ui| {
                    ui.label("You can click and drag tab title bars to reorient them. Try to drag this page onto the Home page and take note of the popup that gives you the option to layout the pages in different ways.");
                    ui.label("There are links highlighted all across pages on this site. They can take you to new pages which will provide more information. Kind of like Wikipedia.")
                });
                ui.collapsing(egui::RichText::new("Site Overview").heading(), |ui| {
                    ui.label("This is a project to develop an interactive resume website for school (Florida State University) where I can show off my talents to potential employers.");
                    ui.label("This website is designed similarly to how I developed another project, a desktop application designed for organizing data/files. The idea is to present it similarly to how a desktop application designed for viewing/creating resumes would be presented.");
                });
            }

            Page::Home => {
                ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 7.0);
                ui.heading("Welcome!");
                ui.weak("This is the personal website of Matthew Norman.");
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(1.0, 0.0);
                    ui.label("If this is your first time visiting, I'd suggest you visit the ");
                    if ui.link("Help page").clicked() {
                        self.requests.push(Request::OpenPage(Page::Help));
                    }
                    ui.label(" for a quick guide on how this website works and a short overview of the design.");
                });
                ui.separator();
                ui.heading("Main Pages:");
                if ui.link("Skills").clicked() {
                    self.requests.push(Request::OpenPage(Page::Skills));
                }
                if ui.link("Portfolio").clicked() {
                    self.requests.push(Request::OpenPage(Page::Portfolio));
                }
                if ui.link("Education").clicked() {
                    self.requests.push(Request::OpenPage(Page::Education));
                }
                if ui.link("Work History").clicked() {
                    self.requests.push(Request::OpenPage(Page::WorkHistory));
                }
                if ui.link("Career Goals").clicked() {
                    self.requests.push(Request::OpenPage(Page::Goals));
                }
            }
            Page::Contact => {
                ui.style_mut().spacing.item_spacing = egui::vec2(7.0, 7.0);
                ui.heading("Matthew Norman");
                ui.separator();
                // ui.label("📞 (850) 555-5555");
                ui.label("📧 mmn23a@fsu.edu");
                ui.hyperlink_to(" GitHub", "https://www.github.com/mmnorm");
                ui.hyperlink_to(" LinkedIn", "https://www.linkedin.com/in/matthew-norman-67b10025a/");
            }
            Page::Skills => {
                ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 11.0);
                ui.horizontal(|ui| {
                    ui.visuals_mut().button_frame = false;
                    if ui.button("Home").clicked() {
                        self.requests.push(Request::OpenPage(Page::Home));
                    }
                    ui.weak(">");
                    ui.label("Skills");
                });
                ui.separator();
                ui.collapsing("Programming", |ui| {
                    ui.label("I am proficient in Rust, Python, and C. However, my primary language for the past couple years has been Rust. I would consider my programming skills to be well above average.");
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        ui.separator();
                        if ui.link("Portfolio").clicked() {
                            self.requests.push(Request::OpenPage(Page::Portfolio));
                        }
                    });
                }).header_response.on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.collapsing("System Administration", |ui| {
                    ui.label("I've been using Linux full time for about a year now. I have extensive experience in low-level system management and am extremely familiar with the terminal.");
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        ui.separator();
                        if ui.link("Work History").clicked() {
                            self.requests.push(Request::OpenPage(Page::WorkHistory));
                        }
                    });
                }).header_response.on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.collapsing("System Design", |ui| {
                    ui.label("I've developed and maintained several large-scale and small-scale projects that relied on effective information architecture and design. Organization is one of my strongest attributes. I am an excellent critical thinker capable of planning complex systems on my own.");
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        ui.separator();
                        if ui.link("Portfolio").clicked() {
                            self.requests.push(Request::OpenPage(Page::Portfolio));
                        }
                        ui.separator();
                        if ui.link("Work History").clicked() {
                            self.requests.push(Request::OpenPage(Page::WorkHistory));
                        }
                    });
                }).header_response.on_hover_cursor(egui::CursorIcon::PointingHand);
                egui::TopBottomPanel::bottom("skills-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Portfolio").clicked() {
                            self.requests.push(Request::OpenPage(Page::Portfolio));
                        }
                        ui.separator();
                        if ui.link("Education").clicked() {
                            self.requests.push(Request::OpenPage(Page::Education));
                        }
                    });
                });
            }
            Page::Education => {
                ui.horizontal(|ui| {
                    ui.visuals_mut().button_frame = false;
                    if ui.button("Home").clicked() {
                        self.requests.push(Request::OpenPage(Page::Home));
                    }
                    ui.weak(">");
                    ui.label("Education");
                });
                ui.separator();
                ui.heading("Florida State University");
                ui.horizontal(|ui| {
                    ui.label("Bachelor of Science in Information Technology");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Expected 2025");
                    });
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("GPA:");
                    ui.label("3.99");
                });
                egui::TopBottomPanel::bottom("education-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Work History").clicked() {
                            self.requests.push(Request::OpenPage(Page::WorkHistory));
                        }
                        ui.separator();
                        if ui.link("Portfolio").clicked() {
                            self.requests.push(Request::OpenPage(Page::Portfolio));
                        }
                    });
                });
            }
            Page::Portfolio => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Portfolio");
                });
                ui.separator();
                ui.heading("COMING SOON");
                ui.label("...");

                egui::TopBottomPanel::bottom("portfolio-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Skills").clicked() {
                            self.requests.push(Request::OpenPage(Page::Skills));
                        }
                        ui.separator();
                        if ui.link("Education").clicked() {
                            self.requests.push(Request::OpenPage(Page::Education));
                        }
                    });
                });
            }
            Page::WorkHistory => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Work History");
                });
                ui.separator();
                ui.heading("IT Intern");
                ui.horizontal(|ui| {
                    ui.label("Florida Fish and Wildlife Conservation Commission");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Spring 2022 - Fall 2023");
                    });
                });
                ui.heading("Duties:");
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Effectively maintained critical law enforcement IT infrastructure.");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Responded to IT-related concerns by internal staff.");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Acted and presented appropriately in a professional environment.");
                });

                egui::TopBottomPanel::bottom("work-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Education").clicked() {
                            self.requests.push(Request::OpenPage(Page::Education));
                        }
                    });
                });
            }
            Page::Goals => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Goals");
                });
                ui.separator();
                ui.heading("Career Goals");
                ui.label("My ultimate goal is independence. I know that may not be what a potential employer is looking for, but it's true. I'd like to eventually see myself working full time on my own projects with little to no oversight.");
                ui.label("I'm also genuinely interested in learning. I'd say that I am extremely self-motivated and capable of gaining an in-depth understanding of anything I find interesting. And just about anything an employer looking for an IT guy needs is something I find interesting.");

                ui.heading("Quick List");
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Master computing in all forms (networking, software development, hardware, infrastructure, etc.).");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Work on exciting projects related to computing that I genuinely believe in.");
                });

                egui::TopBottomPanel::bottom("goals-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Skills").clicked() {
                            self.requests.push(Request::OpenPage(Page::Skills));
                        }
                    });
                });
            }
        }
    }

    fn tab_style_override(&self, _tab: &Self::Tab, global_style: &TabStyle) -> Option<TabStyle> {
        let mut style = global_style.clone();
        style.tab_body.inner_margin = 13.0.into();
        Some(style)
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum Page {
    #[default]
    Home,
    Help,
    Contact,
    Skills,
    Portfolio,
    Education,
    WorkHistory,
    Goals,
}

// ------------------------------------------------------------------------------------------------

pub enum Request {
    OpenPage(Page),
}

// ------------------------------------------------------------------------------------------------
