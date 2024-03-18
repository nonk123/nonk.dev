const articles = [
    {
        date: "24-03-18",
        title: "IDEs juxtapose the UNIX philosophy"
    }
];

const maxTitleLen = 14;
const linksRoot = document.getElementById("links");

for (const article of articles) {
    const link = document.createElement("a");

    const file = `/articles/${article.date}.txt`;
    link.onclick = () => { fetch(file).then((r) => r.text()).then((content) => setArticle(article.title, content)); };

    if (article.title.length > maxTitleLen) {
    	link.textContent = article.title.slice(0, maxTitleLen).trim() + "...";
    } else {
        link.textContent = article.title;
    }

    linksRoot.appendChild(link);
}

function setArticle(title, content) {
    const paragraphs = content.split("\n\n");

    const ps = paragraphs.map((par) => {
		const p = document.createElement("p");
		p.textContent = par;
		return p;
    });

    const root = document.getElementById("content");
    root.textContent = "";

	const header = document.createElement("h1");
	header.textContent = title;
    root.appendChild(header);

    ps.forEach((p) => root.appendChild(p));
}

setArticle("nonk's blog", "Hi and welcome to my blog!\n\nCheck out some of my spiciest articles by clicking the links on the right.");
