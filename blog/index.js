addEventListener("popstate", (event) => applyState(event.state));

const defaultState = { file: "/blog/articles/hello.txt", article: { title: "nonk's blog" } };
history.replaceState(defaultState, "")
applyState(defaultState);

fetch("/blog/listing.json").then((x) => x.json()).then(initArticlesIndex);

function applyState(state) {
    fetch(state.file).then((r) => r.text()).then((content) => {
        setArticle(state.article.title, content);
    });
}

function initArticlesIndex(articles) {
    const maxTitleLen = 24;
    const linksRoot = document.getElementById("links");

    for (const article of articles) {
        const link = document.createElement("a");
        link.style.cursor = "pointer";

        const file = `/blog/articles/${article.date}.txt`;

        link.onclick = () => {
            const state = { article: article, file: file };
            history.pushState(state, "");
            applyState(state);
        };

        if (article.title.length > maxTitleLen) {
            link.textContent = article.title.slice(0, maxTitleLen - 3).trim() + "...";
        } else {
            link.textContent = article.title;
        }

        linksRoot.appendChild(link);
    }
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
