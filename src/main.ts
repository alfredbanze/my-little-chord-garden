import { keyboard } from "./chords";

const layouts = [
  {
    keys: ["q","w","e","r","t","z","u","i","o"],
    chords: keyboard[0]
  },
  {
    keys: ["a","s","d","f","g","h","j","k","l"],
    chords: keyboard[1]
  },
  {
    keys: ["y","x","c","v","b","n","m",",","."],
    chords: keyboard[2]
  }
];

const keyMap = new Map<
  string,
  {
    button: HTMLDivElement;
    label: HTMLDivElement;
  }
>();

window.addEventListener("DOMContentLoaded", () => {

  document.title = "My Little Chord Garden";

  const rows = document.querySelectorAll(".keys");

  layouts.forEach((layout, rowIndex) => {

    layout.chords.forEach((name, index) => {

      const wrapper = document.createElement("div");
      wrapper.style.display = "flex";
      wrapper.style.flexDirection = "column";
      wrapper.style.alignItems = "center";

      const label = document.createElement("div");
      label.style.height = "18px";
      label.style.fontSize = "13px";
      label.style.fontWeight = "600";

      const button = document.createElement("div");
      button.className = "key";

      const press = () => {
        button.classList.add("active");
        label.textContent = name;
      };

      const release = () => {
        button.classList.remove("active");
        label.textContent = "";
      };

      button.addEventListener("mousedown", press);
      button.addEventListener("mouseup", release);
      button.addEventListener("mouseleave", release);

      wrapper.append(label, button);
      rows[rowIndex].append(wrapper);

      keyMap.set(layout.keys[index], {
        button,
        label
      });

      button.dataset.chord = name;

    });

  });

});

document.addEventListener("keydown", (e) => {

  if (e.repeat) return;

  const item = keyMap.get(e.key.toLowerCase());

  if (!item) return;

  item.button.classList.add("active");
  item.label.textContent = item.button.dataset.chord ?? "";

});

document.addEventListener("keyup", (e) => {

  const item = keyMap.get(e.key.toLowerCase());

  if (!item) return;

  item.button.classList.remove("active");
  item.label.textContent = "";

});