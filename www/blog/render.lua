local index = json("blog/index.json")

for id, article in pairs(index.articles) do
    local contents = read("blog/md/" .. id .. ".txt")
    local paragraphs = {}

    contents = contents:gsub("(:?^|\n) +([^\n]+)", "%1")
    contents = contents:gsub("([^\n])\n([^\n])", "%1 %2")

    for paragraph in contents:gmatch("[^%c]+") do
        table.insert(paragraphs, paragraph)
    end

    local ctx = {}
    local addendum = {
        id = id,
        default = index.default,
        date_mod = article.date_mod or article.date_pub,
        paragraphs = paragraphs,
        articles = index.articles,
    }
    for k, v in pairs(article) do
        ctx[k] = v
    end
    for k, v in pairs(addendum) do
        ctx[k] = v
    end

    render("blog/_article.html", "blog/" .. id .. "/index.html", ctx)
    if id == index.default then
        render("blog/_article.html", "blog/index.html", ctx)
    end
end
