const pt = require("puppeteer");
const paths = ["/", "/projects", "/about"];

const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

pt.launch({ headless: "new" }).then(async (browser) => {
    const p = await browser.newPage();
    await p.setViewport({ width: 1366, height: 768 });

    for (const idx in paths) {
        const path = paths[idx];

        await p.goto(new URL(`https://nonk.dev${path}`), {
            waitUntil: "domcontentloaded",
        });
        await delay(1000);

        let filename = `.github/assets/screenie-${idx}.png`;
        await p.screenshot({ path: filename });
    }

    await browser.close();
});
