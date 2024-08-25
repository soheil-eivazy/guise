import { Maze, GraphSearchMethod } from "@/workshop/sketches"
import React, { useEffect, useRef } from 'react';


const MazeRunner = () => {

    const renderRef = useRef();
    

    useEffect(() => {
        const p5 = require("p5");
        const { Maze, GraphSearchMethod } = require("@/workshop/sketches")

        // const  = wasm.Maze
        // const GraphSearchMethod = wasm.GraphSearchMethod

        let mazeWidth: number
        let mazeHeight: number
        let cellWidth: number
        let cellHeight: number
        let maze
        let plane: Uint8Array

        const getIndex = (row: number, column: number) => {
            return row * mazeWidth + column;
        };

        new p5(p => {
            p.setup = (canvasParentRef: Element) => {

                p.frameRate(30);
                let size = window.innerWidth > window.innerHeight ? window.innerHeight : window.innerWidth
                size = size - (size % 100)
                p.createCanvas(size, size).parent(canvasParentRef)

                mazeWidth = 100
                mazeHeight = 100
                cellWidth = size / mazeWidth
                cellHeight = size / mazeHeight

                maze = Maze.new(mazeWidth, mazeHeight, GraphSearchMethod.BreadthFirst)
                plane = maze.flat_plane()
                p.draw_plane(p5)
            }

            p.draw = () => {
                p.print("draw!!!")
                let step: Uint16Array = maze.run()
                if (step.length == 0) {
                    p.noLoop()
                    return 
                }

                if (step[0] == 1) {
                    p.draw_plane(p5)
                    p.draw_solved(p5, step)
                    
                    p.noLoop()
                } else {
                    p.draw_steps(p5, step)
                }
                // const cellsPtr = maze.cells();
            }

            p.draw_steps = (step: Uint16Array) => {
                p.stroke(255, 0, 0)
                p.strokeWeight(cellHeight / 2)
                for (let i = 1; i < step.length;) {
                    let x = step[i];
                    let y = step[++i];
                    let x1 = step[++i];
                    let y1 = step[++i];
                    
                    p.line(
                        (x*cellWidth)  + (cellWidth  ), 
                        (y*cellHeight) + (cellHeight / 2), 
                        (x1*cellWidth) + (cellWidth  ), 
                        (y1*cellHeight)+ (cellHeight / 2)
                    )

                    i++
                }
            }

            p.draw_solved = (step: Uint16Array) => {
                p.stroke(255, 0, 0)
                p.strokeWeight(cellHeight / 2)
                for (let i = 1; i < step.length;) {
                    let x = step[i];
                    let y = step[++i];
                    let x1 = step[++i];
                    let y1 = step[i+1];
                    
                    p.line(
                        (x*cellWidth)  + (cellWidth  ), 
                        (y*cellHeight) + (cellHeight / 2), 
                        (x1*cellWidth) + (cellWidth  ), 
                        (y1*cellHeight)+ (cellHeight / 2)
                    )
                }

                p.print(step)
            }

            p.draw_plane = () => {
                p.background(0)
                p.stroke(255, 255, 255)
                p.strokeWeight(0)
                for (let row = 0; row < mazeHeight; row++) {
                    for (let col = 0; col < mazeWidth; col++) {

                        const idx = getIndex(row, col);
                        if (plane[idx] ==  1) {
                            p.fill(255, 255, 255)
                        } else {
                            p.fill(0, 0, 0)
                        }

                        p.rect((col * cellWidth) + (cellWidth / 2), row * cellHeight, cellWidth, cellHeight)
                    }
                }

                let fl = maze.first_and_last()
                p.fill(255, 255, 0)
                p.rect((fl[0] * cellWidth) + (cellWidth / 2), fl[1] * cellHeight, cellWidth, cellHeight)
                p.fill(255, 0, 2550)
                p.rect((fl[2] * cellWidth) + (cellWidth / 2), fl[3] * cellHeight, cellWidth, cellHeight)
            }
        })
    }, [])

    return(
        <div ref={renderRef}></div>
    )
}

export default MazeRunner