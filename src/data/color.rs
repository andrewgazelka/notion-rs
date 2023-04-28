#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Blue,
    BlueBackground,
    Brown,
    BrownBackground,
    Default,
    Gray,
    GrayBackground,
    Green,
    GreenBackground,
    Orange,
    OrangeBackground,
    Yellow,
    Pink,
    PinkBackground,
    Purple,
    PurpleBackground,
    Red,
    RedBackground,
    YellowBackground,
}
