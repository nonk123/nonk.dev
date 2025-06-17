local posts = json("shitposts/index.json");
local index = { ["posts"] = posts };

for _, value in pairs(posts) do
    value.href = "/shitposts/" .. value.file;
end

render("shitposts/_index.html", "shitposts/index.html", index);
