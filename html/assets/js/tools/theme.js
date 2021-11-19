// Sets the color Theme to the given Value by passing a class to the :root element
const setTheme = (theme) => {
    document.documentElement.className = theme;
    localStorage.setItem('theme', theme);
}

// On load -> Set the Color Theme
window.matchMedia("(prefers-color-scheme: dark)").matches ? setTheme("dark") : setTheme("light");

// On load -> listen for prefers-color-scheme change
window.matchMedia("(prefers-color-scheme: dark)").addEventListener(
    "change",
    e => setTheme(e.matches ? "dark" : "light")
);

// On load -> Check if there is a theme stored already
var theme = localStorage.getItem('theme');
theme && setTheme(theme);

// Updates theme when changed by another tab (or console)
const themeUpdater = window.setInterval(() => {
    theme = localStorage.getItem('theme');
    if (theme !== null)
        setTheme(theme);
}, 500);