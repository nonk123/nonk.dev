const browsers = require("playwright");
const paths = ["/", "/projects", "/about"];

(async () => {
    const browser = await browsers.chromium.launch();
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.setViewportSize({ width: 1366, height: 768 });

    const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
    for (const idx in paths) {
        await page.goto(`https://nonk.dev${paths[idx]}`);
        await page.waitForEvent("requestfinished");

        await delay(1000);
        await page.screenshot({ path: `.github/assets/screenie-${idx}.png` });
    }

    await browser.close();
})();
