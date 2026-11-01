async (listener) => {
  const input = document.querySelector('.trip-file');
  const offers = [];
  const taken = [];
  window.dioxus = { send: (picked) => offers.push(picked) };
  window.addEventListener('click', (event) => { if (event.target === input) { taken.push(event.target); } });
  new Function(`(async () => { ${listener} })()`.replace('await new Promise(() => {});', ''))();
  const pick = (bytes, name) => {
    const chosen = new DataTransfer();
    chosen.items.add(new File([bytes], name));
    input.files = chosen.files;
    const before = offers.length;
    input.dispatchEvent(new Event('change', { bubbles: true }));
    return new Promise((done) => {
      const wait = (left) => (offers.length > before || left === 0 ? done() : setTimeout(() => wait(left - 1), 5));
      wait(200);
    });
  };
  input.click();
  await pick('[trip]', 'small.toml');
  await pick(new Uint8Array([0xff, 0xfe, 0x00]), 'photo.heic');
  return JSON.stringify({ taken: taken.length, offers, left: input.files.length });
}
