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
        id = "promt",
        title = "ProMT Playground",
        description = "ProMT wasted-translations playground.",
        play = "/games/promt",
    },
    {
        id = "verlet",
        title = "Verlet Swing",
        description = "An arcade rope-swinging game with high-scores! Powered by Verlet integration.",
        github = "https://github.com/nonk123/VerletSwing",
        play = "https://verlet.games.nonk.dev",
    },
};

render("games/_index.html", "games.html", { games = games });
