use anchor_lang::prelude::*;

#[error_code]
pub enum ChessError {
    #[msg("Invalid player")]
    InvalidPlayer,
    #[msg("Chess piece selected to move does not match the board current state")]
    InvalidPiece,
    #[msg("Cannot remove chess piece")]
    RemoveError,
    #[msg("Cannot move chess piece off of the board")]
    MoveOutOfBounds,
    #[msg("Cannot move chess piece in this scenario")]
    MoveNotPossible,
    #[msg("Cannot move chess piece when king is in check")]
    MoveInCheckViolation,
    #[msg("Cannot move chess piece if that move would leave the king in check")]
    MoveIntoCheckViolation,
    #[msg("Cannot move to a spot occupied by the same team")]
    SpotOccupied,
    #[msg("Cannot move out of turn")]
    NotPlayersTurn,
    #[msg("Cannot start game twice")]
    GameAlreadyStarted
}
