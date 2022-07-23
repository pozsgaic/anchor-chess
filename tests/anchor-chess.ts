import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorChess } from "../target/types/anchor_chess";
import { expect } from 'chai';

console.log("Testing AnchorChess");
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.AnchorChess as Program<AnchorChess>;
console.log("Got program ");

const playerOne = (program.provider as anchor.AnchorProvider).wallet;
console.log("Got playerOne ");

const playerTwo = anchor.web3.Keypair.generate();
console.log("Got playerTwo ");

const gameKey = anchor.web3.Keypair.generate();
console.log("Got gameKey ");
// async function play(program: Program<AnchorChess>,
//                     game,
//                     playerKey,
//                     player,
//                     xFrom, yFrom,
//                     xTo, yTo) {
//   await program.methods
//     .play(player, xFrom, xTo, yFrom, yTo)
//     .accounts({
//         player: playerKey,
//         game: game,
//     })
//     .rpc();
//   const gameState = program.account.chessGame.fetch(game);
// }
var moveNo = 1;
describe("anchor-chess", async () => {
  // Configure the client to use the local cluster.

  it("Chess game is starting", async () => {
    // Add your test here.
    await program.methods.setupGame(playerTwo.publicKey)
      .accounts({
        game: gameKey.publicKey,
        playerOne: playerOne.publicKey,
      })
      .signers([gameKey])
      .rpc();
     
    let theGame = await program.account.chessGame.fetch(gameKey.publicKey);
    
    console.log("INITIAL BOARD LAYOUT (0 - team 1, 1 - team 2, X - EMPTY)");
    for(let i = 0; i < 8; ++i) {
      let line = "";
      for(let j = 0; j < 8; ++j) {
        if(`${theGame.board[i][j]}` == "null") {
          line = line + "X ";
        } else {
          line = line + `${theGame.board[i][j].team} `;
        }
      }
      console.log(line);
    }
          
  });


  it("Move " + `${moveNo}`, async () => {
    let thePiece = {team: 0, pieceType: {knight:{}}, xLoc: 0, yLoc: 1};
    let theMove =  {x: 2, y: 2};
    await program.methods.play(thePiece, theMove)
    .accounts({
      game: gameKey.publicKey,
    })
    .rpc();

    let theGame = await program.account.chessGame.fetch(gameKey.publicKey);
    
    console.log("turn = " + `${theGame.turn - 1}`);
    
    for(let i = 0; i < 8; ++i) {
      let line = "";
      for(let j = 0; j < 8; ++j) {
        if(`${theGame.board[i][j]}` == "null") {
          line = line + "X ";
        } else {
          line = line + `${theGame.board[i][j].team} `;
        }
      }
      console.log(line);
    }
  });

  moveNo++;

  it("Move " + `${moveNo}`, async () => {
    let thePiece = {team: 1, pieceType: {knight:{}}, xLoc: 7, yLoc: 1};
    let theMove =  {x: 5, y: 2};
    await program.methods.play(thePiece, theMove)
    .accounts({
      game: gameKey.publicKey,
    })
    .rpc();

    let theGame = await program.account.chessGame.fetch(gameKey.publicKey);
    
    console.log("turn = " + `${theGame.turn - 1}`);
    
    for(let i = 0; i < 8; ++i) {
      let line = "";
      for(let j = 0; j < 8; ++j) {
        if(`${theGame.board[i][j]}` == "null") {
          line = line + "X ";
        } else {
          line = line + `${theGame.board[i][j].team} `;
        }
      }
      console.log(line);
    }
  });

  moveNo++;
  it("Move " + `${moveNo}`, async () => {
    let thePiece = {team: 0, pieceType: {knight:{}}, xLoc: 0, yLoc: 6};
    let theMove =  {x: 2, y: 5};
    await program.methods.play(thePiece, theMove)
    .accounts({
      game: gameKey.publicKey,
    })
    .rpc();

    let theGame = await program.account.chessGame.fetch(gameKey.publicKey);
    
    console.log("turn = " + `${theGame.turn - 1}`);
    
    for(let i = 0; i < 8; ++i) {
      let line = "";
      for(let j = 0; j < 8; ++j) {
        if(`${theGame.board[i][j]}` == "null") {
          line = line + "X ";
        } else {
          line = line + `${theGame.board[i][j].team} `;
        }
      }
      console.log(line);
    }
  });

  moveNo++;

  it("Move " + `${moveNo}`, async () => {
    let thePiece = {team: 1, pieceType: {pawn:{}}, xLoc: 6, yLoc: 4};
    let theMove =  {x: 4, y: 4};
    await program.methods.play(thePiece, theMove)
    .accounts({
      game: gameKey.publicKey,
    })
    .rpc();

    let theGame = await program.account.chessGame.fetch(gameKey.publicKey);
    
    console.log("turn = " + `${theGame.turn - 1}`);
    
    for(let i = 0; i < 8; ++i) {
      let line = "";
      for(let j = 0; j < 8; ++j) {
        if(`${theGame.board[i][j]}` == "null") {
          line = line + "X ";
        } else {
          line = line + `${theGame.board[i][j].team} `;
        }
      }
      console.log(line);
    }
  });

});
