use std::collections::HashMap;
use std::sync::RwLock;

use egui::RawInput;
use oml_game::math::Matrix32;
use oml_game::math::Vector2;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::renderer::Texture;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

#[derive(Debug, Default)]
pub struct EguiWrapper {
	inner: RwLock<EguiWrapperInner>,
}

impl EguiWrapper {
	pub fn setup(&mut self, pixels_per_point: f32) -> anyhow::Result<()> {
		let mut inner = self.inner.write().unwrap();
		inner.setup(pixels_per_point)
	}

	pub fn set_color(&mut self, color: &Color) {
		let mut inner = self.inner.write().unwrap();
		inner.set_color(color);
	}

	pub fn toggle_input(&mut self) -> bool {
		let mut inner = self.inner.write().unwrap();
		inner.toggle_input()
	}

	pub fn enable_input(&mut self) {
		let mut inner = self.inner.write().unwrap();
		inner.enable_input();
	}

	pub fn disable_input(&mut self) {
		let mut inner = self.inner.write().unwrap();
		inner.disable_input();
	}

	pub fn set_input_disabled(&mut self, input_disabled: bool) {
		let mut inner = self.inner.write().unwrap();
		inner.set_input_disabled(input_disabled);
	}

	pub fn input_disabled(&self) -> bool {
		let mut inner = self.inner.read().unwrap();
		inner.input_disabled()
	}

	pub fn set_effect_id(&mut self, effect_id: u16) {
		let mut inner = self.inner.write().unwrap();
		inner.set_effect_id(effect_id);
	}
	pub fn set_layer_id(&mut self, layer_id: u8) {
		let mut inner = self.inner.write().unwrap();
		inner.set_layer_id(layer_id);
	}
	pub fn update(&mut self, wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		let mut inner = self.inner.write().unwrap();
		inner.update(wuc)
	}

	pub fn run<F>(&self, system: &mut System, mut f: F) -> anyhow::Result<()>
	where
		F: FnMut(&egui::Context) -> anyhow::Result<()>,
	{
		let mut inner = self.inner.write().unwrap();
		inner.run(system, f)
	}

	pub fn render(&mut self, system: &mut System, renderer: &mut Renderer) -> anyhow::Result<()> {
		let mut inner = self.inner.write().unwrap();
		inner.render(system, renderer)
	}
	fn gather_input(&mut self) -> RawInput {
		let mut inner = self.inner.write().unwrap();
		inner.gather_input()
	}

	fn paint(&mut self, system: &mut System, renderer: &mut Renderer) -> anyhow::Result<()> {
		let mut inner = self.inner.write().unwrap();
		inner.paint(system, renderer)
	}

	fn paint_mesh(&self, renderer: &mut Renderer, mesh: egui::epaint::Mesh) -> anyhow::Result<()> {
		let mut inner = self.inner.read().unwrap();
		inner.paint_mesh(renderer, mesh)
	}
}

#[derive(Debug, Default)]
pub struct EguiWrapperInner {
	egui_ctx: egui::Context,
	shapes: Vec<egui::epaint::ClippedShape>,
	textures_delta: egui::TexturesDelta,
	effect_id: u16,
	layer_id: u8,
	texture_ids: HashMap<egui::epaint::TextureId, u16>,
	size: Vector2,
	pixels_per_point: f32,
	events: Vec<egui::Event>,
	primary_mouse_button_was_pressed: bool,
	input_disabled: bool,
	color: Color,
}

impl EguiWrapperInner {
	pub fn setup(&mut self, pixels_per_point: f32) -> anyhow::Result<()> {
		self.pixels_per_point = pixels_per_point;
		Ok(())
	}

	pub fn set_color(&mut self, color: &Color) {
		self.color = *color;
	}

	pub fn toggle_input(&mut self) -> bool {
		self.input_disabled = !self.input_disabled;

		self.input_disabled
	}

	pub fn enable_input(&mut self) {
		self.input_disabled = false;
	}

	pub fn disable_input(&mut self) {
		self.input_disabled = true;
	}

	pub fn set_input_disabled(&mut self, input_disabled: bool) {
		self.input_disabled = input_disabled;
	}

	pub fn input_disabled(&self) -> bool {
		self.input_disabled
	}

	pub fn set_effect_id(&mut self, effect_id: u16) {
		self.effect_id = effect_id;
	}
	pub fn set_layer_id(&mut self, layer_id: u8) {
		self.layer_id = layer_id;
	}
	pub fn update(&mut self, wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		if !self.input_disabled {
			let mut cursor_pos = Vector2::zero();

			cursor_pos.x = 1.0 * (wuc.mouse_pos.x * wuc.window_size.x - 0.5 * wuc.window_size.x);
			cursor_pos.y = -1.0 * (wuc.mouse_pos.y * wuc.window_size.y - 0.5 * wuc.window_size.y);

			self.events.push(egui::Event::PointerMoved(egui::Pos2 {
				x: cursor_pos.x,
				y: cursor_pos.y,
			}));

			if wuc.was_mouse_button_pressed(0) {
				tracing::debug!("Primary Mouse Button Pressed @ {:?}", &cursor_pos);
				wuc.consume_mouse_button_pressed(0);
				self.events.push(egui::Event::PointerButton {
					pos:       egui::Pos2 {
						x: cursor_pos.x,
						y: cursor_pos.y,
					},
					button:    egui::PointerButton::Primary,
					pressed:   true,
					modifiers: egui::Modifiers::default(),
				});
				self.primary_mouse_button_was_pressed = true;
			} else if wuc.was_mouse_button_released(0) {
				//} else if self.primary_mouse_button_was_pressed {
				self.events.push(egui::Event::PointerButton {
					pos:       egui::Pos2 {
						x: cursor_pos.x,
						y: cursor_pos.y,
					},
					button:    egui::PointerButton::Primary,
					pressed:   false,
					modifiers: egui::Modifiers::default(),
				});
				self.primary_mouse_button_was_pressed = false;
			}
		}
		Ok(())
	}

	pub fn run<F>(&mut self, _system: &mut System, mut f: F) -> anyhow::Result<()>
	where
		F: FnMut(&egui::Context) -> anyhow::Result<()>,
	{
		let raw_input: egui::RawInput = self.gather_input();

		self.egui_ctx.begin_frame(raw_input);

		f(&self.egui_ctx).unwrap();

		let full_output = self.egui_ctx.end_frame();

		// tracing::debug!("{:?}", full_output.shapes);
		self.shapes = full_output.shapes;
		self.textures_delta.append(full_output.textures_delta);
		//tracing::debug!("{:?}", full_output.platform_output.cursor_icon);
		/*
				let platform_output = full_output.platform_output;
				my_integration.set_cursor_icon(platform_output.cursor_icon);
				if !platform_output.copied_text.is_empty() {
					my_integration.set_clipboard_text(platform_output.copied_text);
				}
		*/
		Ok(())
	}

	pub fn render(&mut self, system: &mut System, renderer: &mut Renderer) -> anyhow::Result<()> {
		self.size = *renderer.viewport_size();
		//tracing::debug!("Size {:?}", &self.size);
		self.paint(system, renderer)?;
		Ok(())
	}
	fn gather_input(&mut self) -> RawInput {
		//tracing::debug!("pixels_per_point {}", self.pixels_per_point);
		let screen_size_in_points = egui::Vec2 {
			x: self.size.x / self.pixels_per_point,
			y: self.size.y / self.pixels_per_point,
		};
		let ri = RawInput {
			//dropped_files: Vec::new(),
			//hovered_files: Vec::new(),
			//events: 0,
			//has_focus: 0,
			screen_rect: Some(egui::Rect::from_center_size(
				Default::default(),
				screen_size_in_points,
			)),
			pixels_per_point: Some(self.pixels_per_point),
			//			pixels_per_point: Some(self.pixels_per_point*2.0),
			events: self.events.drain(..).collect(),
			..Default::default()
		};
		//tracing::debug!("{:?}", ri.events);
		ri
	}

	fn update_texture_from_image(
		tex: &mut Texture,
		ox: usize,
		oy: usize,
		image: &egui::epaint::image::ImageData,
	) {
		match image {
			egui::epaint::image::ImageData::Color(color_image) => {
				todo!();
			},
			egui::epaint::image::ImageData::Font(font_image) => {
				let mut p = Vector2::zero();
				//let mut color = 0xffffffff;
				for y in 0..font_image.size[1] {
					p.y = (oy + y) as f32;
					for x in 0..font_image.size[0] {
						p.x = (ox + x) as f32;
						let coverage = font_image.pixels[y * font_image.size[0] + x];
						let coverage = (coverage * 255.0) as u8;
						let color = (coverage as u32) * 0x01010101;
						tex.set_texel(&p, color);
					}
				}
			},
		};
	}

	fn paint(&mut self, system: &mut System, renderer: &mut Renderer) -> anyhow::Result<()> {
		let shapes = std::mem::take(&mut self.shapes);
		let mut textures_delta = std::mem::take(&mut self.textures_delta);

		for (id, image_delta) in &textures_delta.set {
			if let Some(pos) = &image_delta.pos {
				// update existing texture
				let name = match id {
					egui::epaint::TextureId::Managed(mid) => {
						format!("egui-{}", mid)
					},
					egui::epaint::TextureId::User(_uid) => {
						todo!();
					},
				};

				renderer.find_texture_mut_and_then(&name, |tex| {
					EguiWrapperInner::update_texture_from_image(
						tex,
						pos[0],
						pos[1],
						&image_delta.image,
					);
					//tex.update_canvas();
					tex.queue_canvas_update();
				});
			} else {
				// create new texture
				let size = &image_delta.image.size();
				let size = if size[0] > size[1] { size[0] } else { size[1] };
				let name = match id {
					egui::epaint::TextureId::Managed(mid) => {
						format!("egui-{}", mid)
					},
					egui::epaint::TextureId::User(_uid) => {
						todo!();
					},
				};
				let mut tex = Texture::create_canvas(&name, size as u32);
				let sy = image_delta.image.size()[1] as f32 / image_delta.image.size()[0] as f32;
				let mtx = Matrix32::identity().with_scaling_xy(1.0, sy);
				tex.set_mtx(&mtx);

				EguiWrapperInner::update_texture_from_image(&mut tex, 0, 0, &image_delta.image);

				tex.update_canvas();
				let tid = renderer.register_texture(tex);
				self.texture_ids.insert(*id, tid);
			}
		}

		let clipped_primitives = self.egui_ctx.tessellate(shapes);
		//tracing::debug!("{:?}", &clipped_primitives);

		renderer.use_layer(self.layer_id);
		renderer.use_effect(self.effect_id);

		for egui::ClippedPrimitive {
			clip_rect: _,
			primitive,
		} in clipped_primitives
		{
			//tracing::debug!("ClipRect: {:?}", clip_rect);
			match primitive {
				egui::epaint::Primitive::Mesh(mesh) => {
					//tracing::debug!("Mesh: {:?}", &mesh );
					self.paint_mesh(renderer, mesh)?;
				},
				p => {
					tracing::warn!("Unsupported primitive {:?}", &p);
				},
			};
		}

		Ok(())
	}

	fn paint_mesh(&self, renderer: &mut Renderer, mesh: egui::epaint::Mesh) -> anyhow::Result<()> {
		let mut vertice_map = HashMap::new();

		//let size = renderer.size();
		//let aspect_ratio = renderer.aspect_ratio();

		//tracing::debug!("Size: {:?}", &size );
		//tracing::debug!("Aspect Ratio: {:?}", &aspect_ratio );

		let texture_id = &mesh.texture_id;
		let tid = match self.texture_ids.get(texture_id) {
			Some(tid) => tid,
			None => return Ok(()),
		};
		// tracing::debug!("Using texture {}", tid);
		renderer.use_texture_id_in_channel(*tid, 0);
		//		renderer.render_textured_fullscreen_quad();

		for (i, v) in mesh.vertices.iter().enumerate() {
			/*
			gl_Position = vec4(
			2.0 * a_pos.x / u_screen_size.x - 1.0,
			1.0 - 2.0 * a_pos.y / u_screen_size.y,
			0.0,
			1.0);
			*/
			let vertex = Vector2::new( v.pos.x, v.pos.y )
			.scaled_vector2( &Vector2::new( 1.0, -1.0 ) ) // upside down :(
			//.scaled( 4.0 )
			//.add( &Vector2::new( 0.0, 0.0 ) )
			//.add( &Vector2::new( 5.0*500.0, 4.0*-500.0 ) )
			;

			//tracing::debug!("TC {}, {}", v.uv.x, v.uv.y);
			renderer.set_tex_coords(&Vector2::new(v.uv.x, v.uv.y /*/8.0*/));
			let color = oml_game::renderer::Color::from_rgba(
				v.color.r() as f32 / 255.0,
				v.color.g() as f32 / 255.0,
				v.color.b() as f32 / 255.0,
				v.color.a() as f32 / 255.0,
			);

			let color = color * self.color;

			renderer.set_color(&color);
			let vi = renderer.add_vertex(&vertex);
			vertice_map.insert(i, vi);
			//tracing::debug!("{} -> {}, {:?}", i, vi, v.pos );
		}

		for t in mesh.indices.chunks(3) {
			let tm: Vec<u32> = t
				.iter()
				.map(|i| *vertice_map.get(&(*i as usize)).unwrap())
				.collect();
			renderer.add_triangle(tm[0], tm[1], tm[2]);
		}
		Ok(())
	}
}
