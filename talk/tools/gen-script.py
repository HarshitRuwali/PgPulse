#!/usr/bin/env python3
"""Regenerate SCRIPT.md from the speaker notes in pgpulse-rust.html.

The deck is the source of truth. Edit <aside class="notes"> in the HTML,
then run this to refresh SCRIPT.md:

    python3 tools/gen-script.py
"""
import html
import re
from pathlib import Path

TALK = Path(__file__).resolve().parent.parent
DECK = TALK / "pgpulse-rust.html"
OUT = TALK / "SCRIPT.md"

HEADER = """# Speaking notes

**Generated from `pgpulse-rust.html`. Do not edit by hand.**
The deck's `<aside class="notes">` blocks are the source of truth, and they are what you see in
reveal's speaker view (press `S`). After changing the deck, run:

```bash
python3 tools/gen-script.py
```

Rust meetup, first talk. {n} slides, all horizontal: the right arrow walks every one.

Rehearse from this. Do not read it on stage.

---
"""

FOOTER = """## Timing

Roughly 20 minutes of talking in a 25 to 30 minute slot. Checkpoints:

| Time | Slide |
| --- | --- |
| 5:00 | 6, what an extension actually is |
| 10:00 | 12, background worker |
| 15:00 | 17, assumption 1 |
| 24:00 | 26, demo |

Behind at 15:00, cut slide 24 (the smaller cuts). Behind at 20:00, cut 25 as well and go
straight to the demo.

## First-talk notes

- Have the PDF open as well as the HTML. If the browser misbehaves you lose thirty seconds, not
  the talk.
- Every slide is horizontal. The right arrow walks all of them, nothing is hidden in a stack.
- The three honesty lines are deliberate: you never bisected the segfault to the faulting
  instruction, you never root-caused the client-library failure, and you observed the
  `IsBackgroundWorker` guard failing without proving why. Keep them.
- "I don't know" is a complete answer at a meetup.
"""


def plain(fragment: str) -> str:
    """Strip tags, but keep <br> as a space so words do not run together."""
    fragment = re.sub(r"<br\s*/?>", " ", fragment)
    return re.sub(r"\s+", " ", html.unescape(re.sub(r"<[^>]+>", "", fragment))).strip()


def main() -> None:
    src = DECK.read_text()
    body = src[src.index('<div class="slides">'): src.index("\n</div>\n</div>")]

    slides = list(re.finditer(r"^<section([^>]*)>\n(.*?)^</section>$", body, re.S | re.M))
    parts = [HEADER.format(n=len(slides))]

    for index, match in enumerate(slides, start=1):
        block = match.group(2)

        title_match = re.search(r"<h[123][^>]*>(.*?)</h[123]>", block, re.S) or re.search(
            r'<p class="statement">(.*?)</p>', block, re.S
        )
        title = plain(title_match.group(1)) if title_match else "(code slide)"

        labels = []
        for pattern in (
            r'<span class="num">(.*?)</span>',
            r'<p class="kicker[^"]*">(.*?)</p>',
            r'<p class="beat \w+">(.*?)</p>',
        ):
            found = re.search(pattern, block, re.S)
            if found:
                labels.append(plain(found.group(1)))

        heading = " · ".join(labels + [title]) if labels else title
        parts.append(f"### {index}. {heading}\n")

        notes = re.search(r'<aside class="notes">(.*?)</aside>', block, re.S)
        if notes:
            text = html.unescape(re.sub(r"<[^>]+>", "", notes.group(1)))
            text = "\n".join(line.strip() for line in text.strip().split("\n"))
            parts.append(re.sub(r"\n{3,}", "\n\n", text) + "\n")
        else:
            parts.append("_No speaker notes on this slide._\n")

        parts.append("---\n")

    parts.append(FOOTER)
    OUT.write_text("\n".join(parts))
    print(f"wrote {OUT.relative_to(TALK)} from {len(slides)} slides")


if __name__ == "__main__":
    main()
