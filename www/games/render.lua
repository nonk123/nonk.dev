local games = {
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
        -- TODO: add icon(s)
    },
    {
        id = "mario",
        title = "Mario Together",
        description = "Mario Forever with <strong>P2P multiplayer</strong>.",
        github = "https://github.com/toggins/Klawiatura",
        play = "https://mario.games.nonk.dev",
        -- TODO: add icon(s)
    },
};

render("games/_index.html", "games/index.html", { games = games });
