local posts = json("shitposts/index.json");
render("shitposts/_index.html", "shitposts/index.html", { ["posts"] = posts });
