//! Algoritmos evaluados para el proyecto vaint
use crate::PixelCoord;

// /// Devuelve un vector de coordenadas de pixeles que representan una línea entre `p0` y `p` usando el algoritmo [DDA](https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)).
// pub fn dda(p0: PixelCoord, p: PixelCoord) -> Vec<PixelCoord> {
//     let mut vec = vec![];
//     write_dda(p0, p, &mut vec);
//     vec
// }

/// Escribe al buffer dado los puntos ubicados entre `p0` y `p` usando el Algoritmo
/// [DDA][algo] (Digital Differential Analyzer).
///
/// [algo]: https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
pub fn write_dda(p0: PixelCoord, p: PixelCoord, puntos: &mut Vec<PixelCoord>) {
    let (x_0, y_0, x, y) = (p0.0, p0.1, p.0, p.1);
    let delta_x = x - x_0;
    let delta_y = y - y_0;

    let steps = i32::max(delta_x.abs(), delta_y.abs());

    let dx = (delta_x as f32) / steps as f32;
    let dy = (delta_y as f32) / steps as f32;

    let steps = steps + 1; // Se agrega un paso adicional para incluir el último punto

    puntos.reserve_exact(steps as usize);
    for k in 0..steps {
        let k: f32 = k as f32;
        let x: i32 = x_0 + (dx * k).round() as i32;
        let y: i32 = y_0 + (dy * k).round() as i32;
        puntos.push((x, y));
    }
}

/// Estructura auxiliar para el dibujo de un círculo.
#[repr(transparent)]
struct Circulo(PixelCoord);
impl Circulo {
    #[allow(clippy::new_without_default)]
    #[inline(always)]
    pub fn new() -> Self { Self((0, 0)) }

    #[inline(always)]
    pub fn x(&self) -> i32 { self.0.0 }

    #[inline(always)]
    pub fn y(&self) -> i32 { self.0.1 }

    #[inline(always)]
    pub fn set_coords(&mut self, p: PixelCoord) { self.0 = p; }

    #[inline(always)]
    pub fn x_mut(&mut self) -> &mut i32 { &mut self.0.0 }

    #[inline(always)]
    pub fn y_mut(&mut self) -> &mut i32 { &mut self.0.1 }
}

/// Escribe al buffer dado los puntos ubicados en la circunferencia del círculo usando el algoritmo
/// de CPM (Circulo de Punto Medio).
pub fn write_circle_middle_point(centro: PixelCoord, r: i32, puntos: &mut Vec<PixelCoord>) {
    let (cx, cy) = centro;
    let mut c = Circulo::new();
    c.set_coords((0, r));
    let mut d = 1 - r;

    let mut plot_circle = |c: &Circulo| {
        // Se insertan los puntos iniciales para cada octante
        puntos.extend([
            (cx + c.x(), cy + c.y()),
            (cx - c.x(), cy + c.y()),
            (cx + c.x(), cy - c.y()),
            (cx - c.x(), cy - c.y()),
            (cx + c.y(), cy + c.x()),
            (cx - c.y(), cy + c.x()),
            (cx + c.y(), cy - c.x()),
            (cx - c.y(), cy - c.x()),
        ]);
    };

    plot_circle(&c);
    let mut circulo = c;
    while circulo.x() < circulo.y() {
        *circulo.x_mut() += 1;
        if d < 0 {
            d += 2 * circulo.x() + 1;
        } else {
            *circulo.y_mut() -= 1;
            d += 2 * (circulo.x() - circulo.y()) + 1;
        }
        plot_circle(&circulo);
    }
}

/// Escribe al buffer dado los puntos ubicados en la circunferencia de una elipse usando el
/// algoritmo de EPM (Elipse de Punto Medio).
pub fn write_ellipse_middle_point(centro: PixelCoord, rx: i32, ry: i32, puntos: &mut Vec<PixelCoord>) {
    let (rx2, ry2) = (rx.pow(2), ry.pow(2));
    let (two_rx2, two_ry2) = (2 * rx2, 2 * ry2);
    let (cx, cy) = centro;

    let round = |arg: f32| -> i32 { (arg + 0.5) as i32 };
    // Función auxiliar para registrar los puntos de la elipse en el vector
    let mut plot_elipse = |x: i32, y: i32| {
        #[rustfmt::skip]
        puntos.extend([
            (cx + x, cy + y),
            (cx - x, cy + y),
            (cx + x, cy - y),
            (cx - x, cy - y),
        ]);
    };

    let (mut x, mut y) = (0, ry);

    plot_elipse(x, y);

    let (mut px, mut py) = (0, two_rx2 * y);
    let mut p = round(ry2 as f32 - (rx2 * y) as f32 + (0.25 * rx2 as f32));

    // Región 1
    while px < py {
        x += 1;
        px += two_ry2;
        if p < 0 {
            p += ry2 + px;
        } else {
            y -= 1;
            py -= two_rx2;
            p += ry2 + px - py;
        }
        plot_elipse(x, y);
    }

    // Región 2
    p = {
        let (ry2, rx2, x, y) = (ry2 as f32, rx2 as f32, x as f32, y as f32);
        round(ry2 * (x + 0.5).powi(2) + rx2 * (y - 1.0).powi(2) - rx2 * ry2)
    };

    while y > 0 {
        y -= 1;
        py -= two_rx2;
        if p > 0 {
            p += rx2 - py;
        } else {
            x += 1;
            px += two_ry2;
            p += rx2 - py + px;
        }
        plot_elipse(x, y);
    }
}
