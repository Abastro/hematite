pub const MAX_MODULUS_COUNT: usize = 64;

/// Denotes a group of basis modulus.
/// RNS decomposition is used to represent large modulus,
/// where the modulus is split into "basis" moduli.
#[derive(Clone, Copy)]
pub struct ModGroup {
    modulus_count: usize,
    used_modulus: [usize; MAX_MODULUS_COUNT],
}

impl ModGroup {
    fn used_modulus(&self) -> &[usize] {
        &self.used_modulus[..self.modulus_count]
    }
}

impl PartialEq for ModGroup {
    fn eq(&self, other: &Self) -> bool {
        self.used_modulus() == other.used_modulus()
    }
}

/// Handle for levels, allowing for easier conversion between certain modulus.
pub trait LevelHandle<Mod> {
    fn get_modulus(&self) -> &Vec<Mod>;

    fn bottom_level(&self) -> ModGroup;

    /// Denotes the top level excluding the auxiliary modulus.
    /// The auxiliary modulus cannot appear in encryptions, so we exclude it in the usual levels.
    fn top_level(&self) -> ModGroup;

    /// 'One level' higher than the given level.
    /// Gives None if the higher level does not exist.
    fn higher_level(&self, level: ModGroup) -> Option<ModGroup>;

    /// 'One level' lower than the given level.
    /// Gives None if the lower level does not exist.
    fn lower_level(&self, level: ModGroup) -> Option<ModGroup>;

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

// TODO Auxiliary modulus for linear level handling

/// Linear level handling with auxiliary modulus.
pub struct LinearLevelHandle<Mod> {
    modulus: Vec<Mod>,
    max_level: usize,
}

impl<Mod> LinearLevelHandle<Mod> {
    pub fn new(modulus: Vec<Mod>, max_level: usize) -> LinearLevelHandle<Mod> {
        assert!(modulus.len() <= MAX_MODULUS_COUNT);
        LinearLevelHandle { modulus, max_level }
    }

    pub fn get_level(&self, level: usize) -> Option<ModGroup> {
        (level <= self.max_level).then(|| ModGroup {
            modulus_count: level + 1,
            used_modulus: LINEARS,
        })
    }

    pub fn level_usize(&self, level: ModGroup) -> Option<usize> {
        (level.used_modulus == LINEARS)
            .then(|| level.modulus_count - 1)
            .filter(|lv| *lv <= self.max_level)
    }
}

impl<Mod> LevelHandle<Mod> for LinearLevelHandle<Mod> {
    fn get_modulus(&self) -> &Vec<Mod> {
        &self.modulus
    }

    fn bottom_level(&self) -> ModGroup {
        self.get_level(0).expect("bottom level should be valid")
    }

    fn top_level(&self) -> ModGroup {
        self.get_level(self.max_level)
            .expect("top level should be valid")
    }

    fn higher_level(&self, level: ModGroup) -> Option<ModGroup> {
        self.level_usize(level)
            .and_then(|lv| self.get_level(lv + 1))
    }

    fn lower_level(&self, level: ModGroup) -> Option<ModGroup> {
        self.level_usize(level)
            .and_then(|lv| self.get_level(lv - 1))
    }
}
