local nav = json("_nav.json")
local sections_ctx = {}

for idx, section in pairs(nav) do
    sections_ctx[idx] = {
        url = section.href,
        lastmod = lastmod(section.lastmod_reference),
    }
end

render("_sitemap.xml", "sitemap.xml", {
    blog = json("blog/index.json"),
    sections = sections_ctx,
})
