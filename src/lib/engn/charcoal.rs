#[derive(Debug, Clone)]
pub struct Charcoal {
    pub(crate) charmap: Vec<char>,
    pub(crate) coef: f32,
}

impl Charcoal {
    pub fn new(chars: String, draw_dist: f32) -> Self {
        let charmap: Vec<char> = chars.chars().collect();
        Self {
            coef: charmap.len() as f32 / draw_dist,
            charmap,
        }
    }

    pub fn ignite(&self, dist: f32) -> char {
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
