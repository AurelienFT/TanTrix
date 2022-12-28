import './App.css';
import Tile1 from './tiles/1.png';
import Tile3 from './tiles/2.png';
import Tile2 from './tiles/3.png';
import Tile from './Tile';
import { submit, getDailyGame } from './api';
import { useEffect, useState } from 'react';

function App() {
  const [tiles, setTiles] = useState(null);

  useEffect(() => {
    const fetchTiles = async () => {
      let res = await getDailyGame();
      setTiles(await res.json());
    };
    //fetchTiles();
  });
  return (
    <div className="App">
      <header className="App-header">
        Tantrix
      </header>
      <Tile id={"1"} tile_image={Tile1} />
      <Tile id={"1"} tile_image={Tile2} />
      <Tile id={"1"} tile_image={Tile3} />
      <div className='footer-container'>
        <div className='footer'>
          <button onClick={() => submit()}>
            Submit
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
