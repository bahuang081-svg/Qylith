const STORAGE_KEY = "qylith-game-hall-v1";

const GAMES = {
  tap: {
    id: "tap",
    name: "快点泡泡",
    mode: "反应",
    hint: "看见亮起的泡泡就点它，连击会加分。",
    metric: "连击",
    time: 20
  },
  memory: {
    id: "memory",
    name: "记忆翻牌",
    mode: "记忆",
    hint: "翻开两张相同图案，步数越少分数越高。",
    metric: "步数",
    time: 0
  },
  color: {
    id: "color",
    name: "色彩急转",
    mode: "判断",
    hint: "根据目标色块选择相同颜色，连续正确会加分。",
    metric: "轮次",
    time: 30
  }
};

const COLORS = [
  { id: "coral", name: "珊瑚红", value: "#e65f4f" },
  { id: "teal", name: "松石绿", value: "#118c7a" },
  { id: "sun", name: "麦穗黄", value: "#f0bd3d" },
  { id: "leaf", name: "叶片绿", value: "#5b9c58" },
  { id: "berry", name: "浆果紫", value: "#7b5ec8" },
  { id: "sky", name: "湖面蓝", value: "#4f8fcf" }
];

const MEMORY_FACES = ["日", "月", "山", "川", "木", "火", "风", "石"];

const $ = selector => document.querySelector(selector);
const $$ = selector => Array.from(document.querySelectorAll(selector));

const els = {
  todayBest: $("#todayBest"),
  todayGame: $("#todayGame"),
  gameTitle: $("#gameTitle"),
  gameHint: $("#gameHint"),
  modeLabel: $("#modeLabel"),
  startBtn: $("#startBtn"),
  scoreValue: $("#scoreValue"),
  metricLabel: $("#metricLabel"),
  metricValue: $("#metricValue"),
  timeValue: $("#timeValue"),
  bestValue: $("#bestValue"),
  statusLine: $("#statusLine"),
  tapBoard: $("#tapBoard"),
  memoryBoard: $("#memoryBoard"),
  targetSwatch: $("#targetSwatch"),
  targetName: $("#targetName"),
  roundValue: $("#roundValue"),
  colorChoices: $("#colorChoices"),
  bestList: $("#bestList"),
  recentList: $("#recentList"),
  clearBtn: $("#clearBtn")
};

const state = {
  active: "tap",
  running: false,
  score: 0,
  metric: 0,
  time: GAMES.tap.time,
  timer: null,
  tapTarget: -1,
  memoryCards: [],
  firstCard: null,
  memoryLocked: false,
  memorySeconds: 0,
  colorRound: 1,
  colorGoal: COLORS[0],
  colorStreak: 0
};

function readStore() {
  try {
    return JSON.parse(localStorage.getItem(STORAGE_KEY)) || { best: {}, plays: [] };
  } catch (error) {
    return { best: {}, plays: [] };
  }
}

function writeStore(store) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(store));
}

function shuffle(list) {
  const next = list.slice();
  for (let i = next.length - 1; i > 0; i -= 1) {
    const j = Math.floor(Math.random() * (i + 1));
    [next[i], next[j]] = [next[j], next[i]];
  }
  return next;
}

function setStatus(text) {
  els.statusLine.textContent = text;
}

function stopTimer() {
  if (state.timer) {
    clearInterval(state.timer);
    state.timer = null;
  }
}

function getBest(gameId = state.active) {
  const store = readStore();
  return store.best[gameId] || 0;
}

function savePlay(meta) {
  const game = GAMES[state.active];
  const store = readStore();
  const best = Math.max(store.best[game.id] || 0, state.score);
  store.best[game.id] = best;
  store.plays = [
    {
      id: `${Date.now()}-${Math.random().toString(16).slice(2)}`,
      gameId: game.id,
      gameName: game.name,
      score: state.score,
      meta,
      createdAt: Date.now()
    },
    ...(store.plays || [])
  ].slice(0, 20);
  writeStore(store);
  renderScores();
  return best;
}

function formatTime(timestamp) {
  const date = new Date(timestamp);
  const pad = value => String(value).padStart(2, "0");
  return `${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

function renderStats() {
  els.scoreValue.textContent = state.score;
  els.metricValue.textContent = state.metric;
  els.timeValue.textContent = state.time || "-";
  els.bestValue.textContent = getBest();
}

function renderScores() {
  const store = readStore();
  const best = store.best || {};
  const rows = Object.values(GAMES)
    .map(game => `<div class="best-row"><span>${game.name}</span><strong>${best[game.id] || 0}</strong></div>`)
    .join("");
  els.bestList.innerHTML = rows;

  const plays = store.plays || [];
  if (plays.length === 0) {
    els.recentList.innerHTML = "<li><span>还没有记录</span>先来一局</li>";
  } else {
    els.recentList.innerHTML = plays
      .slice(0, 8)
      .map(play => `<li>${play.gameName} ${play.score}<span>${play.meta} · ${formatTime(play.createdAt)}</span></li>`)
      .join("");
  }

  const today = plays.filter(play => {
    const playDate = new Date(play.createdAt);
    const now = new Date();
    return playDate.toDateString() === now.toDateString();
  }).sort((a, b) => b.score - a.score)[0];

  els.todayBest.textContent = today ? today.score : 0;
  els.todayGame.textContent = today ? today.gameName : "还没有战绩";
}

function finishGame(message, meta) {
  state.running = false;
  stopTimer();
  const best = savePlay(meta);
  setStatus(`${message} 本局 ${state.score} 分，最高 ${best} 分。`);
  els.startBtn.textContent = "再来";
  renderStats();
}

function setActiveGame(gameId) {
  stopTimer();
  state.active = gameId;
  state.running = false;
  state.score = 0;
  state.metric = 0;
  state.time = GAMES[gameId].time;
  state.tapTarget = -1;
  state.firstCard = null;
  state.memoryLocked = false;
  state.colorRound = 1;
  state.colorStreak = 0;

  const game = GAMES[gameId];
  els.gameTitle.textContent = game.name;
  els.gameHint.textContent = game.hint;
  els.modeLabel.textContent = game.mode;
  els.metricLabel.textContent = game.metric;
  els.startBtn.textContent = "开始";
  setStatus("准备好了就开始。");

  $$(".game-tab").forEach(tab => {
    tab.classList.toggle("is-active", tab.dataset.game === gameId);
  });
  $("#tapPane").hidden = gameId !== "tap";
  $("#memoryPane").hidden = gameId !== "memory";
  $("#colorPane").hidden = gameId !== "color";

  if (gameId === "tap") renderTapBoard();
  if (gameId === "memory") renderMemoryBoard();
  if (gameId === "color") renderColorRound();
  renderStats();
}

function randomTapTarget() {
  let next = Math.floor(Math.random() * 16);
  while (next === state.tapTarget) {
    next = Math.floor(Math.random() * 16);
  }
  state.tapTarget = next;
}

function renderTapBoard() {
  if (state.tapTarget < 0) {
    state.tapTarget = Math.floor(Math.random() * 16);
  }
  els.tapBoard.innerHTML = Array.from({ length: 16 }, (_, index) => (
    `<button class="tap-cell ${index === state.tapTarget ? "is-target" : ""}" type="button" data-index="${index}" aria-label="泡泡 ${index + 1}"></button>`
  )).join("");
}

function startTap() {
  state.running = true;
  state.score = 0;
  state.metric = 0;
  state.time = GAMES.tap.time;
  randomTapTarget();
  renderTapBoard();
  renderStats();
  setStatus("泡泡亮起了。");
  els.startBtn.textContent = "进行中";
  stopTimer();
  state.timer = setInterval(() => {
    state.time -= 1;
    if (state.time <= 0) {
      finishGame("时间到。", `最高连击 ${state.metric}`);
      return;
    }
    renderStats();
  }, 1000);
}

function hitTap(index) {
  if (!state.running || state.active !== "tap") return;
  if (index === state.tapTarget) {
    state.metric += 1;
    state.score += 1 + Math.min(state.metric, 8);
    randomTapTarget();
    setStatus(`命中，连击 ${state.metric}。`);
  } else {
    state.metric = 0;
    state.score = Math.max(0, state.score - 1);
    setStatus("偏了一点，连击重置。");
  }
  renderTapBoard();
  renderStats();
}

function renderMemoryBoard() {
  if (state.memoryCards.length === 0) {
    state.memoryCards = shuffle([...MEMORY_FACES, ...MEMORY_FACES]).map((face, index) => ({
      id: index,
      face,
      open: false,
      matched: false
    }));
  }
  els.memoryBoard.innerHTML = state.memoryCards.map(card => (
    `<button class="memory-card ${card.open ? "is-open" : ""} ${card.matched ? "is-matched" : ""}" type="button" data-id="${card.id}" aria-label="记忆卡">${card.open || card.matched ? card.face : "?"}</button>`
  )).join("");
}

function startMemory() {
  state.running = true;
  state.score = 0;
  state.metric = 0;
  state.time = 0;
  state.memorySeconds = 0;
  state.firstCard = null;
  state.memoryLocked = false;
  state.memoryCards = shuffle([...MEMORY_FACES, ...MEMORY_FACES]).map((face, index) => ({
    id: index,
    face,
    open: false,
    matched: false
  }));
  renderMemoryBoard();
  renderStats();
  setStatus("翻两张相同图案。");
  els.startBtn.textContent = "进行中";
  stopTimer();
  state.timer = setInterval(() => {
    state.memorySeconds += 1;
    state.time = state.memorySeconds;
    renderStats();
  }, 1000);
}

function flipMemory(id) {
  if (!state.running || state.active !== "memory" || state.memoryLocked) return;
  const card = state.memoryCards.find(item => item.id === id);
  if (!card || card.open || card.matched) return;

  card.open = true;
  if (!state.firstCard) {
    state.firstCard = card;
    renderMemoryBoard();
    return;
  }

  state.metric += 1;
  const first = state.firstCard;
  state.firstCard = null;
  state.memoryLocked = true;

  if (first.face === card.face) {
    first.matched = true;
    card.matched = true;
    state.score += 18;
    state.memoryLocked = false;
    setStatus("配对成功。");
    if (state.memoryCards.every(item => item.matched)) {
      state.score = Math.max(20, state.score + 120 - state.metric * 4 - state.memorySeconds * 2);
      finishGame("全部配对完成。", `${state.metric} 步, ${state.memorySeconds} 秒`);
    }
  } else {
    state.score = Math.max(0, state.score - 2);
    setStatus("再记一下位置。");
    setTimeout(() => {
      first.open = false;
      card.open = false;
      state.memoryLocked = false;
      renderMemoryBoard();
      renderStats();
    }, 620);
  }
  renderMemoryBoard();
  renderStats();
}

function nextColorGoal() {
  state.colorGoal = COLORS[Math.floor(Math.random() * COLORS.length)];
}

function renderColorRound() {
  nextColorGoal();
  const choices = shuffle(COLORS).slice(0, 4);
  if (!choices.some(color => color.id === state.colorGoal.id)) {
    choices[Math.floor(Math.random() * choices.length)] = state.colorGoal;
  }
  const ordered = shuffle(choices);

  els.targetSwatch.style.background = state.colorGoal.value;
  els.targetName.textContent = state.colorGoal.name;
  els.roundValue.textContent = `第 ${state.colorRound} 轮`;
  els.colorChoices.innerHTML = ordered.map(color => (
    `<button class="color-choice" type="button" data-id="${color.id}">
      <span class="choice-swatch" style="background:${color.value}"></span>
      ${color.name}
    </button>`
  )).join("");
}

function startColor() {
  state.running = true;
  state.score = 0;
  state.metric = 1;
  state.time = GAMES.color.time;
  state.colorRound = 1;
  state.colorStreak = 0;
  renderColorRound();
  renderStats();
  setStatus("选出和目标一样的颜色。");
  els.startBtn.textContent = "进行中";
  stopTimer();
  state.timer = setInterval(() => {
    state.time -= 1;
    if (state.time <= 0) {
      finishGame("时间到。", `完成 ${state.colorRound - 1} 轮`);
      return;
    }
    renderStats();
  }, 1000);
}

function chooseColor(id) {
  if (!state.running || state.active !== "color") return;
  if (id === state.colorGoal.id) {
    state.colorStreak += 1;
    state.score += 10 + Math.min(state.colorStreak * 2, 12);
    setStatus(`正确，连续 ${state.colorStreak} 次。`);
  } else {
    state.colorStreak = 0;
    state.score = Math.max(0, state.score - 4);
    setStatus("颜色不对，下一轮继续。");
  }

  if (state.colorRound >= 12) {
    state.metric = 12;
    finishGame("12 轮完成。", `用时 ${GAMES.color.time - state.time} 秒`);
    return;
  }

  state.colorRound += 1;
  state.metric = state.colorRound;
  renderColorRound();
  renderStats();
}

function startActiveGame() {
  if (state.running) return;
  if (state.active === "tap") startTap();
  if (state.active === "memory") startMemory();
  if (state.active === "color") startColor();
}

$$(".game-tab").forEach(tab => {
  tab.addEventListener("click", () => setActiveGame(tab.dataset.game));
});

els.startBtn.addEventListener("click", startActiveGame);

els.tapBoard.addEventListener("click", event => {
  const button = event.target.closest(".tap-cell");
  if (!button) return;
  hitTap(Number(button.dataset.index));
});

els.memoryBoard.addEventListener("click", event => {
  const button = event.target.closest(".memory-card");
  if (!button) return;
  flipMemory(Number(button.dataset.id));
});

els.colorChoices.addEventListener("click", event => {
  const button = event.target.closest(".color-choice");
  if (!button) return;
  chooseColor(button.dataset.id);
});

els.clearBtn.addEventListener("click", () => {
  localStorage.removeItem(STORAGE_KEY);
  renderScores();
  renderStats();
  setStatus("本机记录已清空。");
});

window.addEventListener("keydown", event => {
  if (event.code === "Space") {
    event.preventDefault();
    startActiveGame();
  }
});

renderScores();
setActiveGame("tap");
