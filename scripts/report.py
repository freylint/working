import os

PKG_PATH = "./pkg"


def listing_path(listing):
    return os.path.join(PKG_PATH, listing)


def to_md_list(listings):
    accumulator = ""
    for listing in listings:
        accumulator += f"- {listing}\n"
    return accumulator[:-1]


def main():
    with open("assets/global_readme.template.md", "r") as f:
        readme = f.read().format(projects_list=make_pkg_list())
        with open("README.md", "w") as r:
            r.write(readme)


def make_pkg_list():
    accumulator = []
    maybe_pkg_list = os.listdir("./pkg")

    assert maybe_pkg_list != []

    for listing in maybe_pkg_list:
        if is_valid(listing):
            accumulator.append(parse_pkg(listing))
    return to_md_list(accumulator)


def is_valid(listing):
    if os.path.isdir(listing_path(listing)):
        return True
    else:
        return False
    raise NotImplementedError("Unimplemented file type.")


def parse_pkg(listing):
    if "README.md" in os.listdir(listing_path(listing)):
        # TODO pull first section of readme
        return listing
    else:
        return listing


if __name__ == "__main__":
    main()
