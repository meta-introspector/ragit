use serde::{Deserialize, Serialize};

/// Represents a single trait of a meme, acting as a conceptual quasi-fiber.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeTrait {
    pub name: String,
    pub description: String,
    pub initial_value: f64,
    pub prime_quotient: (u32, u32), // Numerator, Denominator for prime-based fractions
}

impl MemeTrait {
    pub fn new(
        name: &str,
        description: &str,
        initial_value: f64,
        prime_quotient: (u32, u32),
    ) -> Self {
        MemeTrait {
            name: name.to_string(),
            description: description.to_string(),
            initial_value,
            prime_quotient,
        }
    }
}

/// A table of MemeTraits, representing a collection of quasi-fibers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeTraitTable {
    pub traits: Vec<MemeTrait>,
}

impl MemeTraitTable {
    pub fn new() -> Self {
        MemeTraitTable { traits: Vec::new() }
    }

    pub fn add_trait(&mut self, meme_trait: MemeTrait) {
        self.traits.push(meme_trait);
    }

    /// Populates the table with the 20 traits from the SOLFUNMEME LaTeX document.
    pub fn populate_solfunmeme_traits(&mut self) {
        self.add_trait(MemeTrait::new("Eb", "Blue Eye: Introspection", 0.8, (2, 5)));
        self.add_trait(MemeTrait::new("Pr", "Red Petals/Fungal Body: Chaotic Growth", 0.7, (7, 10)));
        self.add_trait(MemeTrait::new("My", "Mycelium, hidden: Underlying Network", 0.5, (1, 2)));
        self.add_trait(MemeTrait::new("Cb", "Cosmic Bg: Meta-Context", 0.6, (3, 5)));
        self.add_trait(MemeTrait::new("Glw", "Glowing White: Energy/Hype", 0.4, (2, 5)));
        self.add_trait(MemeTrait::new("Swl", "Swirling: Dynamism", 0.3, (3, 10)));
        self.add_trait(MemeTrait::new("Intp", "Intricate Patterns: Complexity", 0.4545, (5, 11)));
        self.add_trait(MemeTrait::new("Abs", "Abstract: Meta-Interpretation", 0.6364, (7, 11)));
        self.add_trait(MemeTrait::new("Geo", "Geometric Shapes: Immutable Core", 0.1429, (1, 7)));
        self.add_trait(MemeTrait::new("Sur", "Surreal: Beyond Norms", 0.7143, (5, 7)));
        self.add_trait(MemeTrait::new("Fan", "Fantastical: Imaginative Potential", 0.5, (1, 2)));
        self.add_trait(MemeTrait::new("Gl", "Gills: Fungal Structure", 0.4286, (3, 7)));
        self.add_trait(MemeTrait::new("Sp", "Spores: Spore Spread", 0.2857, (2, 7)));
        self.add_trait(MemeTrait::new("Cir", "Circuit Patterns: Tech Integration", 0.3846, (5, 13)));
        self.add_trait(MemeTrait::new("Sym", "Symmetry: Balanced Structure", 0.5385, (7, 13)));
        self.add_trait(MemeTrait::new("Ef", "Energy Flow: Dynamic Propagation", 0.2727, (3, 11)));
        self.add_trait(MemeTrait::new("Pp", "Purple Petals: Hidden Layers", 0.1818, (2, 11)));
        self.add_trait(MemeTrait::new("Bc", "Blue Corridors: Mycelial Channels", 0.2308, (3, 13)));
        self.add_trait(MemeTrait::new("Lp", "Lego-like Protrusions: Modularity", 0.0909, (1, 11)));
        self.add_trait(MemeTrait::new("Sg", "Star Glow: Cosmic Energy", 0.2941, (5, 17)));
    }
}
