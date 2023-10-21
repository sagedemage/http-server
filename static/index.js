/* Navigation bar */
document.getElementById("nav").innerHTML = /*html*/`
    <nav id="navbar">
        <span>Salmaan Saeed</span>
        <a href="/">Home</a>
        <a href="/about">About</a>
    </nav>
`

/* Footer */
document.getElementById("footer").innerHTML = /*html*/`
    <footer>
        &copy; <span id="year"></span> Salmaan Saeed 
    </footer>
`

const year = new Date().getFullYear();

document.getElementById("year").innerHTML = year