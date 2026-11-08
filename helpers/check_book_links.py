"""Check local page, asset and fragment links in a generated mdBook."""

import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


class Page(HTMLParser):
    def __init__(self, source):
        super().__init__()
        self.ids = set()
        self.links = []
        self.feed(source)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if "id" in attrs:
            self.ids.add(attrs["id"])
        if tag == "a" and "name" in attrs:
            self.ids.add(attrs["name"])
        for key in ("href", "src"):
            if attrs.get(key):
                self.links.append(attrs[key])


def main():
    root = Path(sys.argv[1]).resolve()
    pages = {
        p.resolve(): Page(p.read_text(encoding="utf-8"))
        for p in root.rglob("*.html")
    }
    if root / "index.html" not in pages:
        raise SystemExit("Missing index.html")
    errors = set()
    checked = 0
    for path, page in pages.items():
        for link in page.links:
            url = urlsplit(link)
            if (url.scheme or url.netloc) and url.netloc != "norei3510.github.io":
                continue
            # mdBook's 404 template uses the configured public site prefix.
            prefix = "/100-exercises-to-learn-rust-ja/"
            if url.path.startswith(prefix):
                target = root / unquote(url.path[len(prefix):])
            elif url.path.startswith("/"):
                errors.add(f"{path.relative_to(root)}: unexpected root URL {link}")
                continue
            else:
                target = path.parent / unquote(url.path) if url.path else path
            target = target.resolve()
            if target.is_dir():
                target /= "index.html"
            checked += 1
            if not target.is_relative_to(root) or not target.is_file():
                errors.add(f"{path.relative_to(root)}: missing file {link}")
            elif url.fragment and target in pages:
                if unquote(url.fragment) not in pages[target].ids:
                    errors.add(f"{path.relative_to(root)}: missing anchor {link}")
    if errors:
        for error in sorted(errors):
            print(error, file=sys.stderr)
        raise SystemExit(f"{len(errors)} broken internal links")
    print(f"Checked {len(pages)} HTML pages and {checked} internal links: OK")


if __name__ == "__main__":
    main()
