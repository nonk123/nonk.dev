import os
import shutil

from jinja2 import Environment, FileSystemLoader, select_autoescape

global env, OUT_PATH

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

shutil.copy("robots.txt", OUT_PATH)
shutil.copy("sitemap.xml", OUT_PATH) # TODO: templatize
shutil.copy("favicon.png", OUT_PATH)
shutil.copy("style.css", OUT_PATH)

env = Environment(
    loader=FileSystemLoader("templates"),
    autoescape=select_autoescape()
)

def render(path, ctx = {}):
    out_path = os.path.join(OUT_PATH, path).rsplit(".", 1)[0]
    env.get_template(path).stream(ctx).dump(out_path)

render("index.html.j2")
