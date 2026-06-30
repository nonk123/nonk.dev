local projects = {
    {
        id = "klawiatura",
        title = "Klawiatura",
        description = "Mario Forever online, powered by <a href=\"/projects#nutblast\">NutBlast</a>",
        url = "https://github.com/toggins/Klawiatura",
        icon = {
            path = "klawiatura.png",
            alt = "A pixel-art picture of a keyboard"
        }
    },
    {
        id = "nutblast",
        title = "NutBlast",
        description =
        "WebRTC P2P networking for browser & desktop games. the <strong>successor</strong> of <a href=\"/projects#nutpunch\">NutPunch</a>.",
        url = "https://github.com/Schwungus/NutBlast",
        icon = {
            path = "NutBlast.png",
            alt = "A warning sign with a picture of a cracked peanut shell exploding dramatically",
            smooth = true
        }
    },
    {
        id = "nutpunch",
        title = "NutPunch",
        description =
        "UDP hole-punching & P2P networking for <strong>REAL</strong> men (and women). superseded by <a href=\"/projects#nutblast\">NutBlast</a>.",
        url = "https://github.com/Schwungus/NutPunch",
        icon = {
            path = "NutPunch.png",
            alt = "A warning sign with a picture of a cracked peanut shell",
            smooth = true
        }
    },
    {
        id = "sanity",
        title = "sanity",
        description =
            "the only sane static site generator in existence. see also: "
            .. "the <a href=\"/projects#vscode-sanity\">VSCode extension</a>",
        url = "https://github.com/nonk123/sanity"
    },
    {
        id = "lyrix",
        title = "Lyrix",
        description =
            "a based Firefox addon providing a context-menu button "
            .. "to find the lyrics for the played YouTube video",
        url = "https://github.com/nonk123/Lyrix"
    },
    {
        id = "poormans",
        title = "poormans",
        description = "realtime Win32 console graphics library",
        url = "https://github.com/nonk123/poormans"
    },
    {
        id = "caulk",
        title = "caulk",
        description = "<strong>the</strong> SteamAPI wrapper for plain C",
        url = "https://github.com/Schwungus/caulk",
        icon = {
            path = "caulk.png",
            alt = "A splattering of caulk in pixel art"
        }
    },
    {
        id = "promt-everything",
        title = "ProMT Everything",
        description =
            "a silly <a href=\"https://addons.mozilla.org/en-US/firefox/addon/promt-everything/\">Firefox</a>/Chrome extension that lets you translate a webpage through ProMT "
            .. "(the program that made the \"wasted\" GTA:SA translation) with the press of a button",
        url = "https://github.com/nonk123/promt-everything"
    },
    {
        id = "schwungus",
        title = "schwungus",
        description = "me and my homies form the schwungus collective",
        url = "https://schwung.us",
        icon = {
            path = "schwungus.png",
            alt = "A white-on-gray slithering mushroom",
            smooth = true
        }
    },
    {
        id = "vscode-sanity",
        title = "sanity liveserver",
        description = "a live-server for sanity inside Visual Studio Code",
        url = "https://marketplace.visualstudio.com/items?itemName=nonk123.vscode-sanity-liveserver"
    },
    {
        id = "structures",
        title = "S_tructures",
        description = "dynamic datastructures for us plain-C rawdoggers",
        url = "https://github.com/Schwungus/S_tructures"
    },
    {
        id = "vga9x16",
        title = "vga9x16",
        description = "the API for when you need a randomized blob of ASCII art",
        url = "https://vga9x16.ru",
        icon = {
            path = "vga9x16.png",
            alt = "Pseudo-random, full-bright VGA characters in a grid on a black background"
        }
    },
    {
        id = "kalym",
        title = "a local community website",
        description = "my first (and only) commercially backed project. but $50 is $50",
        url = "https://подработка-павлово.рф",
        icon = {
            path = "fiftybucks.webp",
            alt = "The text 'Podrabotka v Pavlovo' with a photo of Pavlovo in the background",
            smooth = true
        }
    }
};

render("projects/_index.html", "projects/index.html", { projects = projects });
