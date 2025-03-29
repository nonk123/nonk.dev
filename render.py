import os
import shutil
import json
import time

from jinja2 import Environment, FileSystemLoader, select_autoescape

global env, blog, OUT_PATH

OUT_PATH = "out"
MAKE_DIRS = ["blog", "projects"]

SHARED_CTX = {
    "last_updated": time.strftime("%Y-%m-%d", time.gmtime()),
}

if os.path.exists(OUT_PATH):
    for child in os.listdir(OUT_PATH):
        child = os.path.join(OUT_PATH, child)

        if os.path.isfile(child):
            os.remove(child)
        else:
            shutil.rmtree(child)
else:
    os.mkdir(OUT_PATH)

for directory in MAKE_DIRS:
    os.mkdir(os.path.join(OUT_PATH, directory))

shutil.copytree("assets", os.path.join(OUT_PATH, "assets"))
shutil.copy("robots.txt", OUT_PATH)

env = Environment(loader=FileSystemLoader("templates"), autoescape=select_autoescape())


def render(path, ctx={}):
    ctx = {**SHARED_CTX, **ctx}
    out_path = os.path.join(OUT_PATH, path).rsplit(".", 1)[0]
    env.get_template(path).stream(ctx).dump(out_path)


with open("blog/index.json", "rt") as fp:
    blog = json.load(fp)


def render_article(id, index=False):
    article = blog["articles"][id]

    with open(os.path.join("blog", id + ".txt"), "rt") as fp:
        contents = fp.read()

    paragraphs = contents.strip().split("\n\n")
    fmt_articles = [{"id": id, **article} for id, article in blog["articles"].items()]

    ctx = {
        **SHARED_CTX,
        "id": id,
        **article,
        "paragraphs": paragraphs,
        "articles": fmt_articles,
    }

    out_path = os.path.join(OUT_PATH, "blog", "index.html" if index else id + ".html")
    env.get_template("article.html.j2").stream(ctx).dump(out_path)


render("index.html.j2")
render("projects/index.html.j2")
render_article(blog["default"], True)

for id in blog["articles"].keys():
    render_article(id)

out_path = os.path.join(OUT_PATH, "sitemap.xml")
env.get_template("sitemap.xml.j2").stream(blog).dump(out_path)
