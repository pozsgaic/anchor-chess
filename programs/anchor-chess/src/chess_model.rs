use anchor_lang::prelude::*;
use num_derive::*;
use crate::chess_error::ChessError;

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Stalemate,
    Won {winner: Pubkey},
}

#[derive(Debug, Default, AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct ChessPlayer {
  pub key: Pubkey,
  pub team: u8,
  pub score: u8
}

#[repr(u8)]
#[derive(Debug, AnchorSerialize, AnchorDeserialize, ToPrimitive, FromPrimitive, Copy, Clone, PartialEq, Eq)]
pub enum ChessPieceTypes {
  Pawn = 0,
  Knight = 1,
  Bishop = 2,
  Rook = 3,
  Queen = 4,
  King = 5
}

impl ChessPieceTypes {
  pub fn value(&self) -> u8 {
    match *self {
      ChessPieceTypes::Pawn => 1,
      ChessPieceTypes::Knight | ChessPieceTypes::Bishop => 3,
      ChessPieceTypes::Rook => 5,
      ChessPieceTypes::Queen => 9,
      ChessPieceTypes::King => 0
    }
  }
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub struct ChessPiece {
    pub team: u8,
    pub piece_type: ChessPieceTypes,
    pub x_loc: u8,
    pub y_loc: u8,
}

impl ChessPiece {
  pub fn new_pawn(tm: u8, x: u8, y: u8) -> ChessPiece {
    ChessPiece { team: tm, piece_type: ChessPieceTypes::Pawn, x_loc: x, y_loc: y}
  }

  pub fn new_knight(tm: u8, x: u8, y: u8) -> ChessPiece {
    ChessPiece { team: tm, piece_type: ChessPieceTypes::Knight, x_loc: x, y_loc: y}
  }

  pub fn new_bishop(tm: u8, x: u8, y: u8) -> ChessPiece {
    ChessPiece { team: tm, piece_type: ChessPieceTypes::Bishop, x_loc: x, y_loc: y}
  }

  pub fn new_rook(tm: u8, x: u8, y: u8) -> ChessPiece {
    ChessPiece { team: tm, piece_type: ChessPieceTypes::Rook, x_loc: x, y_loc: y}
  }

  pub fn new_queen(tm: u8, x: u8, y: u8) -> ChessPiece {
    ChessPiece { team: tm, piece_type: ChessPieceTypes::Queen, x_loc: x, y_loc: y}
  }

  pub fn new_king(tm: u8, x: u8, y: u8) -> ChessPiece {
      ChessPiece {team: tm, piece_type: ChessPieceTypes::King, x_loc: x, y_loc: y}
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Move {
  pub x: u8,
  pub y: u8
}

#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ChessBoard {
    pub players: [ChessPlayer; 2],
    pub board: [ [Option<ChessPiece>; 8]; 8],
    pub player_pieces: [ [Option<ChessPiece>; 16]; 2],
}

impl ChessBoard {
  pub fn new() -> ChessBoard {
    ChessBoard { ..Default::default() }
  }

  pub fn initialize(&mut self) -> Result<()> {
    //  Player 1: fill rows 0 and 1
    self.player_pieces[0][0] = Some(ChessPiece::new_rook(0,0, 0));
    self.board[0][0] = self.player_pieces[0][0];

    self.player_pieces[0][1] =  Some(ChessPiece::new_knight(0, 0, 1));
    self.board[0][1] = self.player_pieces[0][1];

    self.player_pieces[0][2] = Some(ChessPiece::new_bishop(0, 0, 2));
    self.board[0][2] = self.player_pieces[0][2];

    self.player_pieces[0][3] = Some(ChessPiece::new_queen(0, 0, 3));
    self.board[0][3] = self.player_pieces[0][3];

    self.player_pieces[0][4] = Some(ChessPiece::new_king(0, 0, 4));
    self.board[0][4] = self.player_pieces[0][4];

    self.player_pieces[0][5] = Some(ChessPiece::new_bishop(0, 0, 5));
    self.board[0][5] = self.player_pieces[0][5];

    self.player_pieces[0][6] = Some(ChessPiece::new_knight(0, 0, 6));
    self.board[0][6] = self.player_pieces[0][6];

    self.player_pieces[0][7] = Some(ChessPiece::new_rook(0, 0, 7));
    self.board[0][7] = self.player_pieces[0][7];

    self.player_pieces[0][8] = Some(ChessPiece::new_pawn(0,1, 0));
    self.board[1][1] = self.player_pieces[0][8];

    self.player_pieces[0][9] = Some(ChessPiece::new_pawn(0,1, 1));
    self.board[1][1] = self.player_pieces[0][9];

    self.player_pieces[0][10] = Some(ChessPiece::new_pawn(0,1, 2));
    self.board[1][2] = self.player_pieces[0][10];

    self.player_pieces[0][11] = Some(ChessPiece::new_pawn(0,1, 3));
    self.board[1][3] = self.player_pieces[0][11];

    self.player_pieces[0][12] = Some(ChessPiece::new_pawn(0,1, 4));
    self.board[1][4] = self.player_pieces[0][12];

    self.player_pieces[0][13] = Some(ChessPiece::new_pawn(0,1, 5));
    self.board[1][5] = self.player_pieces[0][13];

    self.player_pieces[0][14] = Some(ChessPiece::new_pawn(0,1, 6));
    self.board[1][6] = self.player_pieces[0][14];

    self.player_pieces[0][15] = Some(ChessPiece::new_pawn(0,1, 7));
    self.board[1][7] = self.player_pieces[0][15];

    //  Player 2: fill rows 6 and 7
    self.player_pieces[1][0] = Some(ChessPiece::new_pawn(1,6, 0));
    self.board[6][0] = self.player_pieces[1][0];

    self.player_pieces[1][1] = Some(ChessPiece::new_pawn(1,6, 1));
    self.board[6][1] = self.player_pieces[1][1];

    self.player_pieces[1][2] = Some(ChessPiece::new_pawn(1,6, 2));
    self.board[6][2] = self.player_pieces[1][2];

    self.player_pieces[1][3] = Some(ChessPiece::new_pawn(1,6, 3));
    self.board[6][3] = self.player_pieces[1][3];

    self.player_pieces[1][4] = Some(ChessPiece::new_pawn(1,6, 4));
    self.board[6][4] = self.player_pieces[1][4];

    self.player_pieces[1][5] = Some(ChessPiece::new_pawn(1,6, 5));
    self.board[6][5] = self.player_pieces[1][5];

    self.player_pieces[1][6] = Some(ChessPiece::new_pawn(1,6, 6));
    self.board[6][6] = self.player_pieces[1][6];

    self.player_pieces[1][7] = Some(ChessPiece::new_pawn(1,6, 7));
    self.board[6][7] = self.player_pieces[1][7];

    self.player_pieces[1][8] = Some(ChessPiece::new_rook(1,7, 0));
    self.board[7][0] = self.player_pieces[1][8];

    self.player_pieces[1][9] = Some(ChessPiece::new_knight(1, 7, 1));
    self.board[7][1] = self.player_pieces[1][9];

    self.player_pieces[1][10] = Some(ChessPiece::new_bishop(1, 7, 2));
    self.board[7][2] = self.player_pieces[1][10];

    self.player_pieces[1][11] = Some(ChessPiece::new_queen(1, 7, 3));
    self.board[7][3] = self.player_pieces[1][11];

    self.player_pieces[1][12] = Some(ChessPiece::new_king(1, 7, 4));
    self.board[7][4] = self.player_pieces[1][12];

    self.player_pieces[1][13] = Some(ChessPiece::new_bishop(1, 7, 5));
    self.board[7][5] = self.player_pieces[1][13];

    self.player_pieces[1][14] = Some(ChessPiece::new_knight(1, 7, 6));
    self.board[7][6] = self.player_pieces[1][14];

    self.player_pieces[1][15] = Some(ChessPiece::new_rook(1, 7, 7));
    self.board[7][7] = self.player_pieces[1][15];

    Ok(())
  }

  fn in_check(&self, team_attacking: u8) -> bool {

    let king_team = if 1 == team_attacking {
        0
    } else {
        1
    };

    //  Find the king.
    let king_piece = self.player_pieces[king_team][4].unwrap();
    let _king_row = king_piece.x_loc;
    let _king_col = king_piece.y_loc;

    false
  }

  fn moving_into_check(&self, _chess_move: &Move) -> bool {
    false
  }

  pub fn can_execute_move(&self, piece: &ChessPiece, chess_move: &Move, _occupied: bool) -> bool { 
    //  Under construction

    let mut _move_list: Vec<Move> = Vec::new();
    
    if self.in_check(piece.team) {
        return false;
    }

    if self.moving_into_check(chess_move) {
        return false;
    }
    match piece.piece_type {
      ChessPieceTypes::Pawn => {
        //  Pawn move rules:
        //  Cannot move if king is in check state unless the move removes the check state
        //  Can advance by one or two spaces vertically if at initial location.
        //  Can advance diagonally only if there is an enemy piece other than the king there.
        //  Can advance by one space if not at initial location
        //  Move cannot result in king in check state
      }
      ChessPieceTypes::Knight => {
        //  Cannot move if king is in check state unless the move removes the check state
        //  Can move in following patterns:
        //  x,y <- x+2,y+1
        //      <- x-2,y-1
        //      <- x+2,y-1
        //      <- x-2,y+1
        //  Move cannot result in king in check state
      }
      ChessPieceTypes::Bishop => {
        //  Cannot move if king is in check state unless the move removes the check state
        //  Can move in the following pattern:
        //  x,y <- a+x,a+y (-7 <= a <= 7)
        //  Move cannot result in king in check state
      }
      ChessPieceTypes::Rook => {
        //  Cannot move if king is in check state unless the move removes the check state
        //  Can move in the following patterns:
        //  x,y <- a+x,y (-7 <= a <= 7)
        //  x,y <- x,a+y (-7 <= a <= 7)
        //  Move cannot result in king in check state
      }
      ChessPieceTypes::Queen => {
        //  Cannot move if king is in check state unless the move removes the check state
        //  Can move in the following patterns:
        //  x,y <- a+x,y (-7 <= a <= 7)
        //  x,y <- x,a+y (-7 <= a <= 7)
        //  x,y <- a+x,a+y (-7 <= a <= 7)
        //  Move cannot result in king in check state
      }
      ChessPieceTypes::King => {
        //  Cannot move if king is in check state unless the move removes the check state
        //  Can move in following pattern:
        //  x,y <- a+x,y (-1 <= a <= 1)
        //  x,y <- x,a+y (-1 <= a <= 1)
        //  x,y <- a+x,a+y (-1 <= a <= 1)
        //  Move cannot result in king in check state
      }
    }
    true
  }

  pub fn remove_piece(&mut self, piece: &ChessPiece, make_move: &Move) -> Result<()> {
    let team = piece.team; 
    let score_team = if team == 1 {
      0
    } else {
      1
    };

    //  Iterate through team and remove the piece located at make_move.
    for _i in 0..8 {
      if let Some(piece) = self.board[make_move.x as usize][make_move.y as usize] {
        if piece.x_loc == make_move.x && piece.y_loc == make_move.y {
          self.board[make_move.x as usize][make_move.y as usize] = None;

          self.players[score_team].score += ChessPieceTypes::value(&piece.piece_type);
        }
      } else {
      }
    }
    Err(error!(ChessError::RemoveError))
  }

}

