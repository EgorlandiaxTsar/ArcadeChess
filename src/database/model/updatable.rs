use bigdecimal::BigDecimal;

pub struct OptionalDoubleChessGame {}

pub struct OptionalUser {
    pub password: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub double_chess_bullet_rating: Option<BigDecimal>,
    pub double_chess_blitz_rating: Option<BigDecimal>,
    pub double_chess_rapid_rating: Option<BigDecimal>,
    pub double_chess_classical_rating: Option<BigDecimal>,
}
