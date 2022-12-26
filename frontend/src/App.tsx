import './App.css';
import Tile1 from './tiles/1.png';
import Tile3 from './tiles/2.png';
import Tile2 from './tiles/3.png';
import Tile from './Tile';
import { submit } from './api';

function App() {

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
