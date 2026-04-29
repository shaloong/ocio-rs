use crate::GradingStyle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradingRGBM {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub master: f64,
}

impl GradingRGBM {
    pub fn new(red: f64, green: f64, blue: f64, master: f64) -> Self {
        Self { red, green, blue, master }
    }

    pub fn to_array(&self) -> [f64; 4] {
        [self.red, self.green, self.blue, self.master]
    }

    pub fn from_array(a: &[f64; 4]) -> Self {
        Self { red: a[0], green: a[1], blue: a[2], master: a[3] }
    }
}

impl Default for GradingRGBM {
    fn default() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0, master: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradingRGBMSW {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub master: f64,
    pub start: f64,
    pub width: f64,
}

impl GradingRGBMSW {
    pub fn new(red: f64, green: f64, blue: f64, master: f64, start: f64, width: f64) -> Self {
        Self { red, green, blue, master, start, width }
    }

    pub fn to_array(&self) -> [f64; 6] {
        [self.red, self.green, self.blue, self.master, self.start, self.width]
    }

    pub fn from_array(a: &[f64; 6]) -> Self {
        Self { red: a[0], green: a[1], blue: a[2], master: a[3], start: a[4], width: a[5] }
    }
}

impl Default for GradingRGBMSW {
    fn default() -> Self {
        Self { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.0, width: 1.0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradingPrimary {
    pub brightness: GradingRGBM,
    pub contrast: GradingRGBM,
    pub gamma: GradingRGBM,
    pub offset: GradingRGBM,
    pub exposure: GradingRGBM,
    pub lift: GradingRGBM,
    pub gain: GradingRGBM,
    pub saturation: f64,
    pub pivot: f64,
    pub pivot_black: f64,
    pub pivot_white: f64,
    pub clamp_black: f64,
    pub clamp_white: f64,
}

impl GradingPrimary {
    pub fn new(style: GradingStyle) -> Self {
        let pivot = match style {
            GradingStyle::Log => -0.2,
            _ => 0.18,
        };
        Self {
            brightness: GradingRGBM::default(),
            contrast: GradingRGBM { red: 1.0, green: 1.0, blue: 1.0, master: 1.0 },
            gamma: GradingRGBM { red: 1.0, green: 1.0, blue: 1.0, master: 1.0 },
            offset: GradingRGBM::default(),
            exposure: GradingRGBM::default(),
            lift: GradingRGBM::default(),
            gain: GradingRGBM { red: 1.0, green: 1.0, blue: 1.0, master: 1.0 },
            saturation: 1.0,
            pivot,
            pivot_black: 0.0,
            pivot_white: 1.0,
            clamp_black: GradingPrimary::no_clamp_black(),
            clamp_white: GradingPrimary::no_clamp_white(),
        }
    }

    pub fn no_clamp_black() -> f64 { -0.0001 }
    pub fn no_clamp_white() -> f64 { 1.0001 }

    pub(crate) fn from_flat_array(values: &[f64; 34]) -> Self {
        let mut off = 0;
        let mut read_rgbm = || -> GradingRGBM {
            let r = GradingRGBM { red: values[off], green: values[off+1], blue: values[off+2], master: values[off+3] };
            off += 4;
            r
        };
        let brightness = read_rgbm();
        let contrast = read_rgbm();
        let gamma = read_rgbm();
        let offset_rgbm = read_rgbm();
        let exposure = read_rgbm();
        let lift = read_rgbm();
        let gain = read_rgbm();
        let saturation = values[off]; off += 1;
        let pivot = values[off]; off += 1;
        let pivot_black = values[off]; off += 1;
        let pivot_white = values[off]; off += 1;
        let clamp_black = values[off]; off += 1;
        let clamp_white = values[off]; off += 1;
        Self {
            brightness, contrast, gamma,
            offset: offset_rgbm,
            exposure, lift, gain,
            saturation, pivot, pivot_black, pivot_white,
            clamp_black, clamp_white,
        }
    }

    pub(crate) fn to_flat_array(&self) -> [f64; 34] {
        let mut a = [0.0f64; 34];
        let mut off = 0;
        let mut write_rgbm = |g: &GradingRGBM| {
            a[off] = g.red; a[off+1] = g.green; a[off+2] = g.blue; a[off+3] = g.master;
            off += 4;
        };
        write_rgbm(&self.brightness);
        write_rgbm(&self.contrast);
        write_rgbm(&self.gamma);
        write_rgbm(&self.offset);
        write_rgbm(&self.exposure);
        write_rgbm(&self.lift);
        write_rgbm(&self.gain);
        a[off] = self.saturation; off += 1;
        a[off] = self.pivot; off += 1;
        a[off] = self.pivot_black; off += 1;
        a[off] = self.pivot_white; off += 1;
        a[off] = self.clamp_black;
        a[off + 1] = self.clamp_white;
        a
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradingTone {
    pub blacks: GradingRGBMSW,
    pub shadows: GradingRGBMSW,
    pub midtones: GradingRGBMSW,
    pub highlights: GradingRGBMSW,
    pub whites: GradingRGBMSW,
    pub scontrast: f64,
}

impl GradingTone {
    pub fn new(style: GradingStyle) -> Self {
        let (blacks, shadows, midtones, highlights, whites) = match style {
            GradingStyle::Lin => (
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.0, width: 4.0 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 2.0, width: -7.0 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.0, width: 8.0 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: -2.0, width: 9.0 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.0, width: 8.0 },
            ),
            _ => (
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.4, width: 0.4 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.5, width: 0.0 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.4, width: 0.7 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.2, width: 1.0 },
                GradingRGBMSW { red: 1.0, green: 1.0, blue: 1.0, master: 1.0, start: 0.5, width: 0.5 },
            ),
        };
        Self { blacks, shadows, midtones, highlights, whites, scontrast: 1.0 }
    }

    pub(crate) fn from_flat_array(values: &[f64; 31]) -> Self {
        let mut off = 0;
        let mut read_rgbmsw = || -> GradingRGBMSW {
            let r = GradingRGBMSW {
                red: values[off], green: values[off+1], blue: values[off+2],
                master: values[off+3], start: values[off+4], width: values[off+5],
            };
            off += 6;
            r
        };
        let blacks = read_rgbmsw();
        let shadows = read_rgbmsw();
        let midtones = read_rgbmsw();
        let highlights = read_rgbmsw();
        let whites = read_rgbmsw();
        let scontrast = values[off]; off += 1;
        Self { blacks, shadows, midtones, highlights, whites, scontrast }
    }

    pub(crate) fn to_flat_array(&self) -> [f64; 31] {
        let mut a = [0.0f64; 31];
        let mut off = 0;
        let mut write_rgbmsw = |g: &GradingRGBMSW| {
            a[off] = g.red; a[off+1] = g.green; a[off+2] = g.blue;
            a[off+3] = g.master; a[off+4] = g.start; a[off+5] = g.width;
            off += 6;
        };
        write_rgbmsw(&self.blacks);
        write_rgbmsw(&self.shadows);
        write_rgbmsw(&self.midtones);
        write_rgbmsw(&self.highlights);
        write_rgbmsw(&self.whites);
        a[off] = self.scontrast;
        a
    }
}
