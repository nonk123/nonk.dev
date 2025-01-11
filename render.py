import os
import shutil
import json

from jinja2 import Environment, FileSystemLoader, select_autoescape

global env, blog, OUT_PATH

OUT_PATH = "out"

if os.path.exists(OUT_PATH):
    for child in os.listdir(OUT_PATH):
        child = os.path.join(OUT_PATH, child)

        if os.path.isfile(child):
            os.remove(child)
        else:
            shutil.rmtree(child)
else:
    os.mkdir(OUT_PATH)

os.mkdir(os.path.join(OUT_PATH, "blog"))

shutil.copy("robots.txt", OUT_PATH)
shutil.copy("favicon.png", OUT_PATH)
shutil.copy("style.css", OUT_PATH)

env = Environment(
    loader=FileSystemLoader("templates"),
    autoescape=select_autoescape()
)

def render(path, ctx = {}):
    out_path = os.path.join(OUT_PATH, path).rsplit(".", 1)[0]
    env.get_template(path).stream(ctx).dump(out_path)

with open("blog/index.json", "rt") as fp:
    blog = json.load(fp)

def render_article(id, index = False):
    article = blog["articles"][id]
    title = article["title"]
    date = article["date"]

    with open(os.path.join("blog", id + ".txt"), "rt") as fp:
        contents = fp.read()

    paragraphs = contents.strip().split("\n\n")
    fmt_articles = [{"id": id, "title": article["title"]} for id, article in blog["articles"].items()]

    out_path = os.path.join(OUT_PATH, "blog", "index.html" if index else id + ".html")
    ctx = {"id": id, "title": title, "date": date, "paragraphs": paragraphs, "articles": fmt_articles}
    env.get_template("article.html.j2").stream(ctx).dump(out_path)

render("index.html.j2")
render_article(blog["default"], True)

for id in blog["articles"].keys():
    render_article(id)

out_path = os.path.join(OUT_PATH, "sitemap.xml")
env.get_template("sitemap.xml.j2").stream(blog).dump(out_path)
