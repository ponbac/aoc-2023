use std::time::Duration;

use egui::{Color32, Sense, Slider, Stroke};

use crate::grid::{Direction, Grid};

pub struct MyApp {
    grid: Grid,
    direction: Direction,
    speed: usize,
    step: bool,
    paused: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            grid: Grid::new(include_str!("input.txt")),
            direction: Direction::North,
            speed: 1,
            step: false,
            paused: true,
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    fn update_state(&mut self) {
        let finished = self.grid.slide(&self.direction);
        if finished {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            };
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                // top padding
                ui.style_mut().spacing.interact_size.y *= 1.4;
                ui.style_mut()
                    .text_styles
                    .get_mut(&egui::TextStyle::Button)
                    .unwrap()
                    .size *= 1.4;

                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
                if ui.button("Step").clicked() {
                    self.step = true;
                }

                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });
            });

            ui.horizontal(|ui| {
                ui.label("Speed: ");
                ui.add(Slider::new(&mut self.speed, 1..=20).prefix("x"));
            });
            ui.add_space(2.5);
        });

        if self.step {
            self.update_state();
            self.step = false;
        } else if !self.paused {
            for _ in 0..self.speed {
                self.update_state();
            }
            ctx.request_repaint_after(Duration::from_millis(1000));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut painter_size = ui.available_size_before_wrap();
            if !painter_size.is_finite() {
                painter_size = egui::vec2(500.0, 500.0);
            }

            const SIDE: f32 = 5.0;

            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            let to_panel_pos = |pos: (usize, usize)| {
                (egui::vec2(pos.0 as f32 * SIDE, pos.1 as f32 * SIDE) + center).to_pos2()
            };

            for x in 0..self.grid.width {
                for y in 0..self.grid.height {
                    let dot = self.grid.get(x, y).unwrap();

                    let color = match dot {
                        '#' => Color32::BLACK,
                        '.' => Color32::WHITE,
                        'O' => Color32::RED,
                        _ => continue,
                    };

                    let dot_pos = to_panel_pos((x, y));
                    painter.circle_stroke(dot_pos, 1.0, Stroke::new(2.0, color));
                }
            }
        });
    }
}
