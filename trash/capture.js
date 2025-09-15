const paths = ["/", "/projects", "/about"];
const viewports = [
    {
        name: "landscape",
        width: 1366,
        height: 768,
    },
    {
        name: "portrait",
        width: 720,
        height: 1280,
    },
];

const browsers = require("playwright");
(async () => {
    const browser = await browsers.chromium.launch();
    const context = await browser.newContext();
    const page = await context.newPage();

    const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
    for (const idx in paths) {
        const path = paths[idx];
        await page.goto(`https://nonk.dev${path}`);
        await page.waitForEvent("requestfinished");

        for (const viewport of viewports) {
            await page.setViewportSize(viewport);
            await delay(1000);
            await page.screenshot({
                path: `.github/assets/screenie-${viewport.name}-${idx}.png`,
                scale: "css",
            });
            console.info(`Screenied '${path}' in ${viewport.name} mode`);
        }
    }

    await browser.close();
})();
