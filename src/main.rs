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
                    self.viewer.state.requests.push(Request::OpenPage(Page::Home));
                }
                if ui.button("Help").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    self.viewer.state.requests.push(Request::OpenPage(Page::Help));
                }
                if ui.button("Contact").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    self.viewer.state.requests.push(Request::OpenPage(Page::Contact));
                }
            });
        });
        egui_dock::DockArea::new(&mut self.pages)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.viewer);

        for req in self.viewer.state.requests.drain(..).collect::<Vec<_>>() {
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
        egui_extras::install_image_loaders(&cc.egui_ctx);
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
                state: State {
                    requests: vec![],
                },
                highlight_page: Some(Page::Help),
            },
        }
    }
}

pub struct State {
    requests: Vec<Request>,
}

pub struct PageViewer {
    state: State,
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
            Page::Experience => "Experience".into(),
            Page::Portfolio => "Portfolio".into(),
            Page::Education => "Education".into(),
            Page::WorkHistory => "Work History".into(),
            Page::Goals => "Goals".into(),

            Page::Project1 => "Markdown Editor".into(),
            Page::Project2 => "Modular Programming Model".into(),
            Page::Project3 => "EEV Data Model".into(),
        }
    }
    
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Page::Home => home_page(ui, &mut self.state),
            Page::Help => help_page(ui, &mut self.state),
            Page::Contact => contact_page(ui, &mut self.state),

            Page::Skills => skills_page(ui, &mut self.state),
            Page::Experience => experience_page(ui, &mut self.state),
            Page::Education => education_page(ui, &mut self.state),
            Page::Portfolio => portfolio_page(ui, &mut self.state),
            Page::WorkHistory => work_history_page(ui, &mut self.state),
            Page::Goals => goals_page(ui, &mut self.state),

            Page::Project1 => project_1_page(ui, &mut self.state),
            Page::Project2 => project_2_page(ui, &mut self.state),
            Page::Project3 => project_3_page(ui, &mut self.state),
        }
    }

    fn tab_style_override(&self, _tab: &Self::Tab, global_style: &TabStyle) -> Option<TabStyle> {
        let mut style = global_style.clone();
        style.tab_body.inner_margin = 13.0.into();
        Some(style)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    Home,
    Help,
    Contact,

    Skills,
    Experience,
    Portfolio,
    WorkHistory,
    Education,

    Goals,

    Project1,
    Project2,
    Project3,
}

pub enum Request {
    OpenPage(Page),
}

// ------------------------------------------------------------------------------------------------

fn home_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "home",
            |ui, state| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 7.0);
                        ui.heading("Welcome!");
                        ui.weak("This is the personal website of Matthew Norman.");
                        ui.strong("The blue text pieces are internal links to pages, and you can click and drag tabs to rearrange them.");
                        ui.horizontal_wrapped(|ui| {
                            ui.style_mut().spacing.item_spacing = egui::vec2(1.0, 0.0);
                            ui.label("If this is your first time visiting, I'd suggest you visit the ");
                            if ui.link("Help page").clicked() {
                                state.requests.push(Request::OpenPage(Page::Help));
                            }
                            ui.label(" for a quick guide on how this website works and a short overview of the design.");
                        });
                        ui.separator();
                        ui.heading("Explore");
                        ui.add_space(19.0);
                        tree_item(ui, state, Page::Help, "Help", 0);
                        // tree_item(ui, state, Page::HelpNavigation, "Navigation Guide", 1);
                        tree_item(ui, state, Page::Contact, "Contact", 0);
                        tree_item(ui, state, Page::Skills, "Skills", 0);
                        tree_item(ui, state, Page::Experience, "Experience", 0);
                        tree_item(ui, state, Page::Portfolio, "Portfolio", 1);
                        tree_item(ui, state, Page::WorkHistory, "Work History", 1);
                        // tree_item(ui, state, Page::WorkHistory, "IT Intern", 2);
                        tree_item(ui, state, Page::Education, "Education", 0);
                        tree_item(ui, state, Page::Goals, "Goals", 0);
                        ui.add_space(19.0);
                        ui.weak("Last Updated: 6/20/2024")
                    });
            },
            |ui, _state| {
                ui.heading("Home");
            },
            |ui, state| {
                if ui.link("Help").clicked() {
                    state.requests.push(Request::OpenPage(Page::Help));
                }
                if ui.link("Contact").clicked() {
                    state.requests.push(Request::OpenPage(Page::Contact));
                }
            },
    );
}

fn help_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "help",
            |ui, _state| {
                ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 7.0);
                ui.weak("✱ Try clicking and dragging pages around!");
                ui.add_space(19.0);
                ui.group(|ui| {
                    ui.heading("Navigation");
                    ui.separator();
                    ui.label("You can click and drag tab title bars to reorient them. Try to drag this page onto the Home page and take note of the popup that gives you the option to layout the pages in different ways.");
                    ui.label("There are links highlighted all across pages on this site. They can take you to new pages which will provide more information. Kind of like Wikipedia.");
                });
                ui.add_space(19.0);
                ui.group(|ui| {
                    ui.heading("Site Overview");
                    ui.separator();
                    ui.label("This is a project to develop an interactive resume website for school (Florida State University) where I can show off my talents to potential employers.");
                    ui.label("This website is designed similarly to how I developed another project, a desktop application designed for organizing data/files. The idea is to present it similarly to how a desktop application designed for viewing/creating resumes would be presented.");
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Help");
            },
            |ui, state| {
                if ui.link("Home").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                if ui.link("Contact").clicked() {
                    state.requests.push(Request::OpenPage(Page::Contact));
                }
            },
    );
}

fn contact_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "contact",
            |ui, _state| {
                ui.style_mut().spacing.item_spacing = egui::vec2(7.0, 7.0);
                ui.heading("Matthew Norman");
                ui.separator();
                // ui.label("📞 (850) 555-5555");
                ui.horizontal_wrapped(|ui| {
                    ui.strong("Email:");
                    ui.label("mmn23a@fsu.edu");
                });
                ui.hyperlink_to(" GitHub", "https://www.github.com/mmnorm");
                ui.hyperlink_to(" LinkedIn", "https://www.linkedin.com/in/matthew-norman-67b10025a/");
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Contact Information");
            },
            |ui, state| {
                if ui.link("Help").clicked() {
                    state.requests.push(Request::OpenPage(Page::Help));
                }
            },
    );
}

fn skills_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "skills",
            |ui, _state| {
                ui.group(|ui| {
                    ui.heading("Programming");
                    ui.separator();
                    ui.label("I am proficient in Rust, Python, and C. However, my primary language for the past couple years has been Rust. In fact, this whole website is written in Rust and complied to Web Assembly. I would consider my programming skills to be well above average for my age/peers.");
                });
                ui.add_space(19.0);
                ui.group(|ui| {
                    ui.heading("System Administration");
                    ui.separator();
                    ui.label("I've been using Linux full-time for about a year now. I have extensive experience in low-level system administration and am extremely familiar with a terminal environment. I'd say I'm more comfortable working with Linux than I am with Windows now.");
                });
                ui.add_space(19.0);
                ui.group(|ui| {
                    ui.heading("System Design");
                    ui.separator();
                    ui.label("I've developed and maintained several large-scale and small-scale projects that rely on effective information architecture and design. Organization is one of my strongest attributes. I am an excellent critical thinker and capable of planning complex systems without oversight.");
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Skills");
            },
            |ui, state| {
                if ui.link("Experience").clicked() {
                    state.requests.push(Request::OpenPage(Page::Experience));
                }
            },
    );
}

fn experience_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "experience",
            |ui, state| {
                ui.horizontal_wrapped(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                    ui.label("I would divide my current level of experience into two categories: ");
                    if ui.link("my personal/side projects").clicked() {
                        state.requests.push(Request::OpenPage(Page::Portfolio));
                    }
                    ui.label(", and ");
                    if ui.link("my employment experience").clicked() {
                        state.requests.push(Request::OpenPage(Page::WorkHistory));
                    }
                    ui.label(".");
                });
                ui.add_space(19.0);
                ui.group(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                    let heading = egui::RichText::new("Personal Projects").heading();
                    if ui.link(heading).clicked() {
                        state.requests.push(Request::OpenPage(Page::WorkHistory));
                    }
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.label("As mentioned in ");
                        if ui.link("my Goals Page").clicked() {
                            state.requests.push(Request::OpenPage(Page::Goals));
                        }
                        ui.label(", I am quite independently motivated. My personal projects hep feed my passion for computing in all forms and I'm quite proud of my choice to use my free time toward them these past few years.");
                    });
                });
                ui.add_space(19.0);
                ui.group(|ui| {
                    let heading = egui::RichText::new("Employment History").heading();
                    if ui.link(heading).clicked() {
                        state.requests.push(Request::OpenPage(Page::WorkHistory));
                    }
                    ui.separator();
                    ui.label("Nothing special here, just real-world experience that I have cited in this resume.");
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Experience");
            },
            |ui, state| {
                if ui.link("Skills").clicked() {
                    state.requests.push(Request::OpenPage(Page::Skills));
                }
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
                if ui.link("Work History").clicked() {
                    state.requests.push(Request::OpenPage(Page::WorkHistory));
                }
            },
    );
}

fn portfolio_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "portfolio",
            |ui, state| {
                page_object(ui, state, Some(Page::Project1), "Markdown Editor", |ui, _state| {
                    ui.label("A markdown editor in the same vein as modern note-taking applications.");
                    ui.weak("Click `Learn More` for details");
                });
                ui.add_space(19.0);
                page_object(ui, state, Some(Page::Project2), "Modular Programming Model", |ui, _state| {
                    ui.label("A programming model for building extremely scalable applications without oversight.");
                    ui.weak("Click `Learn More` for details");
                });
                // ui.add_space(19.0);
                // page_object(ui, state, Some(Page::Project3), "EEV Data Model", |ui, _state| {
                //     ui.weak("Click `Learn More` for details");
                // });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Portfolio");
            },
            |ui, state| {
                if ui.link("Experience").clicked() {
                    state.requests.push(Request::OpenPage(Page::Experience));
                }
                if ui.link("Work History").clicked() {
                    state.requests.push(Request::OpenPage(Page::WorkHistory));
                }
            },
    );
}

fn project_1_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "project1",
            |ui, _state| {
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Role:");
                    ui.label("Developer; the sole programmer, designer, and maintainer of the project.");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Status:");
                    ui.colored_label(egui::Color32::LIGHT_GREEN, "IN PROGRESS");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Overview:");
                    ui.label("A simple markdown editor primarily designed for usage in Linux. It was built to run very quickly, even on older computers. The design is very similar to some popular note-taking applications with links, preview support, and customization capability.");
                });
                ui.add_space(19.0);
                ui.heading("Showcase:");
                ui.vertical_centered(|ui| {
                    ui.add(egui::Image::new(
                        "https://raw.githubusercontent.com/MMNorm/personal-website/master/assets/p1_a.png"
                    )
                        .rounding(11.0))
                        .on_hover_text("Markdown editor preview mode showcase");
                    ui.weak("Editor preview mode example.");
                    ui.hyperlink_to("Click here to view a full-size version of the image", "https://raw.githubusercontent.com/MMNorm/personal-website/master/assets/p1_a.png");
                    // ui.add(egui::Image::new(
                    //     "https://raw.githubusercontent.com/MMNorm/personal-website/master/assets/p1_b.png"
                    // )
                    //     .rounding(11.0))
                    //     .on_hover_text("Markdown editor editing mode showcase");
                    // ui.weak("Editor editing mode example.");
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Markdown Editor");
            },
            |ui, state| {
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
                if ui.link("Skills").clicked() {
                    state.requests.push(Request::OpenPage(Page::Skills));
                }
            },
    );
}

fn project_2_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "project2",
            |ui, _state| {
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Role:");
                    ui.label("Designer; the model's creator.");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Status:");
                    ui.colored_label(egui::Color32::YELLOW, "UNDER REVIEW");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Overview:");
                    ui.label("A set of guidelines for program development. The goal was to design a heuristic for developing programs that are small in size, but extremely modular.");
                });
                ui.add_space(19.0);
                ui.heading("Showcase:");
                ui.vertical_centered(|ui| {
                    ui.add(egui::Image::new(
                        "https://raw.githubusercontent.com/MMNorm/personal-website/master/assets/p2_a.png"
                    )
                        .rounding(11.0))
                        .on_hover_text("Chart for the program development model");
                    ui.weak("Program development model chart.");
                    ui.hyperlink_to("Click here to view a full-size version of the image", "https://raw.githubusercontent.com/MMNorm/personal-website/master/assets/p2_a.png");
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Modular Programming Model");
            },
            |ui, state| {
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
                if ui.link("Skills").clicked() {
                    state.requests.push(Request::OpenPage(Page::Skills));
                }
            },
    );
}

fn project_3_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "project3",
            |ui, _state| {
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Role:");
                    ui.label("Creator");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Status:");
                    ui.colored_label(egui::Color32::YELLOW, "UNDER REVIEW");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Overview:");
                    ui.label("A data model designed to mitigate the limitations of using heterogeneous data with the traditional EAV data model.");
                });
                ui.add_space(19.0);
                ui.heading("COMING SOON");
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("EEV Data Model");
            },
            |ui, state| {
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
                if ui.link("Skills").clicked() {
                    state.requests.push(Request::OpenPage(Page::Skills));
                }
            },
    );
}

fn work_history_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "work-history",
            |ui, _state| {
                ui.group(|ui| {
                    ui.heading("IT Intern");
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Florida Fish and Wildlife Conservation Commission (FWC)");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Spring 2022 - Fall 2023");
                        });
                    });
                    ui.add_space(13.0);
                    ui.heading("Duties");
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("◾");
                        ui.label("Effectively maintained critical law enforcement IT infrastructure.");
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("◾");
                        ui.label("Responded to internal IT-related concerns by staff.");
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("◾");
                        ui.label("Acted and presented appropriately in a professional environment.");
                    });
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Work History");
            },
            |ui, state| {
                if ui.link("Experience").clicked() {
                    state.requests.push(Request::OpenPage(Page::Experience));
                }
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
            },
    );
}

fn education_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "education",
            |ui, _state| {
                ui.group(|ui| {
                    ui.heading("Florida State University");
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Bachelor of Science, Information Technology");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Expected 2025");
                        });
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("GPA:");
                        ui.label("3.99");
                    });
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Education");
            },
            |ui, state| {
                if ui.link("Experience").clicked() {
                    state.requests.push(Request::OpenPage(Page::Experience));
                }
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
            },
    );
}

fn goals_page(ui: &mut egui::Ui, state: &mut State) {
    page_ui(ui, state, "goals",
            |ui, _state| {
                ui.heading("Career Goals");
                ui.label("My ultimate goal is independence. I know that may not be what a potential employer is looking for, but it's true. I'd like to eventually see myself working full time on my own projects with little to no oversight.");
                ui.label("I'm also genuinely interested in learning. I'd say that I am extremely self-motivated and capable of gaining an in-depth understanding of anything I find interesting. And just about anything an employer looking for an IT guy needs is something I find interesting.");
                ui.add_space(19.0);
                ui.heading("Quick List");
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Master computing in all forms (networking, software development, hardware, infrastructure, etc.).");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.weak("◾");
                    ui.label("Work on exciting projects related to computing that I genuinely believe in.");
                });
            },
            |ui, state| {
                ui.visuals_mut().button_frame = false;
                if ui.button("🏠").clicked() {
                    state.requests.push(Request::OpenPage(Page::Home));
                }
                ui.heading("Goals");
            },
            |ui, state| {
                if ui.link("Skills").clicked() {
                    state.requests.push(Request::OpenPage(Page::Skills));
                }
                if ui.link("Portfolio").clicked() {
                    state.requests.push(Request::OpenPage(Page::Portfolio));
                }
            },
    );
}

// ------------------------------------------------------------------------------------------------

fn page_ui(
    ui: &mut egui::Ui,
    state: &mut State,
    name: &str,
    content: impl FnOnce(&mut egui::Ui, &mut State),
    header: impl FnOnce(&mut egui::Ui, &mut State),
    footer: impl FnOnce(&mut egui::Ui, &mut State),
) {
    egui::TopBottomPanel::top(format!("{name}-header"))
        .exact_height(43.0)
        .show_inside(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                header(ui, state)
            });
        });
    egui::TopBottomPanel::bottom(format!("{name}-footer"))
        .exact_height(61.0)
        .show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Related:");
                footer(ui, state)
            });
        });
    egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(&ui.ctx().style())
            .inner_margin(13.0))
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                content(ui, state)
            });
        });
}

fn tree_item(ui: &mut egui::Ui, state: &mut State, page: Page, title: &str, level: usize) {
    ui.add_sized(egui::vec2(ui.available_width(), 29.0), |ui: &mut egui::Ui| {
        ui.horizontal_centered(|ui| {
            for _ in 0..level {
                ui.separator();
                ui.add_space(ui.spacing().indent);
            }
            ui.label("▪");
            if ui.link(title).clicked() {
                state.requests.push(Request::OpenPage(page));
            }
        }).response
    });
}

fn page_object(
    ui: &mut egui::Ui,
    state: &mut State,
    page: Option<Page>,
    title: &str,
    content: impl FnOnce(&mut egui::Ui, &mut State),
) {
    ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.visuals_mut().button_frame = false;
            ui.heading(title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let resp = ui.add_enabled(page.is_some(), egui::Link::new("Learn More"));
                if resp.clicked() {
                    if let Some(page) = page {
                        state.requests.push(Request::OpenPage(page));
                    }
                }
            });
        });
        ui.separator();
        content(ui, state)
    });
}

// ------------------------------------------------------------------------------------------------
