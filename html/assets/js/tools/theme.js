const themeEvent = new Event("theme-changed");

// Sets the color Theme to the given Value by passing a class to the :root element
const setTheme = (theme) => {
  document.documentElement.className = theme;
  localStorage.setItem('theme', theme);

  Util.setMdlCheckboxState("use_dark_mode_settings", theme === "dark")
  document.dispatchEvent(themeEvent);
}

// Updates theme when changed by another tab (or console)
window.addEventListener("storage", () => {
  let targetTheme = localStorage.getItem("theme");
  if (targetTheme) {
    setTheme(targetTheme);
  }
})

// Set theme from localStorage (if set)
if (localStorage.getItem('theme')) {
  setTheme(localStorage.getItem('theme'));
}

// Else, set based on prefered color scheme
else {
  Util.awaitDocumentReady(() => {
    window.matchMedia("(prefers-color-scheme: dark)").matches ? setTheme("dark") : setTheme("light");
  });
}

// listen for prefers-color-scheme changes
window.matchMedia("(prefers-color-scheme: dark)").addEventListener(
  "change",
  e => setTheme(e.matches ? "dark" : "light")
);