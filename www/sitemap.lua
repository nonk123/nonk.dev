local sections = { "/", "/about", "/now", "/projects", "/blog", "/guestbook", "/shitposts" };
for idx, path in pairs(sections) do
    sections[idx] = {
        path = path,
        lastmod = lastmod(path:sub(2)),
    };
end

render("_sitemap.xml", "sitemap.xml", {
    blog = json("blog/index.json"),
    sections = sections,
});
