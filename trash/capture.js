const pt = require("puppeteer");
const paths = ["/", "/projects", "/about"];

const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

pt.launch().then(async (browser) => {
    const p = await browser.newPage();
    await p.setViewport({ width: 1366, height: 768 });

    for (const idx in paths) {
        const path = paths[idx];
        await p.goto(new URL(`localhost:8000${path}`), {
            waitUntil: "domcontentloaded",
        });

        delay(1000).then(async () => {
            let filename = `.github/assets/screenie-${idx}.png`;
            await p.screenshot({ path: filename });
        });
    }

    await browser.close();
});
