"""Add Japanese exercise links using mdBook's preprocessor JSON protocol."""

import json
import re
import sys
from pathlib import Path, PurePosixPath


def add_exercise_links(context, book):
    config = context["config"]
    root_url = config["preprocessor"]["exercise-linker"]["exercise_root_url"].rstrip("/")
    book_root = Path(context["root"]).resolve()
    source_root = (book_root / config.get("book", {}).get("src", "src")).resolve()
    exercises_root = book_root.parent / "exercises"

    def visit(items):
        for item in items:
            if not isinstance(item, dict) or "Chapter" not in item:
                continue
            chapter = item["Chapter"]
            visit(chapter.get("sub_items", []))
            source = chapter.get("source_path")
            if not source:
                continue
            source_path = PurePosixPath(source.replace("\\", "/"))
            if not re.match(r"^\d{2}_", source_path.as_posix()):
                continue
            if source_path.is_absolute() or ".." in source_path.parts or source_path.suffix != ".md":
                raise ValueError(f"Invalid exercise chapter path: {source}")
            if not (source_root / source_path).is_file():
                raise ValueError(f"Chapter source does not exist: {source_root / source_path}")
            exercise_path = source_path.with_suffix("")
            if not (exercises_root / exercise_path).is_dir():
                raise ValueError(f"Exercise directory does not exist: {exercises_root / exercise_path}")

            path = exercise_path.as_posix()
            link = f"このセクションの演習は [`{path}`]({root_url}/{path}) にあります。"
            content = chapter["content"]
            heading = re.search(r"(?m)^##[ \t]+(?:演習|Exercise)[ \t]*\r?$", content)
            anchor = "" if re.search(r"\bid=[\"']exercise[\"']", content) else '<a id="exercise"></a>\n\n'
            if heading:
                content = content[:heading.start()] + anchor + "## 演習\n\n" + link + "\n" + content[heading.end():]
            else:
                content = content.rstrip() + "\n\n" + anchor + "## 演習\n\n" + link + "\n"
            chapter["content"] = content

    items = book.get("items", book.get("sections"))
    if not isinstance(items, list):
        raise ValueError("mdBook JSON has no chapter list (items or sections)")
    visit(items)
    return book


def main():
    if len(sys.argv) > 1:
        return 0 if sys.argv[1:] == ["supports", "html"] else 1
    sys.stdin.reconfigure(encoding="utf-8")
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    try:
        context, book = json.load(sys.stdin)
        json.dump(add_exercise_links(context, book), sys.stdout, ensure_ascii=False)
        sys.stdout.write("\n")
    except (KeyError, TypeError, ValueError, OSError) as error:
        print(f"exercise-linker: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
