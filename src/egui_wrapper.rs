use std::collections::HashMap;

use egui::RawInput;
use oml_game::math::Matrix32;
use oml_game::math::Vector2;
use oml_game::renderer::Renderer;
use oml_game::renderer::Texture;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

#[derive(Debug, Default)]
pub struct EguiWrapper {
	egui_ctx:       egui::Context,
	shapes:         Vec<egui::epaint::ClippedShape>,
	textures_delta: egui::TexturesDelta,
	effect_id:      u16,
	aspect_ratio:   f32,
	texture_ids:    HashMap<egui::epaint::TextureId, u16>,
	size:			Vector2,
	pixels_per_point: f32,
}

impl EguiWrapper {
	pub fn setup(&mut self) -> anyhow::Result<()> {
		self.pixels_per_point = 1.0;
		Ok(())
	}

	pub fn set_effect_id(&mut self, effect_id: u16) {
		self.effect_id = effect_id;
	}
	pub fn update(&mut self, wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		Ok(())
	}

	pub fn run<F>(&mut self, _system: &mut System, mut f: F) -> anyhow::Result<()>
	where
		F: FnMut(&egui::Context) -> anyhow::Result<()>,
	{
		let mut raw_input: egui::RawInput = self.gather_input();

		//state.input.screen_rect = Some(painter.screen_rect);
		/*
		raw_input.screen_rect = Some(egui::Rect {
			min: egui::Pos2 {
				x: -512.0 * self.aspect_ratio,
				y: -512.0,
			},
			max: egui::Pos2 {
				x: 512.0 * self.aspect_ratio,
				y: 512.0,
			},
		});
		*/

		//self.egui_ctx.begin_frame(...);
		/*
		egui::CentralPanel::default().show(&egui_ctx, |ui| {
		});
		*/
		//self.egui_ctx.end_frame();

		let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
			f(&egui_ctx).unwrap();
			//my_app.ui(egui_ctx); // add panels, windows and widgets to `egui_ctx` here
		});

		tracing::debug!("{:?}", full_output.shapes);
		self.shapes = full_output.shapes;
		self.textures_delta.append(full_output.textures_delta);
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
		//self.size = *renderer.size();
		/*
		let aspect_ratio = renderer.aspect_ratio();
		self.aspect_ratio = aspect_ratio;
		self.size.y = 1024.0; // fixed height mode
		self.size.x = self.size.y * aspect_ratio;
		*/
		self.size = *renderer.size();
		//tracing::debug!("Size {:?}", &self.size );
		self.paint(system, renderer)?;
		Ok(())
	}
	fn gather_input(&mut self) -> RawInput {
		tracing::debug!("pixels_per_point {}", self.pixels_per_point);
		let screen_size_in_points = egui::Vec2 {
			x: self.size.x / self.pixels_per_point,
			y: self.size.y / self.pixels_per_point,
		};
		RawInput {
			//dropped_files: Vec::new(),
			//hovered_files: Vec::new(),
			//events: 0,
			//has_focus: 0,
			screen_rect: Some(egui::Rect::from_center_size(Default::default(), screen_size_in_points)),
			pixels_per_point: Some(self.pixels_per_point),
			..Default::default()
		}
	}
	fn paint(&mut self, system: &mut System, renderer: &mut Renderer) -> anyhow::Result<()> {
		let shapes = std::mem::take(&mut self.shapes);
		let mut textures_delta = std::mem::take(&mut self.textures_delta);

		for (id, image_delta) in &textures_delta.set {
			//tracing::debug!("{:?}, {:?}", id, image_delta.pos);
			// self.set_texture(display, *id, image_delta);
			if let Some(pos) = &image_delta.pos {
				todo!();
			} else {
				match &image_delta.image {
					egui::epaint::image::ImageData::Color(color_image) => {
						let size = &color_image.size;
						tracing::debug!("New ImageData::Color with size {:?}", &size);
					},
					egui::epaint::image::ImageData::Font(font_image) => {
						let size = &font_image.size;
						tracing::debug!("New ImageData::Font with size {:?}", &size);
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
						let sy = 1.0;
						let mtx = Matrix32::identity().with_scaling_xy(1.0, 1.0 / 8.0);
						tex.set_mtx(&mtx);
						let mut pos = Vector2::zero();
						//let mut color = 0xffffffff;
						for y in 0..font_image.size[1] {
							pos.y = y as f32;
							for x in 0..font_image.size[0] {
								pos.x = x as f32;
								let coverage = font_image.pixels[y * font_image.size[0] + x];
								let coverage = (coverage * 255.0) as u8;
								let color = (coverage as u32) * 0x01010101;
								tex.set_texel(&pos, color);
							}
						}
						tex.update_canvas();
						let tid = renderer.register_texture(tex);
						self.texture_ids.insert(*id, tid);
						//todo!();
					},
				}
			}
		}

		let clipped_primitives = self.egui_ctx.tessellate(shapes);
		//tracing::debug!("{:?}", &clipped_primitives);

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
		let tid = self.texture_ids.get(texture_id).unwrap();
		tracing::debug!("Using texture {}", tid);
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
			.scaled_vector2( &Vector2::new( 1.0, -1.0 ) )
			//.scaled( 4.0 )
			.add( &Vector2::new( 0.0, 0.0 ) )
			//.add( &Vector2::new( 5.0*500.0, 4.0*-500.0 ) )
			;

			//tracing::debug!("TC {}, {}", v.uv.x, v.uv.y);
			renderer.set_tex_coords(&Vector2::new(v.uv.x, v.uv.y /*/8.0*/));
			renderer.set_color(&oml_game::renderer::Color::from_rgba(
				v.color.r() as f32 / 255.0,
				v.color.g() as f32 / 255.0,
				v.color.b() as f32 / 255.0,
				v.color.a() as f32 / 255.0,
			));
			let vi = renderer.add_vertex(&vertex);
			vertice_map.insert(i, vi);
			//tracing::debug!("{} -> {}, {:?}", i, vi, v.pos );
		}

		for t in mesh.indices.chunks(3) {
			let tm: Vec<u32> = t
				.iter()
				.map(|i| *vertice_map.get(&(*i as usize)).unwrap())
				.collect();

			//tracing::debug!("{:?}", &tm );
			renderer.add_triangle(tm[0], tm[1], tm[2]);
		}
		Ok(())
	}
}
