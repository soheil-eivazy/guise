import React, { useEffect, useRef } from 'react';

var x = 50;
var speed = 5;
const P5Sketch = () => {
    const renderRef = useRef();

    useEffect(() => {
        const p5 = require("p5");

        new p5(p => {
            p.setup = () => {
                p.createCanvas(500, 400).parent(renderRef.current);
            }

            p.draw = () => {
                p.frameRate(30);
                // If we're travelling towards the right or left
                if (speed > 0) {
                    // If the ball has reached the end of the container or not
                    if (x + 50 < p.width) {
                        x += speed
                    } else {
                        speed = -speed;
                    }
                } else {
                    if (x - 50 > 0) {
                        x += speed;
                    } else {
                        speed = -speed;
                    }
                }
        
                p.background(255, 120, 20);
                p.ellipse(x, 100, 100);
            }
        })
    }, [])

    return(
        <div ref={renderRef}></div>
    )
}

export default P5Sketch;