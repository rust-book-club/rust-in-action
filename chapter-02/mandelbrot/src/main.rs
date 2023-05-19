use num::complex::Complex;

fn mandelbrot_at_point(
    cx: f64,
    cy: f64,
    max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize
) -> Vec<Vec<usize>> {

    let mut rows = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {

    fn color(int: i32) -> String {
        format!("\x1b[0;{}m\u{2588}\x1b[0m", int)
    }

    let red = color(31);
    let yellow = color(33);
    let green = color(32);
    let cyan = color(36);
    let blue = color(34);
    let magenta = color(35);

    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => " ",
                3..=5 => ".",
                6..=10 => "•",
                11..=30 => magenta.as_str(),
                31..=100 => blue.as_str(),
                101..=200 => cyan.as_str(),
                201..=300 => green.as_str(),
                301..=999 => yellow.as_str(),
                _ => red.as_str(),
            };

            line.push_str(val);
        }

        println!("{line}");
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 200, 48);

    render_mandelbrot(mandelbrot);
}
