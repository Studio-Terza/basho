() => {
  const shown = window.bashoHeard.findLast((event) => event.shown)?.shown;
  const failed = window.bashoHeard.includes("failed");
  const home = location.origin;
  const away = performance.getEntriesByType("resource").map((entry) => entry.name).filter((name) => !name.startsWith(home) && !name.startsWith("blob:"));
  return JSON.stringify({ shown: shown ?? null, failed, away });
}
