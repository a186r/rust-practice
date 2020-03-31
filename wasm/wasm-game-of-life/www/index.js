import { Universe, Cell} from "wasm-game-of-life";
import {memory} from "wasm-game-of-life/wasm_game_of_life_bg"; // 导入WebAssembly memory

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

//构建宇宙，并获取其宽高
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

//给细胞画布一个1像素的边框
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

//为了绘制细胞之间的网格，我们绘制一组等间隔的水平线和一组等间隔的垂直线，形成网格
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

//    垂直线
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
//    水平线
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j*(CELL_SIZE + 1) * height + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    ctx.stroke();
};

const getIndex = (row, colum) => {
    return row * width + colum;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    //fillStyle属性为宇宙中的每个细胞在每个动画帧中都设置一次
    //设置fillStyle是昂贵的，可以为所有的活细胞设置一次，再为所有的死细胞设置一次，一共只需要设置两次fillStyle
    // for (let row = 0; row < height; row++) {
    //     for (let col = 0; col < width; col++) {
    //         const idx = getIndex(row, col);
    //         ctx.fillStyle = cells[idx] === Cell.Dead
    //             ?DEAD_COLOR
    //             :ALIVE_COLOR;
    //         ctx.fillRect(
    //             col * (CELL_SIZE + 1) + 1,
    //             row * (CELL_SIZE + 1) + 1,
    //             CELL_SIZE,
    //             CELL_SIZE
    //         );
    //     }
    // }

    //活细胞
    ctx.fillStyle = ALIVE_COLOR;
    for(let row = 0; row < height; row++){
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if(cells[idx] !== Cell.Alive) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    //死细胞
    ctx.fillStyle = DEAD_COLOR;
    for(let row = 0; row < height; row++) {
        for(let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Dead){
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

let animationId = null;

const renderLoop = () => {
    fps.render();

    universe.tick();

    drawGrid();
    drawCells();


    animationId = requestAnimationFrame(renderLoop);
};

// 在任何时刻我们都可以通过检查animationId的值来判断游戏是否暂停
const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    }else{
        pause();
    }
});

play();

//监听点击事件，将点击事件的页面相对坐标翻译为画布相对坐标，然后转换为行和列，调用toggle_cell方法，最后重绘场景。
canvas.addEventListener("click",  event => {
    const boudingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boudingRect.width;
    const scaleY = canvas.height / boudingRect.height;
    const canvasLeft = (event.clientX - boudingRect.width) * scaleX;
    const canvasTop = (event.clientY - boudingRect.height) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
    //    将上一次渲染帧的事件增量转换为fps的测量
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

    //    只保存最新的100个时间
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

    //    找出100个最近事件的最大值、最小值和平均值
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for(let i = 0; i < this.frames.length; i++){
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

    //    渲染统计信息
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
};