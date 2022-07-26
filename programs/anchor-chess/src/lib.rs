use anchor_lang::prelude::*;

mod chess_error;
mod chess_model;
use chess_error::ChessError;
use chess_model::*;

declare_id!("8nS2cyFkip1yibKmJXkzhy6NkVoabUhdLQa9CwhdNWUp");

#[program]
pub mod anchor_chess {
  //pub mod chess_error;
  use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey ) -> Result<()> {
        let players = [ctx.accounts.player_one.key(), player_two];
        ctx.accounts.game.start(&players)
    }

    pub fn make_move(
      ctx: Context<MakeMove>,
      piece: ChessPiece,
      move_to: Move) -> Result<()> {
        ctx.accounts.game.make_move(piece, move_to)
      }
}

#[account]
pub struct ChessGame {
  turn: u16,
  board: ChessBoard,
  state: GameState,
}

impl ChessGame {
  pub const MAXIMUM_SIZE: usize = (32*2) + 1 + (64*4*4) + (32 + 1);

  pub fn start(&mut self, players: &[Pubkey]) -> Result<()> {
    let player1 = ChessPlayer {key: players[0], team: 0, score: 0};
    let player2 = ChessPlayer {key: players[1], team: 1, score:0};

    self.board.players = [player1, player2];
    self.turn = 1;
    self.board.initialize()?;

    Ok(())
  }

  /// Player passes in the chess piece they
  /// want to move and a move coordinate
  /// where they want to move.
  pub fn make_move(
    &mut self, 
    chess_piece: ChessPiece,
    move_to: Move
  ) -> Result<()> {
    
    msg!("Running PLAY with Piece {:?} and Move {:?}", &chess_piece, &move_to);
    
    //  Ensure the chess piece selected matches what is on the board.
    if let Some(check_piece) = self.board.board[chess_piece.x_loc as usize][chess_piece.y_loc as usize] {
      if check_piece.x_loc != chess_piece.x_loc || check_piece.y_loc != chess_piece.y_loc {
        return Err(error!(ChessError::InvalidPiece));
      }
    } else {
      return Err(error!(ChessError::InvalidPiece));
    }

    //  Ensure move to coordinates are valid.
    if move_to.x > 7  || move_to.y > 7 {
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
      if let Some(x) = self.board.board[move_to.x as usize][move_to.y as usize] {
        if x.team == player_index {
          return Err(error!(ChessError::SpotOccupied));
        }
      }
    } else {
      return Err(error!(ChessError::NotPlayersTurn));
    }

    if let Some(enemy_piece) = self.board.board[move_to.x as usize][move_to.y as usize] {
      //  We already know it's either empty or occupied by the other team.
      
      //  Determine if the move is legal for this chess piece at its current location.
      if self.board.can_execute_move(&chess_piece, &move_to, true) {
        self.board.board[move_to.x as usize][move_to.y as usize] = Some(chess_piece);
        if let Some(mut cell) = self.board.board[move_to.x as usize][move_to.y as usize] {
          cell.x_loc = move_to.x;
          cell.y_loc = move_to.y;
        }

        //  Remove the enemy_piece from the board.
        self.board.remove_piece(&enemy_piece, &move_to)?;
      } else {
        return Err(error!(ChessError::MoveNotPossible));    
      }
    } else {
      self.board.board[move_to.x as usize][move_to.y as usize] = Some(chess_piece);
      self.board.board[chess_piece.x_loc as usize][chess_piece.y_loc as usize] = None;

      if let Some(mut cell) = self.board.board[move_to.x as usize][move_to.y as usize] {
        cell.x_loc = move_to.x;
        cell.y_loc = move_to.y;
      }

      if let Some(cell) = self.board.board[move_to.x as usize][move_to.y as usize] {
        msg!("After PLAY at {},{} = {:?}", move_to.x, move_to.y, &cell);
      }
    }

    self.turn += 1;
    msg!("After PLAY - turn = {}", self.turn);
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
pub struct MakeMove<'info> {
  #[account(mut)]
  pub game:  Account<'info, ChessGame>,
  pub player: Signer<'info>,
}


