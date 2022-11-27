var flags = {
    user: JSON.parse(localStorage.getItem('user')) || null
}

var app = Elm.Main.init({ flags: flags })

app.ports.outgoing.subscribe(({ tag, content }) => {
    switch (tag) {
        case 'storeUser':
            return localStorage.setItem('user', JSON.stringify(content))
        case 'clearUser':
            return localStorage.removeItem('user')
        default:
            return console.warn(`Unrecognized Port`, tag)
    }
})
