local section_names = { "/", "/about", "/now", "/projects", "/blog", "/guestbook", "/shitposts" }
local sections = {}

for idx, path in pairs(section_names) do
    sections[idx] = {
        path = path,
        lastmod = lastmod(path:sub(2)),
    }
end

render("_sitemap.xml", "sitemap.xml", {
    blog = json("blog/index.json"),
    sections = sections,
})
