import "./App.css";
import {useEffect, useState} from "preact/hooks";
import {invoke} from "@tauri-apps/api/core";

function App() {
  const [gameBoard, setGameBoard] = useState<any>();
  const [currentPiece,setPiece] = useState("Cross");
  const [winner,setWinner] = useState("Empty");
  const togglePiece = ()=>{
    switch (currentPiece){
      case "Cross": setPiece("Nought"); break;
      case "Nought": setPiece("Cross"); break;
    }
  }
  useEffect(() => {
    (async () => {
      await invoke('reset_game');
      await updateGameBoard();
      console.log('wow')
      console.log(gameBoard)
    })()
    return () => {

    }
  }, [])
  const updateGameBoard = async () => {
    setGameBoard(await invoke('get_game_board'))
  }
  return (
    <main class="container" style={{display:"flex",flexDirection:"column",alignItems:"center"}}>
      <button onClick={async ()=>{
        await invoke('reset_game');
        await updateGameBoard();
      }}>Reset</button>
      <div style={{
        display: "grid",
        gridTemplateColumns: "1fr 1fr 1fr",
        width: "320px",
        // marginLeft: "auto",
        // marginRight: "auto"
      }}>
        {(() => {
          let vals = [];
          for (let i = 0; i < 9; i++) {
            let di = <div key={i} onClick={
              async () => {
                await invoke('play_move', {y: (i % 3), x: (Math.floor(i / 3)), piece: currentPiece})
                togglePiece();
                setWinner(await invoke('check_winner'))
                setGameBoard(await invoke('get_game_board'))
              }
            } style={{
              margin: "5px",
              width: "100px",
              height: "100px",
              fontSize:"100px",
              lineHeight:"100px",
              textAlign:"center",
              border: "solid black 2px",
              justifySelf: "center",
              alignSelf: "center"
            }}>{translatePieceToSymbol(gameBoard?.board[i % 3][Math.floor(i / 3)])}</div>
            vals.push(di);
          }
          return vals;
        })()}
      </div>
      <div>{winner!="Empty"?winner:""}</div>
    </main>
  );
}

const translatePieceToSymbol = (piece: string): string => {
  switch (piece) {
    case "Cross":
      return "X";
    case "Nought":
      return "O";
    case "Empty":
      return " ";
    default:
      return " "
  }
}

export default App;
