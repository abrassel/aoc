use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, PartialEq, Default, TryFromPrimitive, derive_more::Display)]
#[repr(u8)]
pub enum Color {
    #[default]
    #[display("⬛")]
    Black = 0,
    #[display("⬜")]
    White = 1,
    #[display("☒")]
    Transparent = 2,
}
