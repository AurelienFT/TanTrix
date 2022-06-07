import Moveable from "react-moveable";
import Draggable from "react-draggable";
import React, { useRef } from "react";
import "./Tile.css"

function Tile(props: {tile_image: string, id: string}) {

    const [rotate, setRotate] = React.useState(false);
    const ref1 = useRef(null);
    const [frame, setFrame] = React.useState({
        translate: [0, 0],
        rotate: 0,
        transformOrigin: "50% 50%"
      });
    console.log(rotate);
    return (
    <div>
     <Moveable
        target={ref1}
        origin={false}
        draggable={true}
        throttleDrag={0}
        startDragRotate={0}
        throttleDragRotate={0}
        zoom={1}
        padding={{ left: 0, top: 0, right: 0, bottom: 0 }}
        rotatable={true}
        throttleRotate={0}
        hideDefaultLines={true}
        rotationPosition={"top"}
        onDragOriginStart={(e) => {
          e.dragStart && e.dragStart.set(frame.translate);
        }}
        onDragOrigin={(e) => {
          frame.translate = e.drag.beforeTranslate;
          frame.transformOrigin = e.transformOrigin;
        }}
        onDragStart={(e) => {
          e.set(frame.translate);
        }}
        onDrag={(e) => {
          frame.translate = e.beforeTranslate;
        }}
        onRotateStart={(e) => {
          e.set(frame.rotate);
        }}
        onRotate={(e) => {
          frame.rotate = e.beforeRotate;
        }}
        onRender={(e) => {
          const { translate, rotate, transformOrigin } = frame;
          e.target.style.transformOrigin = transformOrigin;
          e.target.style.transform =
            `translate(${translate[0]}px, ${translate[1]}px)` +
            ` rotate(${rotate}deg)`;
        }}
      />
        <img ref={ref1} src={props.tile_image} alt="Tantrix"  className={"Tantrix-tile"} />
    </div>
    )
}

export default Tile;