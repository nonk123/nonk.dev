initLastUpdatedDate();

addEventListener("popstate", (event) => applyState(event.state));

const defaultState = { file: "/blog/articles/hello.txt", article: { title: "nonk's blog" } };
history.replaceState(defaultState, "")
applyState(defaultState);

fetch("/blog/listing.json", { cache: "no-store" })
    .then((x) => x.json())
    .then(initArticlesIndex);

function applyState(state) {
    fetch(state.file, { cache: "no-store" })
        .then((r) => r.text())
        .then((content) => setArticle(state.article.title, content));
}

function initArticlesIndex(articles) {
    const maxTitleLen = 20;
    const linksRoot = document.getElementById("links");

    for (const article of articles) {
        const root = document.createElement("li");

        const link = document.createElement("a");
        link.style.cursor = "pointer";
        root.appendChild(link);

        const file = `/blog/articles/${article.date}.txt`;

        link.onclick = () => {
            const state = { article: article, file: file };
            history.pushState(state, "");
            applyState(state);
        };

        if (article.title.length > maxTitleLen) {
            link.textContent = article.title.slice(0, maxTitleLen - 3).trim();
            root.appendChild(document.createTextNode("..."));
        } else {
            link.textContent = article.title;
        }

        linksRoot.appendChild(root);
    }

    const main = document.getElementsByTagName("main").item(0);
    main.classList.remove("no-fade-in");
}

function setArticle(title, content) {
    const paragraphs = content.split("\n\n");

    const ps = paragraphs.map((par) => {
        const p = document.createElement("p");
        p.textContent = par;
        return p;
    });

    const root = document.getElementsByTagName("article").item(0);
    root.textContent = "";

    const header = document.createElement("h1");
    header.textContent = title;
    root.appendChild(header);

    // XXX: can't terse this up to just `root.appendChild` due to JS retardation...
    ps.forEach((p) => root.appendChild(p));
}

function initLastUpdatedDate() {
    /// NOTE: I need lowercase English month names regardless of the user's locale.
    const monthFmt = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];

    const date = new Date(document.lastModified);
    const monthName = monthFmt[date.getMonth()];
    const year = date.getFullYear();

    const lastUpdated = document.getElementById("last-updated");
    const fancy = `${monthName} ${year}`;
    lastUpdated.textContent = fancy;
}
