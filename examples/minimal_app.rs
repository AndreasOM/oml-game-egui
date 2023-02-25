use oml_game::math::Matrix44;
use oml_game::math::Vector2;
use oml_game::renderer::Color;
use oml_game::renderer::Effect;
use oml_game::renderer::Renderer;
use oml_game::system::filesystem_disk::FilesystemDisk;
use oml_game::system::filesystem_layered::FilesystemLayered;
use oml_game::system::System;
use oml_game::window::Window;
use oml_game::window::WindowUpdateContext;
use oml_game::App;
use oml_game_egui::EguiWrapper;

enum EffectId {
	Default         = 0,
	Textured        = 1,
	ColoredTextured = 2,
	Colored         = 3,
}

enum LayerId {
	Egui  = 1,
	Debug = 2,
}

#[derive(Debug, Default)]
pub struct MinimalApp {
	is_done:           bool,
	total_time:        f64,
	size:              Vector2,
	viewport_size:     Vector2,
	scaling:           f32,
	renderer:          Option<Renderer>,
	system:            System,
	cursor_pos:        Vector2,
	egui_wrapper:      EguiWrapper,
	font_size:         u16,
	use_blend_factors: bool,
	cull_face:         bool,
	frame_count:       usize,

	telemetry: oml_game_egui::EguiTelemetryWidget,
}

impl MinimalApp {
	fn add_filesystem_disk(&mut self, lfs: &mut FilesystemLayered, path: &str, enable_write: bool) {
		let datadir = if path.starts_with("/") {
			path.to_owned()
		} else {
			let cwd = std::env::current_dir().unwrap();
			let cwd = cwd.to_string_lossy();

			println!("CWD: {:?}", cwd);
			let datadir = format!("{}/{}", &cwd, &path);
			println!("datadir: {:?}", datadir);
			datadir
		};

		let mut dfs = FilesystemDisk::new(&datadir);
		if enable_write {
			dfs.enable_write();
		}
		lfs.add_filesystem(Box::new(dfs));
	}
}

impl App for MinimalApp {
	fn remember_window_layout(&self) -> bool {
		true
	}
	fn app_name(&self) -> &str {
		"oml-game-egui - example - minimal"
	}

	fn setup(&mut self, window: &mut Window) -> anyhow::Result<()> {
		window.set_title(self.app_name());

		let mut lfs = FilesystemLayered::new();
		self.add_filesystem_disk(&mut lfs, "./examples/data", false); // :TODO: fix path, relative to bin?
		self.system.set_default_filesystem(Box::new(lfs));

		let mut renderer = Renderer::new();
		renderer.setup(window, &mut self.system)?;

		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Default as u16,
			"Default",
			"default_vs.glsl",
			"default_fs.glsl",
		));

		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Textured as u16,
			"Textured",
			"textured_vs.glsl",
			"textured_fs.glsl",
		));

		renderer.register_effect(
			Effect::create(
				&mut self.system,
				EffectId::ColoredTextured as u16,
				"ColoredTextured",
				"coloredtextured_vs.glsl",
				"coloredtextured_fs.glsl",
			)
			.with_cull_face(false), //.with_blend_func( oml_game::renderer::BlendFactor::One, oml_game::renderer::BlendFactor::OneMinusSrcAlpha )
		);
		/*
		renderer.find_effect_mut_and_then( "ColoredTextured", |e| {
			e.set_blend_func( oml_game::renderer::BlendFactor::One, oml_game::renderer::BlendFactor::OneMinusSrcAlpha );
		});
		*/
		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Colored as u16,
			"Colored",
			"colored_vs.glsl",
			"colored_fs.glsl",
		));

		self.renderer = Some(renderer);

		let scale_factor = window.scale_factor() as f32;
		tracing::debug!("scale_factor {}", scale_factor);
		self.scaling = scale_factor;
		self.egui_wrapper.setup(scale_factor);
		self.egui_wrapper
			.set_effect_id(EffectId::ColoredTextured as u16);
		self.egui_wrapper.set_layer_id(LayerId::Egui as u8);

		self.font_size = 10;

		oml_game::DefaultTelemetry::enable();
		Ok(())
	}

	fn teardown(&mut self) {
		self.renderer = None;
	}

	fn is_done(&self) -> bool {
		//		println!("is_done {} <= 0", &self.count );
		self.is_done
	}

	fn update(&mut self, wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		self.total_time += wuc.time_step();
		self.frame_count += 1;
		oml_game::DefaultTelemetry::trace::<f32>("target_frame_time", (1.0 / 30.0) / 1.0);
		oml_game::DefaultTelemetry::trace::<f32>("time_step", (wuc.time_step() as f32) / 1.0);
		if self.frame_count % 100 < 50 {
			oml_game::DefaultTelemetry::trace::<f32>(
				"sin of frame_count",
				(1.0 / 60.0) * (self.frame_count as f32 * 0.01).sin() as f32,
			);
		}

		if wuc.is_escape_pressed {
			self.is_done = true;
		}

		//		self.scaling = 0.5; // abused for zoom
		//self.scaling = 1.0; //

		self.viewport_size = wuc.window_size;

		//		self.size.x = (self.scaling) * self.viewport_size.x;
		//		self.size.y = (self.scaling) * self.viewport_size.y;

		self.size.x = self.viewport_size.x;
		self.size.y = self.viewport_size.y;

		//		self.cursor_pos.x = 0.5 * self.scaling * wuc.window_size.x * (2.0 * wuc.mouse_pos.x - 1.0);
		//		self.cursor_pos.y = 0.5 * self.scaling * wuc.window_size.y * (2.0 * wuc.mouse_pos.y - 1.0);

		// self.cursor_pos.x = 0.5 * self.scaling * wuc.window_size.x * (wuc.mouse_pos.x - 1.0);
		// self.cursor_pos.y = 0.5 * self.scaling * wuc.window_size.y * (wuc.mouse_pos.y - 1.0);

		/*
		self.cursor_pos.x = wuc.mouse_pos.x * 0.5 * wuc.window_size.x - 0.25 * wuc.window_size.x;
		self.cursor_pos.y = wuc.mouse_pos.y * 0.5 * wuc.window_size.y - 0.25 * wuc.window_size.y;
		*/

		// a × (b + c)  =  a × b  +  a × c
		/*
		c = m * 0.5 * w - 0.25 * w
		c = w * ( 0.5 * m) + w * ( -0.25 )
		c = w * ( 0.5 * m - 0.25 )
		*/
		/*
		self.cursor_pos.x = wuc.window_size.x * (wuc.mouse_pos.x * 0.5 - 0.25);
		self.cursor_pos.y = wuc.window_size.y * (wuc.mouse_pos.y * 0.5 - 0.25);

		*/
		/*
		self.cursor_pos.x = 0.5 * (wuc.mouse_pos.x * wuc.window_size.x - 0.5 * wuc.window_size.x);
		self.cursor_pos.y = 0.5 * (wuc.mouse_pos.y * wuc.window_size.y - 0.5 * wuc.window_size.y);
		*/
		self.cursor_pos.x = 1.0 * (wuc.mouse_pos.x * wuc.window_size.x - 0.5 * wuc.window_size.x);
		self.cursor_pos.y = 1.0 * (wuc.mouse_pos.y * wuc.window_size.y - 0.5 * wuc.window_size.y);

		//tracing::debug!("cursor {}, {}", self.cursor_pos.x, self.cursor_pos.y);

		if let Some(renderer) = &mut self.renderer {
			renderer.update(&mut self.system);
		}

		self.egui_wrapper.update(wuc);
		self.egui_wrapper.run(&mut self.system, |ctx| {
			ctx.set_visuals(egui::style::Visuals::light());
			ctx.set_visuals(egui::style::Visuals::dark());

			if true {
				let mut style = (*ctx.style()).clone();
				style.override_text_style = Some(
					egui::TextStyle::Heading, //egui::FontId::new(30.0, egui::FontFamily::Proportional)
				);

				style.text_styles = [
					(
						egui::TextStyle::Heading,
						egui::FontId::new(30.0, egui::FontFamily::Proportional),
					),
					(
						egui::TextStyle::Name("Heading2".into()),
						egui::FontId::new(25.0, egui::FontFamily::Proportional),
					),
					(
						egui::TextStyle::Name("Context".into()),
						egui::FontId::new(23.0, egui::FontFamily::Proportional),
					),
					(
						egui::TextStyle::Body,
						egui::FontId::new(18.0, egui::FontFamily::Proportional),
					),
					(
						egui::TextStyle::Monospace,
						egui::FontId::new(14.0, egui::FontFamily::Proportional),
					),
					(
						egui::TextStyle::Button,
						egui::FontId::new(14.0, egui::FontFamily::Proportional),
					),
					(
						egui::TextStyle::Small,
						//						egui::FontId::new(10.0, egui::FontFamily::Proportional),
						egui::FontId::new(self.font_size as f32, egui::FontFamily::Proportional),
					),
				]
				.into();

				ctx.set_style(style);
			}

			egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
				// The top panel is often a good place for a menu bar:
				egui::menu::bar(ui, |ui| {
					ui.menu_button("File", |ui| {
						if ui.button("Quit").clicked() {
							self.is_done = true;
							// frame.quit();
						}
					});
				});
			});
			/*
			egui::SidePanel::left("my_side_panel").show(ctx, |ui| {
				if ui.button("Quit").clicked() {
					// frame.quit();
				}
			});
			*/

			egui::CentralPanel::default().show(ctx, |ui| {
				ui.heading("My egui Application");
				ui.heading("AAAAAAAAAAAAAAA");
				ui.heading("... is not working yet!");
				let r = ui.add(egui::Slider::new(&mut self.font_size, 10..=120).text("Font Size"));
				if r.dragged() {
					ctx.request_repaint();
				};
				ui.label(
					egui::RichText::new("Small Text")
						.text_style(egui::style::TextStyle::Small)
						.strong(),
				);
				if ui.button("Quit?").clicked() {}

				ui.checkbox(&mut self.use_blend_factors, "Blend Factors");
				ui.checkbox(&mut self.cull_face, "Cull Face");

				ui.image(
					egui::epaint::TextureId::Managed(0),
					egui::Vec2 {
						x: 1024.0,
						y: 256.0,
					},
				);
			});
			{
				egui::Window::new("Telemetry Widget")
					//.default_width(1000.0)
					.resize(|r| r.default_width(1000.0))
					.resizable(true)
					.show(ctx, |ui| {
						//ui.add(oml_game_egui::EguiTelemetryWidget::default());
						self.telemetry.show(ui);
					});
			}
			/*
			{
				egui::Window::new("Telemetry")
					.default_width(1000.0)
					//.resize(|r| r.default_width( 1000.0 ))
					//.resizable(true)
					.show(ctx, |ui| {
						use egui::plot::{Line, Plot, PlotPoints};

						let traces_info = oml_game::DefaultTelemetry::traces_info();
						//tracing::debug!("Traces: {:#?}", traces_info);
						let mut lines = Vec::new();
						let mut vlines = Vec::new();
						let maximum_length = oml_game::DefaultTelemetry::maximum_length() as f64;
						let frames = oml_game::DefaultTelemetry::frames() as f64;
						let gone_frames = (frames - maximum_length).max(0.0);
						let x_offset = gone_frames;
						{
							let v = oml_game::DefaultTelemetry::get::<f32>("time_step");
							let points: PlotPoints = v
								.iter()
								.enumerate()
								.filter_map(|(i, f)| {
									if let Some(f) = f {
										Some([i as f64 + x_offset, *f as f64])
									} else {
										None
									}
								})
								//.map(|(i, f)| [i as f64, *f as f64])
								.collect();
							let line = Line::new(points);
							lines.push(("time_step", line));
						}
						{
							let v = oml_game::DefaultTelemetry::get::<f32>("sin of frame_count");
							let mut gap = false;
							let points: PlotPoints = v
								.iter()
								.enumerate()
								.filter_map(|(i, f)| {
									let r = if let Some(f) = f {
										Some([i as f64 + x_offset, *f as f64])
									} else {
										None
									};
									let prev_gap = gap;
									gap = r.is_none();
									if gap != prev_gap {
										vlines.push(egui::widgets::plot::VLine::new(
											i as f64 + x_offset,
										));
									}
									r
								})
								.collect();
							let line = Line::new(points);
							lines.push(("sin of time_step", line));
						}
						Plot::new("time_step").view_aspect(2.0).show(ui, |plot_ui| {
							for (_name, line) in lines {
								plot_ui.line(line)
							}
							for vl in vlines {
								plot_ui.vline(vl);
							}
						});
					});
			}
			*/
			/*
			ctx.set_visuals(egui::style::Visuals::light());
			egui::Window::new("My Window")
				.resizable(true)
				.show(ctx, |ui| {
					ui.label("Hello World!");
				});
			*/
			Ok(())
		});

		oml_game::DefaultTelemetry::update();
		Ok(())
	}

	fn render(&mut self) {
		if let Some(renderer) = &mut self.renderer {
			renderer.set_size(&self.size);
			renderer.set_viewport(&Vector2::zero(), &self.viewport_size);
			renderer.begin_frame();
			let color = Color::from_rgba(
				0.5 + 0.5 * (self.total_time * 0.5).sin() as f32,
				0.5,
				0.5,
				1.0,
			);
			renderer.clear(&color);

			//let scaling = 0.5;
			//let scaling = 0.25;
			//let scaling = 1.0;
			//let scaling = self.scaling;
			//let scaling = 1.0 / self.scaling;
			let half_scaling = 0.5 / self.scaling;
			//				dbg!(&scaling);
			let left = -self.size.x * half_scaling;
			let right = self.size.x * half_scaling;
			let top = self.size.y * half_scaling;
			let bottom = -self.size.y * half_scaling;
			let near = 1.0;
			let far = -1.0;

			//tracing::debug!("x: {} - {}, y: {} - {}", left, right, bottom, top);

			//				dbg!(&top,&bottom);

			let mvp = Matrix44::ortho(left, right, bottom, top, near, far);

			//				dbg!(&mvp);

			renderer.set_mvp_matrix(&mvp);

			//renderer.use_effect(EffectId::Textured as u16);
			self.egui_wrapper.render(&mut self.system, renderer);

			renderer.use_layer(LayerId::Debug as u8);
			renderer.use_effect(EffectId::Textured as u16);
			renderer.use_texture("cursor");
			renderer.find_effect_mut_and_then("ColoredTextured", |e| {
				if self.use_blend_factors {
					e.set_blend_func(
						oml_game::renderer::BlendFactor::One,
						oml_game::renderer::BlendFactor::OneMinusSrcAlpha,
					);
				} else {
					e.set_blend_func(
						oml_game::renderer::BlendFactor::SrcAlpha,
						oml_game::renderer::BlendFactor::OneMinusSrcAlpha,
					);
				}
				e.set_cull_face(self.cull_face);
			});

			renderer.render_textured_quad(&self.cursor_pos, &Vector2::new(128.0, 128.0));
			renderer.end_frame();
		}
	}
}
