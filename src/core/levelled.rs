pub const MAX_MODULUS_COUNT: usize = 64;

/// Denotes a level as a group of basis modulus.
/// RNS decomposition is used to represent large modulus,
/// where the modulus is split into "basis" moduli.
#[derive(Clone, Copy)]
pub struct Level {
    modulus_count: usize,
    used_modulus: [usize; MAX_MODULUS_COUNT],
}

impl Level {
    fn used_modulus(&self) -> &[usize] {
        &self.used_modulus[..self.modulus_count]
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.used_modulus() == other.used_modulus()
    }
}

/// Handle for levels, allowing for easier conversion between certain modulus.
pub trait LevelHandle<Mod> {
    fn get_modulus(&self) -> &Vec<Mod>;

    fn bottom_level(&self) -> Level;

    /// Denotes the top level excluding the auxiliary modulus.
    /// The auxiliary modulus cannot appear in encryptions, so we exclude it in the usual levels.
    fn top_level(&self) -> Level;

    /// 'One level' higher than the given level.
    /// Gives None for top level.
    fn higher_level(&self, level: Level) -> Option<Level>;

    /// 'One level' lower than the given level.
    /// Gives None for bottom level.
    fn lower_level(&self, level: Level) -> Option<Level>;

    // TODO How to handle base extension?
}

/// Verbose
const LINEARS: [usize; MAX_MODULUS_COUNT] = {
    let mut out = [0; MAX_MODULUS_COUNT];
    let mut i = 0;
    while i < MAX_MODULUS_COUNT {
        out[i] = i;
        i += 1;
    }
    out
};

/// Usual, linear level handling.
pub struct LinearLevelHandle<Mod> {
    modulus: Vec<Mod>,
    max_level: usize,
}

impl<Mod> LinearLevelHandle<Mod> {
    // pub fn new(modulus: Vec<Mod>, max_level: usize) {}
}

impl<Mod> LevelHandle<Mod> for LinearLevelHandle<Mod> {
    fn get_modulus(&self) -> &Vec<Mod> {
        &self.modulus
    }

    fn bottom_level(&self) -> Level {
        Level {
            modulus_count: 1,
            used_modulus: LINEARS,
        }
    }

    fn top_level(&self) -> Level {
        Level {
            modulus_count: self.max_level + 1,
            used_modulus: LINEARS,
        }
    }

    fn higher_level(&self, level: Level) -> Option<Level> {
        todo!()
    }

    fn lower_level(&self, level: Level) -> Option<Level> {
        todo!()
    }
}
