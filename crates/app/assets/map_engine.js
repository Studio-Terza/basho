const bashoMapEngine = async (files, send) => {
  const INK = "#000000";
  const PAPER = "#ffffff";
  const GREY = "#6a6a6a";
  const PIXELS = 2;
  const FIT_PADDING = 34;
  const PINCH_RATIO = 1.3;
  const TAP_REACH = 18;
  const OWN_LAYERS = ["marks"];
  const BASE_LAYER = /^poi-/;

  const shapes = {
    square: "M5 5h14v14H5z",
    diamond: "M12 2l10 10-10 10L2 12z",
    cross: "M12 3v18M3 12h18",
    bar: "M12 3v18",
    star: "M12 1l3 7.5 8 .6-6.2 5.2 2 7.8L12 18l-6.8 4.1 2-7.8L1 9.1l8-.6z",
    triangle: "M12 3l10 18H2z",
    dot: "M12 5a7 7 0 1 0 0 14a7 7 0 1 0 0-14z",
    ring: "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18z",
    house: "M3 21v-10l9-8 9 8v10z",
    flag: "M5 22V3h15l-5 5 5 5H7v9z",
    tick: "M4 12h16",
    ex: "M4 4l16 16M20 4L4 20",
    cone: "M12 24L0 2a22 22 0 0 1 24 0z",
  };
  const looks = {
    food: ["square", "solid"],
    shop: ["diamond", "hollow"],
    sight: ["diamond", "solid"],
    service: ["cross", "stroke"],
    landmark: ["bar", "stroke"],
    catch: ["square", "hollow"],
    star: ["star", "solid"],
    peak: ["triangle", "solid"],
    day_start: ["flag", "solid"],
    leg_end: ["tick", "stroke"],
    goal: ["ring", "target"],
    loop_stop: ["dot", "solid"],
    night: ["house", "solid"],
    you: ["ring", "you"],
    you_estimate: ["ring", "you_hollow"],
    you_last_fix: ["ex", "stroke"],
    no_path: ["triangle", "hollow"],
  };
  const baseLooks = {
    konbini: ["square", "faint"],
    food: ["dot", "faint"],
    shop: ["diamond", "faint"],
    service: ["cross", "faint_stroke"],
    toilet: ["cross", "stroke"],
    lodging: ["house", "faint"],
    worship: ["bar", "stroke"],
    sight: ["diamond", "solid"],
    station: ["square", "hollow"],
    peak: ["triangle", "solid"],
  };

  const library = await import(files.library);
  const workerCode = await fetch(files.worker).then((reply) => reply.blob());
  library.setWorkerUrl(URL.createObjectURL(new Blob([workerCode], { type: "text/javascript" })));
  const sheet = document.createElement("link");
  sheet.rel = "stylesheet";
  sheet.href = files.sheet;
  document.head.append(sheet);

  const painted = (shape, tone, size) => {
    const canvas = document.createElement("canvas");
    canvas.width = size * PIXELS;
    canvas.height = size * PIXELS;
    const pen = canvas.getContext("2d");
    pen.scale((size * PIXELS) / 24, (size * PIXELS) / 24);
    const path = new Path2D(shapes[shape]);
    pen.lineJoin = "round";
    pen.lineCap = "round";
    const outlined = (fill, line, width) => {
      pen.strokeStyle = PAPER;
      pen.lineWidth = width + 3;
      pen.stroke(path);
      pen.fillStyle = fill;
      pen.fill(path);
      pen.strokeStyle = line;
      pen.lineWidth = width;
      pen.stroke(path);
    };
    const inner = (fill, radius) => {
      pen.fillStyle = fill;
      pen.beginPath();
      pen.arc(12, 12, radius, 0, Math.PI * 2);
      pen.fill();
    };
    const drawn = {
      solid: () => outlined(INK, INK, 1.5),
      hollow: () => outlined(PAPER, INK, 2),
      faint: () => outlined(GREY, GREY, 1),
      target: () => {
        outlined(INK, INK, 1.5);
        inner(PAPER, 4.5);
      },
      you: () => {
        outlined(PAPER, INK, 2.5);
        inner(INK, 5);
      },
      you_hollow: () => {
        outlined(PAPER, INK, 2.5);
        pen.strokeStyle = INK;
        pen.lineWidth = 2;
        pen.beginPath();
        pen.arc(12, 12, 4.5, 0, Math.PI * 2);
        pen.stroke();
      },
      stroke: () => {
        pen.strokeStyle = PAPER;
        pen.lineWidth = 6;
        pen.stroke(path);
        pen.strokeStyle = INK;
        pen.lineWidth = 2.6;
        pen.stroke(path);
      },
      faint_stroke: () => {
        pen.strokeStyle = GREY;
        pen.lineWidth = 2.4;
        pen.stroke(path);
      },
      cone: () => {
        pen.fillStyle = "rgba(0, 0, 0, 0.28)";
        pen.fill(path);
      },
    };
    drawn[tone]();
    return { image: pen.getImageData(0, 0, size * PIXELS, size * PIXELS), pixelRatio: PIXELS };
  };

  const host = document.createElement("div");
  host.className = "map-canvas";
  const seated = () => {
    const slot = document.querySelector(".map-slot");
    if (slot && host.parentNode !== slot) {
      slot.append(host);
      map?.resize();
    }
  };
  let map = null;
  const style = await fetch(files.style).then((reply) => reply.json());
  library.addProtocol("basho", async (request, stopped) => {
    const reply = await fetch(`${files.tiles}/${request.url.split("/").slice(3).join("/")}`, { signal: stopped.signal });
    return { data: await reply.arrayBuffer() };
  });
  map = new library.Map({
    container: host,
    style,
    center: [140.5, 38.7],
    zoom: 8,
    minZoom: 5,
    maxZoom: 18,
    attributionControl: false,
    dragRotate: false,
    pitchWithRotate: false,
    touchPitch: false,
    keyboard: false,
    boxZoom: false,
    doubleClickZoom: false,
    fadeDuration: 0,
    canvasContextAttributes: { preserveDrawingBuffer: false, antialias: true },
  });
  map.touchZoomRotate.disableRotation();
  map.addControl(new library.ScaleControl({ unit: "metric", maxWidth: 90 }), "bottom-left");
  map.addControl(new library.AttributionControl({ compact: false, customAttribution: "© OpenStreetMap contributors, ODbL" }), "bottom-right");
  Object.entries(looks).forEach(([name, [shape, tone]]) => map.addImage(`mark-${name}`, painted(shape, tone, 22).image, { pixelRatio: PIXELS }));
  Object.entries(baseLooks).forEach(([name, [shape, tone]]) => map.addImage(`base-${name}`, painted(shape, tone, 11).image, { pixelRatio: PIXELS }));
  map.addImage("walker-cone", painted("cone", "cone", 64).image, { pixelRatio: PIXELS });

  let framed = true;
  let lastBounds = null;
  let lastKey = "";
  let keyStale = true;
  let hearer = send;
  const loaded = new Promise((resolve) => map.once("load", resolve));
  const gestures = [map.dragPan, map.touchZoomRotate, map.scrollZoom];
  const guided = (guiding) => {
    framed = guiding;
    gestures.forEach((gesture) => (guiding ? gesture.disable() : gesture.enable()));
    map.touchZoomRotate.disableRotation();
  };
  const frame = (bounds, moved) => {
    lastBounds = bounds ?? lastBounds;
    if (framed && lastBounds) {
      map.fitBounds(lastBounds, { padding: FIT_PADDING, maxZoom: 17, duration: moved ? 300 : 0 });
    }
  };

  const shownKey = () => {
    const found = new Set();
    map.queryRenderedFeatures().forEach((feature) => {
      const key = feature.layer.metadata?.key ?? feature.properties?.mark;
      if (key) {
        found.add(key);
      }
    });
    const key = [...found].sort().join(" ");
    if (key !== lastKey) {
      lastKey = key;
      hearer({ shown: [...found].sort() });
    }
  };
  map.on("moveend", () => {
    keyStale = true;
  });
  map.on("sourcedata", (event) => {
    keyStale = keyStale || event.sourceId === "base";
  });
  map.on("idle", () => {
    if (keyStale) {
      keyStale = false;
      shownKey();
    }
  });

  map.on("click", (event) => {
    const box = [
      [event.point.x - TAP_REACH, event.point.y - TAP_REACH],
      [event.point.x + TAP_REACH, event.point.y + TAP_REACH],
    ];
    const near = map.queryRenderedFeatures(box);
    const own = near.find((feature) => OWN_LAYERS.includes(feature.layer.id));
    const base = near.find((feature) => BASE_LAYER.test(feature.layer.id));
    const tapped = own ?? base;
    hearer({
      picked: tapped
        ? { kind: tapped.layer.metadata?.key ?? tapped.properties.mark, name: tapped.properties.label ?? null }
        : null,
    });
  });

  const spread = (touches) => Math.hypot(touches[0].clientX - touches[1].clientX, touches[0].clientY - touches[1].clientY);
  let pinchStart = null;
  let pinchEnd = null;
  host.addEventListener("touchstart", (event) => {
    if (event.touches.length === 2) {
      pinchStart = spread(event.touches);
      pinchEnd = pinchStart;
    }
  });
  host.addEventListener("touchmove", (event) => {
    if (event.touches.length === 2) {
      pinchEnd = spread(event.touches);
    }
  });
  host.addEventListener("touchend", () => {
    if (framed && pinchStart && pinchEnd / pinchStart > PINCH_RATIO) {
      hearer({ pinch: "apart" });
    } else if (framed && pinchStart && pinchStart / pinchEnd > PINCH_RATIO) {
      hearer({ pinch: "together" });
    }
    pinchStart = null;
  });

  new MutationObserver(seated).observe(document.body, { childList: true, subtree: true });
  seated();
  guided(true);
  await loaded;
  return {
    heardBy: (listener) => {
      hearer = listener;
      lastKey = "";
      keyStale = true;
    },
    receive: ({ route, marks, walker, bounds, mode }) => {
      if (mode) {
        guided(mode === "guide");
        frame(null, true);
      }
      if (route) {
        map.getSource("route").setData(route);
      }
      if (marks) {
        map.getSource("marks").setData(marks);
        keyStale = true;
      }
      if (walker) {
        map.getSource("walker").setData(walker);
      }
      if (bounds) {
        frame(bounds, Boolean(lastBounds));
      }
    },
  };
};

try {
  const files = await dioxus.recv();
  window.bashoEngine ??= bashoMapEngine(files, (event) => dioxus.send(event));
  const engine = await window.bashoEngine;
  engine.heardBy((event) => dioxus.send(event));
  while (true) {
    engine.receive(await dioxus.recv());
  }
} catch (trouble) {
  console.error(trouble);
  dioxus.send("failed");
}
