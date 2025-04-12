local index = json("blog/index.json");

for id, article in pairs(index.articles) do
    local contents = read("blog/" .. id .. ".txt");
    local paragraphs = {}

    for paragraph in string.gmatch(contents, "[^\n]+[^\n]") do
        table.insert(paragraphs, paragraph);
    end

    local ctx = {
        id = id,
        date = article.date,
        title = article.title,
        paragraphs = paragraphs,
        all_articles = index.articles,
    };

    render("blog/_article.html", "blog/" .. id .. "/index.html", ctx);

    if id == index.default then
        render("blog/_article.html", "blog/index.html", ctx);
    end
end
