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

#[derive(Debug, Default)]
pub struct MinimalApp {
	is_done:       bool,
	total_time:    f64,
	size:          Vector2,
	viewport_size: Vector2,
	scaling:       f32,
	renderer:      Option<Renderer>,
	system:        System,
	cursor_pos:    Vector2,
	egui_wrapper:  EguiWrapper,
	age:           i8,
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

		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::ColoredTextured as u16,
			"ColoredTextured",
			"coloredtextured_vs.glsl",
			"coloredtextured_fs.glsl",
		));
		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Colored as u16,
			"Colored",
			"colored_vs.glsl",
			"colored_fs.glsl",
		));

		self.renderer = Some(renderer);

		self.egui_wrapper.setup();
		self.egui_wrapper
			.set_effect_id(EffectId::ColoredTextured as u16);
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

		if wuc.is_escape_pressed {
			self.is_done = true;
		}

		self.scaling = 0.5; // abused for zoom

		self.viewport_size = wuc.window_size;

		self.size.x = (self.scaling) * self.viewport_size.x;
		self.size.y = (self.scaling) * self.viewport_size.y;

		self.cursor_pos.x = 0.5 * self.scaling * wuc.window_size.x * (2.0 * wuc.mouse_pos.x - 1.0);
		self.cursor_pos.y = 0.5 * self.scaling * wuc.window_size.y * (2.0 * wuc.mouse_pos.y - 1.0);

		if let Some(renderer) = &mut self.renderer {
			renderer.update(&mut self.system);
		}

		self.egui_wrapper.update(wuc);
		self.egui_wrapper.run(&mut self.system, |ctx| {
			ctx.set_visuals(egui::style::Visuals::light());
			ctx.set_visuals(egui::style::Visuals::dark());
			/*
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
					egui::FontId::new(10.0, egui::FontFamily::Proportional),
				),
			]
			.into();

			ctx.set_style(style);
			*/

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
				ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
				if ui.button("Quit?").clicked() {}

				ui.image(
					egui::epaint::TextureId::Managed(0),
					egui::Vec2 {
						x: 1024.0,
						y: 256.0,
					},
				);
			});

			ctx.set_visuals(egui::style::Visuals::light());
			egui::Window::new("My Window")
				.resizable(true)
				.show(ctx, |ui| {
					ui.label("Hello World!");
				});
			Ok(())
		});

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

			let scaling = 0.5;
			//				dbg!(&scaling);
			let left = -self.size.x * scaling;
			let right = self.size.x * scaling;
			let top = self.size.y * scaling;
			let bottom = -self.size.y * scaling;
			let near = 1.0;
			let far = -1.0;

			//				dbg!(&top,&bottom);

			let mvp = Matrix44::ortho(left, right, bottom, top, near, far);

			//				dbg!(&mvp);

			renderer.set_mvp_matrix(&mvp);

			renderer.use_effect(EffectId::Textured as u16);
			renderer.use_texture("cursor");
			renderer.render_textured_quad(&self.cursor_pos, &Vector2::new(128.0, 128.0));

			self.egui_wrapper.render(&mut self.system, renderer);
			renderer.end_frame();
		}
	}
}
