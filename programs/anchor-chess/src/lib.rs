use anchor_lang::prelude::*;
//use num_traits::*;
use num_derive::*;

declare_id!("8nS2cyFkip1yibKmJXkzhy6NkVoabUhdLQa9CwhdNWUp");

#[program]
pub mod anchor_chess {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey ) -> Result<()> {
        ctx.accounts.game.start([ctx.accounts.player_one.key(), player_two])
    }

    pub fn play(
      ctx: Context<Play>,
      piece: ChessPiece,
      make_move: Move) -> Result<()> {
        ctx.accounts.game.play(piece, make_move)
      }
}

#[account]
pub struct ChessGame {
  players: [Pubkey; 2],
  turn: u16,
  board: [ [Option<ChessPiece>; 8]; 8],
  state: GameState,
}

impl<'info> ChessGame {
  pub const MAXIMUM_SIZE: usize = (32*2) + 1 + (64*4*4) + (32 + 1);

  pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
    self.players = players;
    self.turn = 1;
    self.initialize_board()?;
    Ok(())
  }

  /// Player passes in the chess piece they
  /// want to move and a move coordinate
  /// where they want to move.
  pub fn play(
    &mut self, 
    chess_piece: ChessPiece,
    make_move: Move
  ) -> Result<()> {
    msg!("Running PLAY with Piece {:?} and Move {:?}", &chess_piece, &make_move);
    if make_move.x > 7  || make_move.y > 7 {
      return Err(error!(ChessError::MoveOutOfBounds))
    } 

    let player_index = if self.turn % 2 > 0 {
      0
    } else {
      1
    };

    //  Bail if same team.  This is a fast calculation so perform this before
    //  any O(n) type of stuff.
    if chess_piece.team == player_index {
      if let Some(x) = self.board[make_move.x as usize][make_move.y as usize] {
        if x.team == player_index {
          return Err(error!(ChessError::SpotOccupied));
        }
      }
    } else {
      return Err(error!(ChessError::NotPlayersTurn));
    }

    if let Some(enemy_piece) = self.board[make_move.x as usize][make_move.y as usize] {
      //  We already know it's either empty or occupied by the other team.
      
      //  Determine if the move is legal for this chess piece at its current location.
      if self.can_execute_move(&chess_piece, &make_move, true) {
        self.board[make_move.x as usize][make_move.y as usize] = Some(chess_piece);
        if let Some(mut cell) = self.board[make_move.x as usize][make_move.y as usize] {
          cell.x_loc = make_move.x;
          cell.y_loc = make_move.y;
        }

        //  Remove the enemy_piece from the board.
        self.remove_piece(&enemy_piece)?;
      } else {
        return Err(error!(ChessError::MoveNotPossible));    
      }
    } else {
      self.board[make_move.x as usize][make_move.y as usize] = Some(chess_piece);
      self.board[chess_piece.x_loc as usize][chess_piece.y_loc as usize] = None;

      if let Some(mut cell) = self.board[make_move.x as usize][make_move.y as usize] {
        cell.x_loc = make_move.x;
        cell.y_loc = make_move.y;
      }

      if let Some(cell) = self.board[make_move.x as usize][make_move.y as usize] {
        msg!("After PLAY at {},{} = {:?}", make_move.x, make_move.y, &cell);
      }
    }

    self.turn += 1;
    Ok(())
  }

  fn remove_piece(&mut self, _piece: &ChessPiece) -> Result<()> {
    //  Under construction
    Ok(())
  }
  fn can_execute_move(&self, piece: &ChessPiece, _chess_move: &Move, _occupied: bool) -> bool { 
    //  Under construction

    let mut _move_list: Vec<Move> = Vec::new();
    
    match piece.piece_type {
      ChessPieceTypes::Pawn => {
        //  Pawn move rules:
        //  Can advance by one or two spaces vertically if at initial location.
        //  Can advance diagonally only if there is an enemy piece other than the king there.
        //  Can advance by one space if not at initial location
      }
      ChessPieceTypes::Knight => {
        //  Can move in following patterns:
        //  x,y <- x+2,y+1
        //      <- x-2,y-1
        //      <- x+2,y-1
        //      <- x-2,y+1
      }
      ChessPieceTypes::Bishop => {
        //  Can move in the following pattern:
        //  x,y <- a+x,a+y (-7 <= a <= 7)
      }
      ChessPieceTypes::Rook => {
        //  Can move in the following patterns:
        //  x,y <- a+x,y (-7 <= a <= 7)
        //  x,y <- x,a+y (-7 <= a <= 7)
      }
      ChessPieceTypes::Queen => {
        //  Can move in the following patterns:
        //  x,y <- a+x,y (-7 <= a <= 7)
        //  x,y <- x,a+y (-7 <= a <= 7)
        //  x,y <- a+x,a+y (-7 <= a <= 7)
      }
      ChessPieceTypes::King => {
        //  Can move in following pattern:
        //  x,y <- a+x,y (-1 <= a <= 1)
        //  x,y <- x,a+y (-1 <= a <= 1)
        //  x,y <- a+x,a+y (-1 <= a <= 1)
      }
    }
    true
  }
  fn initialize_board(&mut self) -> Result<()> {
    //  Start with empty grid and populate the initial locations
    //  with the appropriate piece

    //  Player 1: fill rows 0 and 1
    self.board[0][0] = Some(ChessPiece::new_rook(0,Some(0), Some(0)));
    self.board[0][1] = Some(ChessPiece::new_knight(0, Some(0), Some(1)));
    self.board[0][2] = Some(ChessPiece::new_bishop(0, Some(0), Some(2)));
    self.board[0][3] = Some(ChessPiece::new_queen(0, Some(0), Some(3)));
    self.board[0][4] = Some(ChessPiece::new_king(0, Some(0), Some(4)));
    self.board[0][5] = Some(ChessPiece::new_bishop(0, Some(0), Some(5)));
    self.board[0][6] = Some(ChessPiece::new_knight(0, Some(0), Some(7)));
    self.board[0][7] = Some(ChessPiece::new_rook(0, Some(0), Some(7)));
    self.board[1][0] = Some(ChessPiece::new_pawn(0,Some(1), Some(0)));
    self.board[1][1] = Some(ChessPiece::new_pawn(0,Some(1), Some(1)));
    self.board[1][2] = Some(ChessPiece::new_pawn(0,Some(1), Some(2)));
    self.board[1][3] = Some(ChessPiece::new_pawn(0,Some(1), Some(3)));
    self.board[1][4] = Some(ChessPiece::new_pawn(0,Some(1), Some(4)));
    self.board[1][5] = Some(ChessPiece::new_pawn(0,Some(1), Some(5)));
    self.board[1][6] = Some(ChessPiece::new_pawn(0,Some(1), Some(6)));
    self.board[1][7] = Some(ChessPiece::new_pawn(0,Some(1), Some(7)));

    self.board[2] = [None; 8];
    self.board[3] = [None; 8];
    self.board[4] = [None; 8];
    self.board[5] = [None; 8];

    //  Player 2: fill rows 6 and 7
    self.board[6][0] = Some(ChessPiece::new_pawn(1,Some(6), Some(0)));
    self.board[6][1] = Some(ChessPiece::new_pawn(1,Some(6), Some(1)));
    self.board[6][2] = Some(ChessPiece::new_pawn(1,Some(6), Some(2)));
    self.board[6][3] = Some(ChessPiece::new_pawn(1,Some(6), Some(3)));
    self.board[6][4] = Some(ChessPiece::new_pawn(1,Some(6), Some(4)));
    self.board[6][5] = Some(ChessPiece::new_pawn(1,Some(6), Some(5)));
    self.board[6][6] = Some(ChessPiece::new_pawn(1,Some(6), Some(6)));
    self.board[6][7] = Some(ChessPiece::new_pawn(1,Some(6), Some(7)));
    self.board[7][0] = Some(ChessPiece::new_rook(1,Some(7), Some(0)));
    self.board[7][1] = Some(ChessPiece::new_knight(1, Some(7), Some(1)));
    self.board[7][2] = Some(ChessPiece::new_bishop(1, Some(7), Some(2)));
    self.board[7][3] = Some(ChessPiece::new_queen(1, Some(7), Some(3)));
    self.board[7][4] = Some(ChessPiece::new_king(1, Some(7), Some(4)));
    self.board[7][5] = Some(ChessPiece::new_bishop(1, Some(7), Some(5)));
    self.board[7][6] = Some(ChessPiece::new_knight(1, Some(7), Some(6)));
    self.board[7][7] = Some(ChessPiece::new_rook(1, Some(7), Some(7)));

    Ok(())
  }
}
  
#[derive(Accounts)]
pub struct SetupGame<'info> {
  #[account(init, payer = player_one, space = 8 + ChessGame::MAXIMUM_SIZE)]
  pub game:  Account<'info, ChessGame>,
  #[account(mut)]
  pub player_one: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Play<'info> {
  #[account(mut)]
  pub game:  Account<'info, ChessGame>,
  pub player: Signer<'info>,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Stalemate,
    Won {winner: Pubkey},
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

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub struct ChessPiece {
    team: u8,
    piece_type: ChessPieceTypes,
    x_loc: u8,
    y_loc: u8,
}

impl ChessPiece {
  pub fn new_pawn(team: u8, x: Option<u8>, y: Option<u8>) -> ChessPiece {
    let x_pos = x.unwrap_or_default();
    let y_pos = y.unwrap_or_default();
    ChessPiece { team: team, piece_type: ChessPieceTypes::Pawn, x_loc: x_pos, y_loc: y_pos}
  }

  pub fn new_knight(team: u8, x: Option<u8>, y: Option<u8>) -> ChessPiece {
    let x_pos = x.unwrap_or_default();
    let y_pos = y.unwrap_or_default();
    ChessPiece { team: team, piece_type: ChessPieceTypes::Knight, x_loc: x_pos, y_loc: y_pos}
  }

  pub fn new_bishop(team: u8, x: Option<u8>, y: Option<u8>) -> ChessPiece {
    let x_pos = x.unwrap_or_default();
    let y_pos = y.unwrap_or_default();
    ChessPiece { team: team, piece_type: ChessPieceTypes::Bishop, x_loc: x_pos, y_loc: y_pos}
  }

  pub fn new_rook(team: u8, x: Option<u8>, y: Option<u8>) -> ChessPiece {
    let x_pos = x.unwrap_or_default();
    let y_pos = y.unwrap_or_default();
    ChessPiece { team: team, piece_type: ChessPieceTypes::Rook, x_loc: x_pos, y_loc: y_pos}
  }

  pub fn new_queen(team: u8, x: Option<u8>, y: Option<u8>) -> ChessPiece {
    let x_pos = x.unwrap_or_default();
    let y_pos = y.unwrap_or_default();
    ChessPiece { team: team, piece_type: ChessPieceTypes::Queen, x_loc: x_pos, y_loc: y_pos}
  }

  pub fn new_king(team: u8, x: Option<u8>, y: Option<u8>) -> ChessPiece {
      let x_pos = x.unwrap_or_default();
      let y_pos = y.unwrap_or_default();
      ChessPiece {team: team, piece_type: ChessPieceTypes::King, x_loc: x_pos, y_loc: y_pos}
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Move {
  x: u8,
  y: u8
}

#[error_code]
pub enum ChessError {
    #[msg("Invalid player")]
    InvalidPlayer,
    #[msg("Cannot move chess piece off of the board")]
    MoveOutOfBounds,
    #[msg("Cannot move chess piece in this scenario")]
    MoveNotPossible,
    #[msg("Cannot move to an occupied spot")]
    SpotOccupied,
    #[msg("Cannot move out of turn")]
    NotPlayersTurn,
    #[msg("Cannot start game twice")]
    GameAlreadyStarted
}
