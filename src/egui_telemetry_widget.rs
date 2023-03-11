use std::collections::HashMap;

use egui::epaint::{emath::lerp, vec2, Color32, Pos2, Shape, Stroke};
use egui::plot::{Line, Plot, PlotPoints};
use egui::WidgetWithState;
use egui::{Response, Sense, Ui, Widget};
use oml_game::telemetry::TraceInfo;

#[derive(Debug)]
pub struct TraceConfig {
	pub enabled: bool,
	pub color:   egui::Color32,
}

impl Default for TraceConfig {
	fn default() -> Self {
		Self {
			enabled: true,
			color:   egui::Color32::GOLD, //TRANSPARENT,
		}
	}
}

#[derive(Debug, Default)]
pub struct EguiTelemetryWidget {
	count:               usize,
	size:                Option<f32>,
	trace_configs:       HashMap<String, TraceConfig>,
	next_auto_color_idx: usize,
}

impl EguiTelemetryWidget {
	pub fn show(&mut self, ui: &mut Ui) {
		let size = self
			.size
			.unwrap_or_else(|| ui.style().spacing.interact_size.y);
		let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());

		self.count += 1;
		if ui.is_rect_visible(rect) {
			let traces_info = oml_game::DefaultTelemetry::traces_info();
			egui::SidePanel::left("traces_panel")
				.resizable(true)
				.default_width(150.0)
				.width_range(80.0..=200.0)
				.show_inside(ui, |ui| {
					ui.vertical_centered(|ui| {
						ui.heading("Traces");
					});
					egui::ScrollArea::vertical().show(ui, |ui| {
						for ti in traces_info.iter() {
							let color = self.next_auto_color();
							ui.group(|ui| {
								let tc = self
									.trace_configs
									.entry(ti.id().to_string())
									//.or_default();
									.or_insert(TraceConfig {
										color, //: self.next_auto_color(),
										..Default::default()
									});
								//let tc =
								//	self.trace_configs.entry("hoax".to_string()).or_default();
								ui.checkbox(&mut tc.enabled, ti.name());
								//ui.label(ti.name());
								//ui.set_min_height(20.0); // :HACK:
								ui.end_row();
							});
						}
					});
				});

			// :TODO: egui doesn't like the central panel with just a left panel
			egui::TopBottomPanel::bottom("bottom_panel")
				.resizable(false)
				.min_height(0.0)
				.show_inside(ui, |ui| {});

			egui::CentralPanel::default().show_inside(ui, |ui| {
				Plot::new("time_step").show(ui, |plot_ui| {
					for ti in traces_info.iter() {
						let tc = self.trace_configs.entry(ti.id().to_string()).or_default();
						if tc.enabled {
							let color = tc.color;
							let mut line = self.lines_from_trace_info(ti);
							line = line.color(color);
							plot_ui.line(line);
						}
					}
				});
			});
		}
	}
	/*
		fn ys_from_trace<T>(&self, name: &str) -> Vec<Option<f32>>
		where T: oml_game::telemetry::TelemetryEntry,
		{
			let v = oml_game::DefaultTelemetry::get::<T>(name);
			//let u: u8 = v.iter();
			//v.iter().flatten().map(|f| Some(*f as f32) ).collect()
			v.iter().map(|mt| mt.as_ref().map(|t| *t as f32)).collect()
		}
	*/
	fn ys_from_trace_f32(&self, name: &str) -> Vec<Option<f32>> {
		let v = oml_game::DefaultTelemetry::get::<f32>(name);
		//let u: u8 = v.iter();
		//v.iter().flatten().map(|f| Some(*f as f32) ).collect()
		v.iter().map(|mt| mt.as_ref().map(|t| *t as f32)).collect()
	}

	fn ys_from_trace_f64(&self, name: &str) -> Vec<Option<f32>> {
		let v = oml_game::DefaultTelemetry::get::<f64>(name);
		//let u: u8 = v.iter();
		//v.iter().flatten().map(|f| Some(*f as f32) ).collect()
		v.iter().map(|mt| mt.as_ref().map(|t| *t as f32)).collect()
	}

	fn ys_from_trace(&self, name: &str) -> Vec<Option<f32>> {
		let ys_f32 = self.ys_from_trace_f32(name);
		if !ys_f32.is_empty() {
			return ys_f32;
		}
		let ys_f64 = self.ys_from_trace_f64(name);
		if !ys_f64.is_empty() {
			return ys_f64;
		}

		Vec::new()
	}

	fn lines_from_trace_info(&self, trace_info: &TraceInfo) -> Line {
		// :TODO: verify type
		//let v = oml_game::DefaultTelemetry::get::<f32>(trace_info.name());
		//tracing::debug!("lines_from_trace_info: {:#?}", v);
		let maximum_length = oml_game::DefaultTelemetry::maximum_length() as f64;
		let frames = oml_game::DefaultTelemetry::frames() as f64;
		let gone_frames = (frames - maximum_length).max(0.0);
		let x_offset = gone_frames;
		let ys = self.ys_from_trace(trace_info.name());
		//tracing::debug!("{} -> {}", trace_info.name(), ys.len());
		let points: PlotPoints = ys
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
		Line::new(points)
	}
	// borrowed directly from egui
	fn next_auto_color(&mut self) -> Color32 {
		let i = self.next_auto_color_idx;
		self.next_auto_color_idx += 1;
		let golden_ratio = (5.0_f32.sqrt() - 1.0) / 2.0; // 0.61803398875
		let h = i as f32 * golden_ratio;
		egui::epaint::Hsva::new(h, 0.85, 0.5, 1.0).into()
	}
}
