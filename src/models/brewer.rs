
#[derive(Debug, Clone, PartialEq)]
pub enum BrewerType {
    Chemex,
    Immersion,
    NoBypass,
    Aeropress,
    MokaPot,
    PourOver,
    DripMachine,
    FrenchPress,
    SemiAutomaticEspressoMachine,
    ManualEspressoMachine
}

#[derive(Debug, Clone, PartialEq)]
pub struct Brewer  {
    brand: String,
    model: String,
    r#type: Vec<BrewerType>,
    link: Option<String>,
    max_capacity: Option<u32>,
    min_capacity: Option<u32>,
    description: Option<String>
}