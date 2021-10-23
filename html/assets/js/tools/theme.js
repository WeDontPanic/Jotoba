// Sets the color Theme to the given Value by passing a class to the :root element
const setTheme = (theme) => {
    document.documentElement.className = theme;
    localStorage.setItem('theme', theme);
}

// On load -> Prepare the Color Theme
window.matchMedia("(prefers-color-scheme: dark)").addEventListener(
    "change", 
     e => e.matches && setTheme("dark")
);      

// On load -> Check if there is a theme stored already
const theme = localStorage.getItem('theme');
theme && setTheme(theme);