"""sup prototype
New sup will only parse single standup.yaml and report single project markdown.
I want to do it in Rust using yaml-serde and tera.
"""
import argparse
import jinja2
import yaml

STANDUP_FILE = "/home/tpreston/w/standup.yaml"
ME = {"name": "Thomas Preston", "username": "tpreston"}

def in_lower(a, b):
    return a.lower() in b.lower()

def get_member(search, members):
    """Returns the first matching member dict."""
    members.insert(0, {"name": "Discussion", "username": None})
    for m in members:
        if in_lower(search, m["name"]):
            return m
        if m["username"] and in_lower(search, m["username"]):
            return m
    return ""

def pr_standup(args):
    """Prints the standup markdown"""
    with open(STANDUP_FILE) as sfile:
        standups = yaml.load_all(sfile.read(), Loader=yaml.BaseLoader)

    standup = None
    for s in standups:
        if s['project'] == args.project:
            standup = s

    if args.me:
        standup["me"] = ME
    if args.next_person:
        standup["next_person"] = get_member(args.next_person, standup["members"])

    tloader = jinja2.FileSystemLoader("templates")
    tenv = jinja2.Environment(loader=tloader, lstrip_blocks=True)
    template = tenv.get_template("standup.jtpl")
    print(template.render(standup))

def get_args():
    """Returns program arguments"""
    parser = argparse.ArgumentParser(description="sup prototype")
    parser.add_argument("project", help="Project Code")
    parser.add_argument("-m", "--me", help="Print standup ",
                        action="store_true")
    parser.add_argument("-n", "--next-person", help="Next person")
    return parser.parse_args()

if __name__ == "__main__":
    pr_standup(get_args())
