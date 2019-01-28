extern crate palette;

use core::f32::consts::PI;
use palette::{Lab, Srgb};

fn sqrtf(n: f32) -> f32 {
    n.sqrt()
}

fn powf(n: f32, exp: f32) -> f32 {
    n.powf(exp)
}

fn absf(n: f32) -> f32 {
    n.abs()
}

fn cosf(n: f32) -> f32 {
    n.cos()
}

fn sinf(n: f32) -> f32 {
    n.sin()
}

pub fn rgb_to_lab(c: &Srgb) -> Lab {
    let f = c.into_format();
    Lab::from(f)
}

pub fn ciede2000_diff(c1: &Lab, c2: &Lab) -> f32 {
    let l1 = c1.l;
    let a1 = c1.a;
    let b1 = c1.b;

    let l2 = c2.l;
    let a2 = c2.a;
    let b2 = c2.b;

    // Weight factors
    let kl = 1.0;
    let kc = 1.0;
    let kh = 1.0;

    let c1 = (a1.powf(2.0) + b1.powf(2.0)).sqrt();
    let c2 = (a2.powf(2.0) + b2.powf(2.0)).sqrt();

    let a_c1_c2 = (c1 + c2) / 2.0;

    let g =
        0.5 * (1.0 - (a_c1_c2.powf(7.0) / (a_c1_c2.powf(7.0) + (25.0 as f32).powf(7.0))).sqrt());

    let a1p = (1.0 + g) * a1; //(5)
    let a2p = (1.0 + g) * a2; //(5)

    let c1p = sqrtf(powf(a1p, 2.0) + powf(b1, 2.0)); //(6)
    let c2p = sqrtf(powf(a2p, 2.0) + powf(b2, 2.0)); //(6)

    let h1p = hp_f(b1, a1p); //(7)
    let h2p = hp_f(b2, a2p); //(7)

    // Step 2: Calculate dLp, dCp, dHp
    let dlp = l2 - l1; //(8)
    let dcp = c2p - c1p; //(9)

    let dhp = dhp_f(c1, c2, h1p, h2p); //(10)
    let dhp = 2.0 * sqrtf(c1p * c2p) * (radians(dhp) / 2.0).sin(); //(11)

    // Step 3: Calculate CIEDE2000 Color-Difference
    let a_l = (l1 + l2) / 2.0; //(12)
    let a_cp = (c1p + c2p) / 2.0; //(13)

    let a_hp = a_hp_f(c1, c2, h1p, h2p); //(14)

    let t = 1.0 - 0.17 * cosf(radians(a_hp - 30.0))
        + 0.24 * cosf(radians(2.0 * a_hp))
        + 0.32 * cosf(radians(3.0 * a_hp + 6.0))
        - 0.2 * cosf(radians(4.0 * a_hp - 63.0));
    let d_ro = 30.0 * (-powf((a_hp - 275.0) / 25.0, 2.0)).exp();
    let rc = sqrtf(powf(a_cp, 7.0) / (powf(a_cp, 7.0) + powf(25.0, 7.0)));

    let sl = 1.0 + (0.015 * powf(a_l - 50.0, 2.0)) / sqrtf(20.0 + powf(a_l - 50.0, 2.0)); //(18)
    let sc = 1.0 + 0.045 * a_cp; //(19)
    let sh = 1.0 + 0.015 * a_cp * t; //(20)
    let rt = -2.0 * rc * sinf(radians(2.0 * d_ro)); //(21)

    let de = sqrtf(
        powf(dlp / (sl * kl), 2.0)
            + powf(dcp / (sc * kc), 2.0)
            + powf(dhp / (sh * kh), 2.0)
            + rt * (dcp / (sc * kc)) * (dhp / (sh * kh)),
    );
    return de.floor();
}

// ---------------- internal utils  -----------------------
fn degrees(n: f32) -> f32 {
    n * (180.0 / PI)
}

fn radians(n: f32) -> f32 {
    n * (PI / 180.0)
}

fn hp_f(x: f32, y: f32) -> f32 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    } else {
        let tmphp = degrees(y.atan2(x));
        if tmphp >= 0.0 {
            return tmphp;
        } else {
            return tmphp + 360.0;
        }
    }
}

fn dhp_f(c1: f32, c2: f32, h1p: f32, h2p: f32) -> f32 {
    if c1 * c2 == 0.0 {
        return 0.0;
    } else if (h2p - h1p).abs() <= 180.0 {
        return h2p - h1p;
    } else if (h2p - h1p) > 180.0 {
        return (h2p - h1p) - 360.0;
    } else if (h2p - h1p) < -180.0 {
        return (h2p - h1p) + 360.0;
    }
    0.0
}

fn a_hp_f(c1: f32, c2: f32, h1p: f32, h2p: f32) -> f32 {
    if c1 * c2 == 0.0 {
        return h1p + h2p;
    } else if absf(h1p - h2p) <= 180.0 {
        return (h1p + h2p) / 2.0;
    } else if absf(h1p - h2p) > 180.0 && h1p + h2p < 360.0 {
        return (h1p + h2p + 360.0) / 2.0;
    } else if absf(h1p - h2p) > 180.0 && h1p + h2p >= 360.0 {
        return (h1p + h2p - 360.0) / 2.0;
    }
    0.0
}
