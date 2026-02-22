/**
 * Generates tray icons for idle, working, and break states.
 * Run: node scripts/generate-tray-icons.cjs
 */

const { PNG } = require("pngjs");
const fs = require("fs");
const path = require("path");

const SIZE = 22;
const OUT_DIR = path.join(__dirname, "../src-tauri/icons/tray");

fs.mkdirSync(OUT_DIR, { recursive: true });

function setPixel(data, x, y, r, g, b, a) {
  if (x < 0 || x >= SIZE || y < 0 || y >= SIZE) return;
  const i = (y * SIZE + x) * 4;
  data[i] = r;
  data[i + 1] = g;
  data[i + 2] = b;
  data[i + 3] = a;
}

function writePng(filename, data) {
  const png = { width: SIZE, height: SIZE, data };
  const buffer = PNG.sync.write(png);
  fs.writeFileSync(path.join(OUT_DIR, filename), buffer);
  console.log(`Created ${filename}`);
}

const cx = SIZE / 2 - 0.5;
const cy = SIZE / 2 - 0.5;

const workingData = Buffer.alloc(SIZE * SIZE * 4, 0);
for (let y = 0; y < SIZE; y++) {
  for (let x = 0; x < SIZE; x++) {
    const dist = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2);
    const alpha = dist <= 8 ? 255 : 0;
    setPixel(workingData, x, y, 0, 0, 0, alpha);
  }
}
writePng("tray_working.png", workingData);

const breakData = Buffer.alloc(SIZE * SIZE * 4, 0);
const barW = 3;
const gap = 4;
const left = Math.floor((SIZE - barW * 2 - gap) / 2);
for (let y = 6; y < SIZE - 6; y++) {
  for (let x = left; x < left + barW; x++) {
    setPixel(breakData, x, y, 0, 0, 0, 255);
  }
  for (let x = left + barW + gap; x < left + barW * 2 + gap; x++) {
    setPixel(breakData, x, y, 0, 0, 0, 255);
  }
}
writePng("tray_break.png", breakData);
