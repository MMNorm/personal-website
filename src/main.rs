//! Interactive Resume

use eframe::egui;

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
        egui::TopBottomPanel::top("top-bar").exact_height(61.0).show(ctx, |ui| {
            ui.visuals_mut().button_frame = false;
            ui.horizontal_centered(|ui| {
                if ui.button("Home 🏠").clicked() {
                    self.viewer.requests.push(Request::OpenPage(Page::Home));
                }
                if ui.button("Help ❓").clicked() {
                    self.viewer.requests.push(Request::OpenPage(Page::Help));
                }
                if ui.button("Contact 📞").clicked() {
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
                ctx: cc.egui_ctx.clone(),
                requests: vec![],
                highlight_page: Some(Page::Help),
                highlight_id: egui::Id::new("page-highlighter"),
                unhighlight_id: egui::Id::new("page-unhighlighter"),
            },
        }
    }
}

pub struct PageViewer {
    ctx: egui::Context,
    requests: Vec<Request>,
    highlight_page: Option<Page>,
    highlight_id: egui::Id,
    unhighlight_id: egui::Id,
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
            Page::ProjectSwamp => "Swamp".into(),
            Page::ProjectHarbor => "Harbor".into(),
            Page::Education => "Education".into(),
            Page::FloridaState => "Florida State University".into(),
            Page::WorkHistory => "Work History".into(),
            Page::ITIntern => "IT Intern".into(),
            Page::SysAdmin2 => "Sys Admin II".into(),
            Page::Awards => "Awards".into(),
            Page::Goals => "Goals".into(),
            Page::FiveYearPlan => "5-Year Plan".into(),
        }
    }
    
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Page::Help => {
                ui.horizontal(|ui| {
                    ui.label("Help");
                });
                ui.separator();
                // if ui.link(egui::RichText::new("Navigation").heading()).clicked() {
                //     self.requests.push(Request::OpenPage(Page::HelpNavigation));
                // }
                ui.collapsing(egui::RichText::new("Navigation").heading(), |ui| {
                    ui.label("You can click and drag tab title bars to reorient them. Try to drag this page onto the Home page and take note of the popup that gives you the option to layout the pages in different ways.");
                    ui.horizontal_wrapped(|ui| {
                        ui.style_mut().spacing.item_spacing = egui::vec2(1.0, 0.0);
                        ui.label("This website is designed similarly to how I developed another project, ");
                        if ui.link("the Harbor Organizer").clicked() {
                            self.requests.push(Request::OpenPage(Page::ProjectHarbor));
                        }
                        ui.label(". The idea is to present it similarly to how a desktop application designed for viewing/creating resumes would be presented.");
                    });
                });
                // if ui.link(egui::RichText::new("Site Overview").heading()).clicked() {
                //     self.requests.push(Request::OpenPage(Page::HelpSiteOverview));
                // }
                ui.collapsing(egui::RichText::new("Site Overview").heading(), |ui| {
                    ui.label("This is a project to develop an interactive resume website for school (Florida State University) where I can show off my talents to potential employers.");
                });
                ui.separator();
            }

            Page::Home => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                });
                ui.separator();
                if ui.link("Help").clicked() {
                    self.requests.push(Request::OpenPage(Page::Help));
                }
                if ui.link("Contact").clicked() {
                    self.requests.push(Request::OpenPage(Page::Contact));
                }
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
                if ui.link("Awards & Achievements").clicked() {
                    self.requests.push(Request::OpenPage(Page::Awards));
                }
                if ui.link("Career Goals").clicked() {
                    self.requests.push(Request::OpenPage(Page::Goals));
                }
            }
            Page::Contact => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Contact");
                });
                ui.separator();
                ui.heading("Matthew Norman");
                ui.separator();
                ui.label("📞 (850) 555-5555");
                ui.label("📧 emailaddress@place.com");
                ui.hyperlink_to(" GitHub", "https://www.github.com/mmnorm");
                ui.hyperlink_to(" LinkedIn", "https://www.github.com/mmnorm");
            }
            Page::Skills => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Skills");
                });
                ui.separator();
                ui.collapsing("Programming", |ui| {
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Related:");
                        ui.separator();
                        if ui.link("Portfolio").clicked() {
                            self.requests.push(Request::OpenPage(Page::Portfolio));
                        }
                        ui.separator();
                        if ui.link("Swamp Desktop Environment").clicked() {
                            self.requests.push(Request::OpenPage(Page::ProjectSwamp));
                        }
                        ui.separator();
                        if ui.link("Harbor Organizer").clicked() {
                            self.requests.push(Request::OpenPage(Page::ProjectHarbor));
                        }
                    });
                });
                ui.collapsing("System Administration", |ui| {
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        ui.separator();
                        if ui.link("Work History").clicked() {
                            self.requests.push(Request::OpenPage(Page::WorkHistory));
                        }
                        ui.separator();
                        if ui.link("IT Internship").clicked() {
                            self.requests.push(Request::OpenPage(Page::ITIntern));
                        }
                        ui.separator();
                        if ui.link("System Administrator II").clicked() {
                            self.requests.push(Request::OpenPage(Page::SysAdmin2));
                        }
                    });
                });
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
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Education");
                });
                ui.separator();
                if ui.link(egui::RichText::new("Florida State University").heading()).clicked() {
                    self.requests.push(Request::OpenPage(Page::FloridaState));
                }
                ui.horizontal(|ui| {
                    ui.label("Bachelor of Science in Information Technology");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Expected 2025");
                    });
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
            Page::FloridaState => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Education");
                    ui.weak(">");
                    ui.label("Florida State University");
                });
                ui.separator();
                ui.heading("My Experience");
                ui.label("I don't think that I've had a");

                egui::TopBottomPanel::bottom("fsu-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Education").clicked() {
                            self.requests.push(Request::OpenPage(Page::Education));
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
                let harbor_label = egui::RichText::new("Harbor")
                    .heading();
                if ui.link(harbor_label).clicked() {
                    self.requests.push(Request::OpenPage(Page::ProjectHarbor));
                }
                ui.label("A data/file management program for the terminal.");
            }
            Page::ProjectSwamp => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Portfolio");
                    ui.weak(">");
                    ui.label("Harbor");
                });
                ui.separator();
                egui::TopBottomPanel::bottom("swamp-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Project Harbor").clicked() {
                            self.requests.push(Request::OpenPage(Page::ProjectHarbor));
                        }
                    });
                });
            }
            Page::ProjectHarbor => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Portfolio");
                    ui.weak(">");
                    ui.label("Harbor");
                });
                ui.separator();
                egui::TopBottomPanel::bottom("harbor-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Project Swamp").clicked() {
                            self.requests.push(Request::OpenPage(Page::ProjectSwamp));
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
            }
            Page::ITIntern => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Work History");
                    ui.weak(">");
                    ui.label("IT Intern");
                });
                ui.separator();
            }
            Page::SysAdmin2 => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Work History");
                    ui.weak(">");
                    ui.label("System Administrator II");
                });
                ui.separator();
            }
            Page::Awards => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Awards");
                });
                ui.separator();

                egui::TopBottomPanel::bottom("awards-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Skills").clicked() {
                            self.requests.push(Request::OpenPage(Page::Skills));
                        }
                        ui.separator();
                        if ui.link("Portfolio").clicked() {
                            self.requests.push(Request::OpenPage(Page::Portfolio));
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

                egui::TopBottomPanel::bottom("goals-bottom").show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Related:");
                        if ui.link("Five Year Plan").clicked() {
                            self.requests.push(Request::OpenPage(Page::FiveYearPlan));
                        }
                    });
                });
            }
            Page::FiveYearPlan => {
                ui.horizontal(|ui| {
                    ui.label("Home");
                    ui.weak(">");
                    ui.label("Goals");
                    ui.weak(">");
                    ui.label("Five Year Plan");
                });
                ui.separator();
                ui.heading("My 5-Year Plan");
            }
        }
    }

    // fn tab_style_override(&self, tab: &Self::Tab, global_style: &egui_dock::TabStyle) -> Option<egui_dock::TabStyle> {
    //     let mut style = global_style.clone();
    //     if self.highlight_page == Some(*tab) {
    //         let bg_color_alpha = self.ctx.animate_value_with_time(
    //             self.highlight_id, 
    //             111.0, 
    //             1.0,
    //         );
    //         style.tab_body.bg_fill = egui::Color32::from_white_alpha(bg_color_alpha as u8);
    //     } else {
    //         let factor = self.ctx.animate_value_with_time(
    //             self.unhighlight_id, 
    //             1.0, 
    //             1.0,
    //         );
    //         style.tab_body.bg_fill = global_style.tab_body.bg_fill.linear_multiply(factor);
    //     }
    //     Some(style)
    // }
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
    ProjectSwamp,
    ProjectHarbor,
    Education,
    FloridaState,
    WorkHistory,
    ITIntern,
    SysAdmin2,
    Awards,
    Goals,
    FiveYearPlan,
}

// ------------------------------------------------------------------------------------------------

pub enum Request {
    OpenPage(Page),
}

// ------------------------------------------------------------------------------------------------
