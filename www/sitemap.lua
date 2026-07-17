local sections = {
    { url = "",          file = "index.html.j2" },
    { url = "about",     file = "about.html.j2" },
    { url = "now",       file = "now.html.j2" },
    { url = "projects",  file = "projects" },
    { url = "blog",      file = "blog" },
    { url = "guestbook", file = "guestbook.html.j2" },
    { url = "shitposts", file = "shitposts" }
};

local sections_ctx = {}

for idx, section in pairs(sections) do
    sections_ctx[idx] = {
        url = section.url,
        lastmod = lastmod(section.file),
    }
end

render("_sitemap.xml", "sitemap.xml", {
    blog = json("blog/index.json"),
    sections = sections_ctx,
})
