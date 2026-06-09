use crate::cpu::CpuDebugSnapshot;
use crate::ppu::PpuDebugSnapshot;
use crate::{Pixels, HEIGHT, WIDTH};
use rgb::Rgb;

pub const DEBUG_PANEL_WIDTH: usize = 168;
pub const APP_WIDTH: usize = WIDTH + DEBUG_PANEL_WIDTH;

#[derive(Clone, Copy, Debug, Default)]
pub struct ColorToggles {
    pub orange: bool,
    pub indigo: bool,
}

impl ColorToggles {
    pub fn frame_color(self) -> Rgb<u8> {
        match (self.orange, self.indigo) {
            (false, false) => Rgb {
                r: 255,
                g: 255,
                b: 0,
            },
            (true, false) => Rgb {
                r: 255,
                g: 108,
                b: 0,
            },
            (false, true) => Rgb {
                r: 70,
                g: 120,
                b: 255,
            },
            (true, true) => Rgb {
                r: 220,
                g: 70,
                b: 255,
            },
        }
    }
}

pub fn draw_app_frame(
    buffer: &mut [u32],
    pixels: &Pixels,
    cpu_debug: &CpuDebugSnapshot,
    ppu_debug: &PpuDebugSnapshot,
    color_toggles: ColorToggles,
) {
    buffer.fill(0x111318);
    pixels.copy_to_app_buffer(buffer, APP_WIDTH);
    draw_debug_panel(buffer, cpu_debug, ppu_debug, color_toggles);
}

fn draw_debug_panel(
    buffer: &mut [u32],
    cpu_debug: &CpuDebugSnapshot,
    ppu_debug: &PpuDebugSnapshot,
    color_toggles: ColorToggles,
) {
    let left = WIDTH;

    for y in 0..HEIGHT {
        buffer[y * APP_WIDTH + left] = 0x2A2F3A;
    }

    draw_text(buffer, left + 10, 10, "CPU DEBUG", 0xE6EDF3);
    draw_text(
        buffer,
        left + 10,
        28,
        &format!("PC  ${:04X}", cpu_debug.program_counter),
        0xC9D1D9,
    );
    draw_text(
        buffer,
        left + 10,
        40,
        &format!(
            "A {:02X} X {:02X} Y {:02X}",
            cpu_debug.accumulator, cpu_debug.x, cpu_debug.y
        ),
        0xC9D1D9,
    );
    draw_text(
        buffer,
        left + 10,
        52,
        &format!(
            "SP {:02X} P {:02X}",
            cpu_debug.stack_pointer, cpu_debug.processor_status
        ),
        0xC9D1D9,
    );
    draw_text(
        buffer,
        left + 10,
        70,
        &format!("CYC {}", cpu_debug.total_cpu_cycles),
        0xA5D6FF,
    );
    draw_text(
        buffer,
        left + 10,
        82,
        &format!("INS {}", cpu_debug.instruction_count),
        0xA5D6FF,
    );
    draw_text(
        buffer,
        left + 10,
        94,
        &format!("PPU S{:03} D{:03}", ppu_debug.scanline, ppu_debug.dot),
        0x7EE787,
    );
    draw_text(
        buffer,
        left + 10,
        106,
        &format!("FRM {}", ppu_debug.frame),
        0x7EE787,
    );
    draw_text(
        buffer,
        left + 10,
        118,
        if ppu_debug.in_vblank {
            "VBLANK YES"
        } else {
            "VBLANK NO"
        },
        if ppu_debug.in_vblank {
            0xF2CC60
        } else {
            0x7EE787
        },
    );
    draw_text(
        buffer,
        left + 10,
        136,
        if cpu_debug.last_instruction_success {
            "STATUS OK"
        } else {
            "STATUS WAIT"
        },
        if cpu_debug.last_instruction_success {
            0x7EE787
        } else {
            0xF2CC60
        },
    );
    draw_text(
        buffer,
        left + 10,
        148,
        &format!(
            "O {} I {}",
            toggle_label(color_toggles.orange),
            toggle_label(color_toggles.indigo)
        ),
        0xFFA657,
    );

    draw_text(buffer, left + 10, 168, "CURRENT", 0xE6EDF3);
    for (index, line) in wrap_debug_text(&cpu_debug.current_instruction, 21)
        .iter()
        .take(5)
        .enumerate()
    {
        draw_text(buffer, left + 10, 182 + (index * 12), line, 0xC9D1D9);
    }
}

fn toggle_label(enabled: bool) -> &'static str {
    if enabled {
        "ON"
    } else {
        "OFF"
    }
}

fn wrap_debug_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if !current.is_empty() && current.len() + word.len() + 1 > width {
            lines.push(current);
            current = String::new();
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn draw_text(buffer: &mut [u32], x: usize, y: usize, text: &str, color: u32) {
    for (index, character) in text.chars().enumerate() {
        draw_character(buffer, x + index * 6, y, character, color);
    }
}

fn draw_character(buffer: &mut [u32], x: usize, y: usize, character: char, color: u32) {
    for (row, bits) in glyph(character).iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                let target_x = x + col;
                let target_y = y + row;
                if target_x < APP_WIDTH && target_y < HEIGHT {
                    buffer[target_y * APP_WIDTH + target_x] = color;
                }
            }
        }
    }
}

fn glyph(character: char) -> [u8; 7] {
    match character.to_ascii_uppercase() {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0E],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x1F],
        'J' => [0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0E],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x15, 0x0A],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        '3' => [0x1E, 0x01, 0x01, 0x0E, 0x01, 0x01, 0x1E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x10, 0x1E, 0x01, 0x01, 0x1E],
        '6' => [0x0E, 0x10, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x01, 0x0E],
        '$' => [0x04, 0x0F, 0x14, 0x0E, 0x05, 0x1E, 0x04],
        ':' => [0x00, 0x04, 0x04, 0x00, 0x04, 0x04, 0x00],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x08],
        '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F],
        '(' => [0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02],
        ')' => [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08],
        ' ' => [0x00; 7],
        _ => [0x1F, 0x01, 0x02, 0x04, 0x04, 0x00, 0x04],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_panel_expands_frame_and_draws_snapshot() {
        let pixels = Pixels::new();
        pixels.write(
            0,
            0,
            Rgb {
                r: 0xAA,
                g: 0x55,
                b: 0x11,
            },
        );

        let snapshot = CpuDebugSnapshot {
            instruction_address: 0xC000,
            program_counter: 0xC000,
            accumulator: 0x01,
            x: 0x02,
            y: 0x03,
            stack_pointer: 0xFD,
            processor_status: 0x24,
            current_instruction: "Instruction { opcode: LDA }".to_string(),
            last_instruction_success: true,
            total_cpu_cycles: 123,
            instruction_count: 45,
        };

        let mut buffer = vec![0; APP_WIDTH * HEIGHT];
        let ppu_snapshot = PpuDebugSnapshot {
            scanline: 241,
            dot: 0,
            frame: 2,
            in_vblank: true,
        };
        draw_app_frame(
            &mut buffer,
            &pixels,
            &snapshot,
            &ppu_snapshot,
            ColorToggles::default(),
        );

        assert_eq!(APP_WIDTH, WIDTH + DEBUG_PANEL_WIDTH);
        assert_eq!(buffer[0], 0xAA5511);
        assert_eq!(buffer[WIDTH], 0x2A2F3A);
        let text_row_start = 10 * APP_WIDTH;
        assert!(
            buffer[text_row_start + WIDTH + 10..text_row_start + APP_WIDTH]
                .iter()
                .any(|pixel| *pixel != 0x111318)
        );
    }

    #[test]
    fn color_toggles_change_frame_color() {
        assert_eq!(
            ColorToggles::default().frame_color(),
            Rgb {
                r: 255,
                g: 255,
                b: 0
            }
        );
        assert_eq!(
            ColorToggles {
                orange: true,
                indigo: false
            }
            .frame_color(),
            Rgb {
                r: 255,
                g: 108,
                b: 0
            }
        );
        assert_eq!(
            ColorToggles {
                orange: false,
                indigo: true
            }
            .frame_color(),
            Rgb {
                r: 70,
                g: 120,
                b: 255
            }
        );
        assert_eq!(
            ColorToggles {
                orange: true,
                indigo: true
            }
            .frame_color(),
            Rgb {
                r: 220,
                g: 70,
                b: 255
            }
        );
    }
}
