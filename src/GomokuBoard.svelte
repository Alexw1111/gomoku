<script lang="ts">
  import { onMount, afterUpdate } from 'svelte';

  // Props
  export let boardSize: number = 15;
  export let board: number[][] = [];
  export let lastMove: { row: number; col: number } | null = null;
  export let onCellClick: (row: number, col: number) => void = () => {};

  const EMPTY = 0;
  const BLACK = 1;
  const WHITE = 2;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let cellSize = 35;
  let padding = cellSize / 2;
  let canvasSize = 0;

  // 计算画布尺寸
  $: {
    cellSize = Math.min(500 / boardSize, 35);
    padding = cellSize / 2;
    canvasSize = (boardSize - 1) * cellSize + 2 * padding;
  }

  onMount(() => {
    ctx = canvas.getContext('2d');
    if (ctx) {
      // 设置高 DPI 支持
      const dpr = window.devicePixelRatio || 1;
      canvas.width = canvasSize * dpr;
      canvas.height = canvasSize * dpr;
      canvas.style.width = `${canvasSize}px`;
      canvas.style.height = `${canvasSize}px`;
      ctx.scale(dpr, dpr);

      drawBoard();
    }
  });

  afterUpdate(() => {
    if (ctx) {
      drawBoard();
    }
  });

  // 绘制整个棋盘
  function drawBoard() {
    if (!ctx) return;

    // 清空画布
    ctx.clearRect(0, 0, canvasSize, canvasSize);

    // 绘制网格线
    drawGrid();

    // 绘制星位点
    drawStarPoints();

    // 绘制所有棋子
    drawStones();

    // 绘制最后一手标记
    if (lastMove) {
      drawLastMoveMarker(lastMove.row, lastMove.col);
    }
  }

  // 绘制网格线
  function drawGrid() {
    if (!ctx) return;

    ctx.strokeStyle = 'rgba(0, 0, 0, 0.8)';
    ctx.lineWidth = 1;

    // 绘制横线
    for (let i = 0; i < boardSize; i++) {
      const y = padding + i * cellSize;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(padding + (boardSize - 1) * cellSize, y);
      ctx.stroke();
    }

    // 绘制竖线
    for (let i = 0; i < boardSize; i++) {
      const x = padding + i * cellSize;
      ctx.beginPath();
      ctx.moveTo(x, padding);
      ctx.lineTo(x, padding + (boardSize - 1) * cellSize);
      ctx.stroke();
    }
  }

  // 绘制星位点
  function drawStarPoints() {
    if (!ctx) return;

    ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';

    let starPoints: [number, number][] = [];

    if (boardSize === 19) {
      starPoints = [
        [3, 3], [3, 9], [3, 15],
        [9, 3], [9, 9], [9, 15],
        [15, 3], [15, 9], [15, 15]
      ];
    } else if (boardSize === 13) {
      starPoints = [
        [3, 3], [3, 9],
        [6, 6],
        [9, 3], [9, 9]
      ];
    }

    starPoints.forEach(([row, col]) => {
      const x = padding + col * cellSize;
      const y = padding + row * cellSize;
      ctx!.beginPath();
      ctx!.arc(x, y, 4, 0, Math.PI * 2);
      ctx!.fill();
    });
  }

  // 绘制所有棋子
  function drawStones() {
    if (!ctx) return;

    for (let row = 0; row < boardSize; row++) {
      for (let col = 0; col < boardSize; col++) {
        const cell = board[row][col];
        if (cell !== EMPTY) {
          drawStone(row, col, cell);
        }
      }
    }
  }

  // 绘制单个棋子
  function drawStone(row: number, col: number, color: number) {
    if (!ctx) return;

    const x = padding + col * cellSize;
    const y = padding + row * cellSize;
    const radius = cellSize * 0.42;

    // 绘制阴影
    ctx.shadowColor = 'rgba(0, 0, 0, 0.4)';
    ctx.shadowBlur = 4;
    ctx.shadowOffsetX = 2;
    ctx.shadowOffsetY = 2;

    // 绘制棋子
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, Math.PI * 2);

    if (color === BLACK) {
      // 黑子渐变
      const gradient = ctx.createRadialGradient(
        x - radius * 0.3,
        y - radius * 0.3,
        0,
        x,
        y,
        radius
      );
      gradient.addColorStop(0, '#4a4a4a');
      gradient.addColorStop(1, '#0a0a0a');
      ctx.fillStyle = gradient;
    } else {
      // 白子渐变
      const gradient = ctx.createRadialGradient(
        x - radius * 0.3,
        y - radius * 0.3,
        0,
        x,
        y,
        radius
      );
      gradient.addColorStop(0, '#ffffff');
      gradient.addColorStop(1, '#d0d0d0');
      ctx.fillStyle = gradient;

      // 白子边框
      ctx.fill();
      ctx.shadowColor = 'transparent';
      ctx.strokeStyle = 'rgba(0, 0, 0, 0.15)';
      ctx.lineWidth = 1;
      ctx.stroke();
      return;
    }

    ctx.fill();
    ctx.shadowColor = 'transparent';
  }

  // 绘制最后一手标记
  function drawLastMoveMarker(row: number, col: number) {
    if (!ctx) return;

    const x = padding + col * cellSize;
    const y = padding + row * cellSize;
    const radius = cellSize * 0.15;

    ctx.fillStyle = 'rgba(255, 100, 100, 0.7)';
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, Math.PI * 2);
    ctx.fill();
  }

  // 处理点击事件
  function handleClick(event: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // 计算点击的交点
    const col = Math.round((x - padding) / cellSize);
    const row = Math.round((y - padding) / cellSize);

    // 检查是否在棋盘范围内
    if (row >= 0 && row < boardSize && col >= 0 && col < boardSize) {
      onCellClick(row, col);
    }
  }
</script>

<canvas
  bind:this={canvas}
  on:click={handleClick}
  style="cursor: pointer; border-radius: 12px;"
/>

<style>
  canvas {
    display: block;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
</style>
