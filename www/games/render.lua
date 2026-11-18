local games = {
    {
        id = "mario",
        title = "Mario Together",
        description = "Mario Forever with <strong>P2P online multiplayer</strong>.",
        github = "https://github.com/toggins/Klawiatura",
        play = "https://mario.games.nonk.dev",
        screenies = { "klawiatura1.png", "klawiatura2.png" },
    },
    {
        id = "verlet",
        title = "Verlet Swing",
        description = "A rope-swinging racing game, powered by Verlet integration!",
        github = "https://github.com/nonk123/VerletSwing",
        play = "https://verlet.games.nonk.dev",
        screenies = { "verlet1.png" },
    },
    {
        id = "promt",
        title = "ProMT Playground",
        description = "ProMT wasted-translations playground.",
        play = "/games/promt",
    },
};

render("games/_index.html", "games.html", { games = games });
