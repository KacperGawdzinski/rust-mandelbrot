import { Mandelbrot } from "mandelbrot-animated";
import { memory } from "mandelbrot-animated/mandelbrot_animated_bg.wasm";
import Stats from "stats.js";

//await init();
//await initThreadPool(navigator.hardwareConcurrency)

//var stats = new Stats();
//stats.showPanel(0); // 0: fps, 1: ms, 2: mb, 3+: custom
//document.body.appendChild(stats.dom);

const canvas = document.getElementById("mandelbrot");
const ctx = canvas.getContext("2d");

const width = 1024;
const height = 1024;
canvas.width = width;
canvas.height = height;

const render = async () => {
    let scale = 200.0;
    // let centerX = 0.0;
    // let centerY = 1.0;
    let centerX = -2;
    let centerY = 0.0;
    // let centerX = 0.0;
    // let centerY = 0.0;

    const maxIter = 255;
    let pixels = null;

    const mandelbrot = new Mandelbrot(width, height, maxIter);

    const animate = () => {
        //stats.begin();

        //console.log("Iteracje: ", mandelbrot.chuj())

        //performance.mark('Generating mandelbrot');
        mandelbrot.generate(scale, centerX, centerY);
        // console.log("CX ", x)
        // performance.mark('Mandelbrot generated');

        const pixelsPtr = mandelbrot.pixels();

        // performance.mark('Allocating memory');
        pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);
        // performance.mark('Memory allocated');

        // performance.mark('Creating image data');
        const imageData = ctx.createImageData(width, height);
        for (let i = 0; i < pixels.length; i++) {
            const color = pixels[i];
            imageData.data[i * 4] = (color % 16) * 16; // Red
            imageData.data[i * 4 + 1] = (color / 16) * 16; // Green
            imageData.data[i * 4 + 2] = color; // Blue
            imageData.data[i * 4 + 3] = 255; // Blue
        }
        //performance.mark('Image data created');

        //performance.mark('Drawing');
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.putImageData(imageData, 0, 0);
        //performance.mark('Drawed');

        scale *= 1.02;

        pixels = null;

        // const mandelbrotGenerationTime = performance.measure('Generating mandelbrot', 'Generating mandelbrot', 'Mandelbrot generated');
        // const allocatingMemoryTime = performance.measure('Allocating memory', 'Allocating memory', 'Memory allocated');
        // const creatingImageDataTime = performance.measure('Creating image data', 'Creating image data', 'Image data created');
        // const drawingTime = performance.measure('Drawing', 'Drawing', 'Drawed');

        // console.log("Generating mandelbrot", mandelbrotGenerationTime.duration);
        // console.log("Allocating memory", allocatingMemoryTime.duration);
        // console.log("Creating image data", creatingImageDataTime.duration);
        // console.log("Drawing", drawingTime.duration);
        // console.log("\n")

        // stats.end();
        requestAnimationFrame(animate);
    };

    animate();
};

render();
