// Sets the color Theme to the given Value by passing a class to the :root element
const setTheme = (theme) => {
    document.documentElement.className = theme;
    localStorage.setItem('theme', theme);

      Util.setMdlCheckboxState("use_dark_mode_settings", theme === "dark")
}

// On load -> Set the Color Theme
window.matchMedia("(prefers-color-scheme: dark)").matches ? setTheme("dark") : setTheme("light");

// On load -> listen for prefers-color-scheme change
window.matchMedia("(prefers-color-scheme: dark)").addEventListener(
    "change",
    e => setTheme(e.matches ? "dark" : "light")
);

// On load -> Check if there is a theme stored already
let theme = localStorage.getItem('theme');
theme && setTheme(theme);

// Updates theme when changed by another tab (or console)
window.addEventListener("storage", ()=>{
  let newTheme = localStorage.getItem("theme");

  if (newTheme)
    setTheme(newTheme);
})