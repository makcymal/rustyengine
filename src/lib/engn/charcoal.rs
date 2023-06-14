#[derive(Debug, Clone)]
pub struct Charcoal {
    pub(crate) charmap: Vec<char>,
    pub(crate) coef: f64,
}

impl Charcoal {
    pub fn new(chars: String, draw_dist: f64) -> Self {
        let charmap: Vec<char> = chars.chars().collect();
        Self {
            coef: charmap.len() as f64 / draw_dist,
            charmap,
        }
    }

    pub fn ignite(&self, dist: f64) -> char {
        let idx = (dist * self.coef).floor();
        if idx < 0.0 {
            return *self.charmap.last().unwrap();
        }
        match self.charmap.get(idx as usize) {
            Some(c) => *c,
            None => *self.charmap.last().unwrap(),
        }
    }
}
