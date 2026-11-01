(required, legible) => {
  const problems = [];
  const report = (kind, detail) => problems.push({ kind, detail });
  const width = document.documentElement.clientWidth;
  const slack = 1;

  const describe = (element) => {
    const classes = [...element.classList].map((name) => `.${name}`).join("");
    const text = (element.textContent || "").trim().replace(/\s+/g, " ").slice(0, 48);
    return `${element.tagName.toLowerCase()}${classes}${text ? ` "${text}"` : ""}`;
  };
  const styled = new Map();
  const styleShown = (node) => {
    if (!node || node.nodeType !== 1) return true;
    if (styled.has(node)) return styled.get(node);
    const style = getComputedStyle(node);
    const result = !(style.display === "none" || style.visibility === "hidden" || Number(style.opacity) === 0) && styleShown(node.parentElement);
    styled.set(node, result);
    return result;
  };
  const shownAlready = new Map();
  const shown = (element) => {
    if (shownAlready.has(element)) return shownAlready.get(element);
    const folded = element.closest("details:not([open])");
    const box = element.getBoundingClientRect();
    const result = !(folded && !element.closest("summary")) && styleShown(element) && box.width > 0 && box.height > 0;
    shownAlready.set(element, result);
    return result;
  };
  const clipper = (element) => {
    for (let node = element.parentElement; node && node !== document.body; node = node.parentElement) {
      const overflow = getComputedStyle(node).overflowX;
      if (overflow !== "visible") return node;
    }
    return null;
  };
  const clips = new Map();
  const clipBox = (element) => {
    if (!element || element === document.body) return { left: -Infinity, top: -Infinity, right: Infinity, bottom: Infinity };
    if (clips.has(element)) return clips.get(element);
    const outer = clipBox(element.parentElement);
    const style = getComputedStyle(element);
    let box = outer;
    if (style.overflowX !== "visible" || style.overflowY !== "visible") {
      const edge = element.getBoundingClientRect();
      box = { left: Math.max(outer.left, edge.left), top: Math.max(outer.top, edge.top), right: Math.min(outer.right, edge.right), bottom: Math.min(outer.bottom, edge.bottom) };
    }
    clips.set(element, box);
    return box;
  };
  const visible = (element, rect) => {
    const clip = clipBox(element.parentElement);
    const left = Math.max(rect.left, clip.left), top = Math.max(rect.top, clip.top);
    const right = Math.min(rect.right, clip.right), bottom = Math.min(rect.bottom, clip.bottom);
    return { left, top, right, bottom, width: Math.max(0, right - left), height: Math.max(0, bottom - top) };
  };
  const pinned = (element) => {
    const overlay = element.closest("[data-overlay]");
    if (overlay) return overlay;
    for (let node = element; node && node.nodeType === 1; node = node.parentElement) {
      const position = getComputedStyle(node).position;
      if (position === "fixed" || position === "sticky") return node;
    }
    return null;
  };
  const elements = [...document.body.querySelectorAll("*")].filter((element) => element.tagName === "svg" || !element.closest("svg"));
  const textBoxes = () => {
    const boxes = [];
    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    for (let node = walker.nextNode(); node; node = walker.nextNode()) {
      const element = node.parentElement;
      if (!node.textContent.trim() || !element || !shown(element)) continue;
      const content = node.textContent;
      const first = content.search(/\S/), last = content.length - content.split("").reverse().join("").search(/\S/);
      const pieces = element.tagName === "RUBY"
        ? [...content].map((character, index) => [index, index + character.length]).filter(([start]) => /\S/.test(content[start]))
        : [[first, last]];
      for (const [start, end] of pieces) {
        const range = document.createRange();
        range.setStart(node, start);
        range.setEnd(node, end);
        for (const raw of range.getClientRects()) {
          const rect = visible(element, raw);
          if (rect.width > slack && rect.height > slack) boxes.push({ element, rect, raw, pinned: pinned(element) });
          else if (element.closest("[data-overlay]") && raw.width > slack && raw.height > slack) boxes.push({ element, rect: raw, raw, pinned: pinned(element) });
        }
      }
    }
    return boxes;
  };

  if (document.documentElement.scrollWidth > width + slack) {
    report("page-scroll", `the page is ${document.documentElement.scrollWidth}px wide on a ${width}px screen`);
  }

  const wide = new Set();
  for (const element of elements) {
    if (!shown(element) || clipper(element)) continue;
    const box = element.getBoundingClientRect();
    if (box.right > width + slack || box.left < -slack) wide.add(element);
  }
  for (const element of wide) {
    if (wide.has(element.parentElement)) continue;
    const box = element.getBoundingClientRect();
    report("overflow", `${describe(element)} spans ${Math.round(box.left)}–${Math.round(box.right)}px`);
  }

  const boxes = textBoxes();
  const runaway = new Set();
  for (const box of boxes) {
    if (clipper(box.element) || runaway.has(box.element)) continue;
    if (box.rect.right > width + slack || box.rect.left < -slack) {
      runaway.add(box.element);
      report("overflow", `text in ${describe(box.element)} runs to ${Math.round(box.rect.right)}px`);
    }
  }

  for (const element of elements) {
    const style = getComputedStyle(element);
    if (!shown(element) || style.overflowX === "auto" || style.overflowX === "scroll") continue;
    if ((style.overflowX === "hidden" || style.overflowX === "clip") && element.scrollWidth > element.clientWidth + slack) {
      report("clipped", `${describe(element)} hides ${element.scrollWidth - element.clientWidth}px of its content`);
    }
  }

  for (const element of elements) {
    const style = getComputedStyle(element);
    if (!shown(element) || (style.overflowX !== "auto" && style.overflowX !== "scroll")) continue;
    if (element.scrollWidth > element.clientWidth + slack) {
      report("sideways", `${describe(element)} scrolls sideways: ${element.scrollWidth - element.clientWidth}px wider than it is`);
    }
  }

  const seen = new Set();
  const overlaps = (layer) => {
    for (let i = 0; i < layer.length; i += 1) {
      for (let j = i + 1; j < layer.length; j += 1) {
        const a = layer[i], b = layer[j];
        if (a.element === b.element || a.element.contains(b.element) || b.element.contains(a.element)) continue;
        const across = Math.min(a.rect.right, b.rect.right) - Math.max(a.rect.left, b.rect.left);
        const down = Math.min(a.rect.bottom, b.rect.bottom) - Math.max(a.rect.top, b.rect.top);
        if (across > 2 && down > 2) {
          const key = `${describe(a.element)}|${describe(b.element)}`;
          if (seen.has(key)) continue;
          seen.add(key);
          report("overlap", `${describe(a.element)} overlaps ${describe(b.element)} by ${Math.round(across)}×${Math.round(down)}px`);
        }
      }
    }
  };
  overlaps(boxes.filter((box) => !box.pinned));
  const layers = new Map();
  for (const box of boxes) {
    const overlay = box.element.closest("[data-overlay]");
    if (overlay) layers.set(overlay, [...(layers.get(overlay) || []), box]);
  }
  for (const layer of layers.values()) overlaps(layer);

  if (document.querySelector(".screen.fit")) {
    const page = document.scrollingElement;
    if (page.scrollHeight > innerHeight + slack) report("screen-scroll", `the screen scrolls: ${page.scrollHeight}px of content on a ${innerHeight}px screen`);
    const wrap = document.querySelector(".map-wrap"), map = document.querySelector(".day-map");
    if (wrap && map) {
      const short = map.getBoundingClientRect().bottom - wrap.getBoundingClientRect().bottom;
      if (short > slack) report("map-squeezed", `the map spills ${Math.round(short)}px over the status bar: the screen is too full`);
      if (map.getBoundingClientRect().height < innerHeight / 3) report("map-squeezed", `the map is only ${Math.round(map.getBoundingClientRect().height)}px tall: the panels above it take the screen`);
    }
  }

  window.scrollTo(0, document.documentElement.scrollHeight);
  const bars = elements.filter((element) => shown(element) && getComputedStyle(element).position === "fixed");
  const barTop = Math.min(innerHeight, ...bars.map((bar) => bar.getBoundingClientRect().top).filter((top) => top > innerHeight / 2));
  const lowest = textBoxes().filter((box) => !box.pinned).reduce((bottom, box) => Math.max(bottom, box.rect.bottom), 0);
  if (lowest > barTop + slack) {
    report("under-bar", `the last ${Math.round(lowest - barTop)}px of content stay hidden under the fixed bar even when scrolled to the end`);
  }
  window.scrollTo(0, 0);

  for (const selector of required) {
    const found = [...document.querySelectorAll(selector)];
    if (!found.length) report("missing", `nothing matches ${selector}`);
    else if (!found.some(shown)) report("invisible", `${selector} is in the page but not visible`);
  }

  const clockLines = new Map();
  for (const box of boxes) {
    if (box.element.closest("svg") || !box.element.closest(".time-fact, .time-plan")) continue;
    clockLines.set(box.element, new Set([...(clockLines.get(box.element) || []), Math.round(box.raw.top)]));
  }
  for (const [element, tops] of clockLines) {
    if (tops.size > 1) report("wrapped-time", `${describe(element)} wraps onto ${tops.size} lines`);
  }

  for (const reading of document.querySelectorAll("rt")) {
    if (!shown(reading)) continue;
    const size = parseFloat(getComputedStyle(reading).fontSize);
    if (size < legible - 0.05) report("small-reading", `${describe(reading.parentElement)} has furigana at ${size.toFixed(1)}px, under ${legible}px`);
  }

  for (const drawing of document.querySelectorAll("svg")) {
    if (!shown(drawing) && !drawing.closest("details:not([open])")) report("empty-drawing", `${describe(drawing)} has no size`);
  }
  return problems;
}
