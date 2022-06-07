import React from 'react';
import logo from './logo.svg';
import './App.css';
import Draggable from 'react-draggable'; // The default
import Tile1 from './tiles/1.png';
import Tile3 from './tiles/2.png';
import Tile2 from './tiles/3.png';
import Tile from './Tile';

function App() {

  return (
    <div className="App">
      <header className="App-header">
        Tantrix
      </header>
      <Tile id={"1"} tile_image={Tile1} />
      <Tile id={"1"} tile_image={Tile2} />
      <Tile id={"1"} tile_image={Tile3} />
    </div>
  );
}

export default App;
