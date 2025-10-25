<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import GomokuBoard from "./GomokuBoard.svelte";

  const EMPTY = 0;
  const BLACK = 1;
  const WHITE = 2;

  // Board size options
  const SIZES = [
    { size: 9, label: "9×9 (快速)" },
    { size: 13, label: "13×13 (标准)" },
    { size: 15, label: "15×15 (五子棋)" },
    { size: 19, label: "19×19 (围棋)" },
  ];

  let boardSize = 15;
  let board: number[][] = [];
  let currentPlayer = BLACK;
  let playerColor = BLACK;
  let aiThinking = false;
  let gameOver = false;
  let winner: number | null = null;
  let history: Array<{ row: number; col: number; player: number }> = [];
  let status = "选择棋盘大小和棋色";
  let showSettings = true;

  // Computed cell size for responsive board
  $: cellSize = Math.min(500 / boardSize, 35);

  // Initialize board
  function initBoard(size: number) {
    boardSize = size;
    board = Array(size)
      .fill(0)
      .map(() => Array(size).fill(EMPTY));
    history = [];
    gameOver = false;
    winner = null;
    currentPlayer = BLACK;
  }

  // Start game
  function startGame(color: number) {
    playerColor = color;
    currentPlayer = BLACK;
    showSettings = false;
    status = color === BLACK ? "黑棋落子（你的回合）" : "AI 思考中...";

    if (color === WHITE) {
      setTimeout(aiMove, 500);
    }
  }

  // Player move
  async function handleClick(row: number, col: number) {
    if (
      gameOver ||
      aiThinking ||
      currentPlayer !== playerColor ||
      board[row][col] !== EMPTY
    ) {
      return;
    }

    makeMove(row, col, playerColor);
    history = [...history, { row, col, player: playerColor }];

    const result = await checkWin(row, col);
    if (result.is_win) {
      gameOver = true;
      winner = result.winner!;
      status = "🎉 你赢了！";
      return;
    }

    currentPlayer = 3 - currentPlayer;
    await aiMove();
  }

  // AI move
  async function aiMove() {
    if (gameOver) return;

    aiThinking = true;
    status = "🤖 AI 思考中...";

    try {
      const aiColor = 3 - playerColor;
      const result = await invoke<{ row: number; col: number }>("get_ai_move", {
        board,
        currentPlayer: aiColor,
        depth: 4,
      });

      makeMove(result.row, result.col, aiColor);
      history = [...history, { row: result.row, col: result.col, player: aiColor }];

      const winResult = await checkWin(result.row, result.col);
      if (winResult.is_win) {
        gameOver = true;
        winner = winResult.winner!;
        status = "AI 获胜";
        aiThinking = false;
        return;
      }

      currentPlayer = 3 - currentPlayer;
      status = "轮到你了";
    } catch (error) {
      console.error("AI move failed:", error);
      status = "AI 出错了";
    }

    aiThinking = false;
  }

  // Make move
  function makeMove(row: number, col: number, player: number) {
    board[row][col] = player;
    board = board;
  }

  // Check win
  async function checkWin(
    row: number,
    col: number
  ): Promise<{ is_win: boolean; winner: number | null }> {
    try {
      return await invoke("check_win", { board, row, col });
    } catch {
      return { is_win: false, winner: null };
    }
  }

  // Undo move
  function undoMove() {
    if (aiThinking || history.length < 2) return;

    const aiMove = history.pop()!;
    board[aiMove.row][aiMove.col] = EMPTY;

    const playerMove = history.pop()!;
    board[playerMove.row][playerMove.col] = EMPTY;

    board = board;
    history = history;
    currentPlayer = playerColor;
    gameOver = false;
    winner = null;
    status = "轮到你了";
  }

  // New game
  function newGame() {
    initBoard(boardSize);
    showSettings = true;
    status = "选择棋盘大小和棋色";
  }

  // Init
  initBoard(15);
</script>

<main>
  <div class="container">
    <!-- Status Bar -->
    <div class="status-bar glass-panel">
      <span class="status-text">{status}</span>
    </div>

    {#if showSettings}
      <!-- Settings Panel -->
      <div class="settings-panel glass-panel">
        <h2>棋盘大小</h2>
        <div class="slider-container">
          <div class="slider-header">
            <span class="slider-label">尺寸:</span>
            <span class="slider-value">{boardSize}×{boardSize}</span>
          </div>
          <input
            type="range"
            min="9"
            max="19"
            step="1"
            bind:value={boardSize}
            on:input={() => initBoard(boardSize)}
            class="board-slider"
          />
          <div class="slider-marks">
            <span>9×9</span>
            <span>13×13</span>
            <span>15×15</span>
            <span>19×19</span>
          </div>
        </div>

        <h2>选择棋色</h2>
        <div class="color-grid">
          <button class="color-btn black-btn" on:click={() => startGame(BLACK)}>
            <div class="stone-preview black"></div>
            <span>执黑（先手）</span>
          </button>
          <button class="color-btn white-btn" on:click={() => startGame(WHITE)}>
            <div class="stone-preview white"></div>
            <span>执白（后手）</span>
          </button>
        </div>
      </div>
    {:else}
      <!-- Game Board - Canvas -->
      <div class="board-container glass-panel">
        <GomokuBoard
          {boardSize}
          {board}
          lastMove={history.length > 0 ? history[history.length - 1] : null}
          onCellClick={handleClick}
        />
      </div>

      <!-- Controls -->
      <div class="controls glass-panel">
        <button
          class="control-btn"
          on:click={undoMove}
          disabled={aiThinking || history.length < 2}
        >
          <span class="icon">↶</span>
          悔棋
        </button>
        <button class="control-btn primary" on:click={newGame}>
          <span class="icon">🎲</span>
          新游戏
        </button>
      </div>
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "SF Pro Text",
      "Helvetica Neue", sans-serif;
    background: #f5f5f7;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    -webkit-font-smoothing: antialiased;
  }

  /* Dark Mode */
  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: #1d1d1f;
    }
  }

  main {
    width: 100%;
    padding: 20px;
    box-sizing: border-box;
  }

  .container {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: fit-content;
    margin: 0 auto;
  }

  /* Glass Panel Effect - True macOS Style */
  .glass-panel {
    background: rgba(255, 255, 255, 0.35);
    backdrop-filter: blur(40px) saturate(180%);
    -webkit-backdrop-filter: blur(40px) saturate(180%);
    border-radius: 16px;
    border: 1px solid rgba(0, 0, 0, 0.08);
    box-shadow:
      0 0 0 1px rgba(255, 255, 255, 0.4) inset,
      0 8px 32px rgba(0, 0, 0, 0.06),
      0 1px 3px rgba(0, 0, 0, 0.04);
    padding: 20px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @media (prefers-color-scheme: dark) {
    .glass-panel {
      background: rgba(28, 28, 30, 0.75);
      border: 1px solid rgba(255, 255, 255, 0.12);
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.08) inset,
        0 8px 32px rgba(0, 0, 0, 0.5),
        0 1px 3px rgba(0, 0, 0, 0.3);
    }
  }

  .glass-panel:hover {
    border-color: rgba(0, 0, 0, 0.12);
    box-shadow:
      0 0 0 1px rgba(255, 255, 255, 0.5) inset,
      0 12px 40px rgba(0, 0, 0, 0.08),
      0 2px 4px rgba(0, 0, 0, 0.06);
  }

  @media (prefers-color-scheme: dark) {
    .glass-panel:hover {
      border-color: rgba(255, 255, 255, 0.18);
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.12) inset,
        0 12px 40px rgba(0, 0, 0, 0.6),
        0 2px 4px rgba(0, 0, 0, 0.4);
    }
  }

  /* Status Bar */
  .status-bar {
    text-align: center;
    padding: 16px;
  }

  .status-text {
    font-size: 16px;
    font-weight: 500;
    color: #1d1d1f;
  }

  @media (prefers-color-scheme: dark) {
    .status-text {
      color: #f5f5f7;
    }
  }

  /* Settings Panel */
  .settings-panel {
    padding: 28px;
  }

  .settings-panel h2 {
    color: #1d1d1f;
    font-size: 15px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin: 24px 0 16px;
    opacity: 0.9;
  }

  @media (prefers-color-scheme: dark) {
    .settings-panel h2 {
      color: #f5f5f7;
    }
  }

  .settings-panel h2:first-of-type {
    margin-top: 0;
  }

  .color-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 14px;
  }

  .color-btn {
    background: rgba(255, 255, 255, 0.3);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1.5px solid rgba(0, 0, 0, 0.08);
    border-radius: 14px;
    padding: 24px;
    color: #1d1d1f;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
    font-size: 15px;
    font-weight: 500;
  }

  @media (prefers-color-scheme: dark) {
    .color-btn {
      background: rgba(255, 255, 255, 0.08);
      border: 1.5px solid rgba(255, 255, 255, 0.15);
      color: #f5f5f7;
    }
  }

  .color-btn:hover {
    background: rgba(255, 255, 255, 0.45);
    border-color: rgba(0, 0, 0, 0.15);
    transform: translateY(-3px) scale(1.02);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.12);
  }

  @media (prefers-color-scheme: dark) {
    .color-btn:hover {
      background: rgba(255, 255, 255, 0.14);
      border-color: rgba(255, 255, 255, 0.25);
      box-shadow: 0 8px 20px rgba(0, 0, 0, 0.5);
    }
  }

  .color-btn:active {
    transform: translateY(-1px) scale(1);
  }

  .stone-preview {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.35);
    transition: transform 0.2s ease;
  }

  .color-btn:hover .stone-preview {
    transform: scale(1.1);
  }

  .stone-preview.black {
    background: radial-gradient(circle at 30% 30%, #4a4a4a, #0a0a0a);
  }

  .stone-preview.white {
    background: radial-gradient(circle at 30% 30%, #ffffff, #d0d0d0);
    border: 1.5px solid rgba(0, 0, 0, 0.12);
  }

  /* Board Container */
  .board-container {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 24px;
    background: rgba(220, 179, 92, 0.35);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    box-sizing: border-box;
  }

  @media (prefers-color-scheme: dark) {
    .board-container {
      background: rgba(180, 139, 52, 0.25);
    }
  }

  /* Controls */
  .controls {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .control-btn {
    background: rgba(255, 255, 255, 0.25);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1.5px solid rgba(0, 0, 0, 0.08);
    border-radius: 12px;
    padding: 12px 24px;
    color: #1d1d1f;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  @media (prefers-color-scheme: dark) {
    .control-btn {
      background: rgba(255, 255, 255, 0.08);
      border: 1.5px solid rgba(255, 255, 255, 0.15);
      color: #f5f5f7;
    }
  }

  .control-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.4);
    border-color: rgba(0, 0, 0, 0.12);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
  }

  @media (prefers-color-scheme: dark) {
    .control-btn:hover:not(:disabled) {
      background: rgba(255, 255, 255, 0.12);
      border-color: rgba(255, 255, 255, 0.25);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    }
  }

  .control-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .control-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .control-btn.primary {
    background: rgba(255, 255, 255, 0.35);
    border-color: rgba(0, 0, 0, 0.12);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  }

  @media (prefers-color-scheme: dark) {
    .control-btn.primary {
      background: rgba(255, 255, 255, 0.12);
      border-color: rgba(255, 255, 255, 0.2);
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
    }
  }

  .icon {
    font-size: 18px;
  }

  /* Slider Container */
  .slider-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px 20px;
    background: rgba(255, 255, 255, 0.25);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1.5px solid rgba(0, 0, 0, 0.08);
    border-radius: 12px;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @media (prefers-color-scheme: dark) {
    .slider-container {
      background: rgba(255, 255, 255, 0.08);
      border: 1.5px solid rgba(255, 255, 255, 0.15);
    }
  }

  .slider-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;
    font-weight: 500;
  }

  .slider-label {
    color: #1d1d1f;
    opacity: 0.8;
  }

  @media (prefers-color-scheme: dark) {
    .slider-label {
      color: #f5f5f7;
    }
  }

  .slider-value {
    color: #1d1d1f;
    font-weight: 600;
    font-size: 15px;
    padding: 4px 12px;
    background: rgba(0, 0, 0, 0.04);
    border-radius: 8px;
    min-width: 60px;
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    .slider-value {
      color: #f5f5f7;
      background: rgba(255, 255, 255, 0.08);
    }
  }

  /* Range Slider Styling */
  .board-slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    background: rgba(0, 0, 0, 0.08);
    border-radius: 10px;
    outline: none;
    transition: all 0.2s ease;
  }

  @media (prefers-color-scheme: dark) {
    .board-slider {
      background: rgba(255, 255, 255, 0.1);
    }
  }

  /* Webkit (Chrome, Safari, Edge) slider thumb */
  .board-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid rgba(0, 0, 0, 0.12);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .board-slider::-webkit-slider-thumb:hover {
    background: rgba(255, 255, 255, 1);
    transform: scale(1.15);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .board-slider::-webkit-slider-thumb:active {
    transform: scale(1.05);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.25);
  }

  @media (prefers-color-scheme: dark) {
    .board-slider::-webkit-slider-thumb {
      background: rgba(255, 255, 255, 0.85);
      border: 2px solid rgba(255, 255, 255, 0.3);
    }

    .board-slider::-webkit-slider-thumb:hover {
      background: rgba(255, 255, 255, 0.95);
      border-color: rgba(255, 255, 255, 0.4);
    }
  }

  /* Firefox slider thumb */
  .board-slider::-moz-range-thumb {
    width: 20px;
    height: 20px;
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid rgba(0, 0, 0, 0.12);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .board-slider::-moz-range-thumb:hover {
    background: rgba(255, 255, 255, 1);
    transform: scale(1.15);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .board-slider::-moz-range-thumb:active {
    transform: scale(1.05);
  }

  @media (prefers-color-scheme: dark) {
    .board-slider::-moz-range-thumb {
      background: rgba(255, 255, 255, 0.85);
      border: 2px solid rgba(255, 255, 255, 0.3);
    }

    .board-slider::-moz-range-thumb:hover {
      background: rgba(255, 255, 255, 0.95);
      border-color: rgba(255, 255, 255, 0.4);
    }
  }

  /* Firefox slider track */
  .board-slider::-moz-range-track {
    background: rgba(0, 0, 0, 0.08);
    border-radius: 10px;
    height: 6px;
  }

  @media (prefers-color-scheme: dark) {
    .board-slider::-moz-range-track {
      background: rgba(255, 255, 255, 0.1);
    }
  }

  /* Slider Marks */
  .slider-marks {
    display: flex;
    justify-content: space-between;
    padding: 0 2px;
    font-size: 11px;
    color: #1d1d1f;
    opacity: 0.5;
    font-weight: 500;
  }

  @media (prefers-color-scheme: dark) {
    .slider-marks {
      color: #f5f5f7;
    }
  }

  .slider-marks span {
    transition: opacity 0.2s ease;
  }

  .slider-marks span:hover {
    opacity: 1;
  }
</style>
