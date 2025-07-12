local sections = { "/", "/about", "/now", "/projects", "/shitposts", "/guestbook" };
local realsections = {}

for _, path in pairs(sections) do
    table.insert(realsections, {
        path = path,
        lastmod = lastmod(path:sub(2)),
    });
end

render("_sitemap.xml", "sitemap.xml", {
    blog = json("blog/index.json"),
    sections = realsections,
});
