# css/css-pseudo/highlight-painting-003.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-003.html"
}
```

## style[0]

```css

    /*
        Topmost last:
        1.  originating background (grey)
        2.  unselected text (red)
        3.  originating line-through on unselected text (yellow)
        4.  ::selection background (green)
        5.  ::selection underline on selected text (blue)
        6.  selected text (purple)
        7.  originating line-through on selected text (purple!)

        Decoration paints are ordered by text-decoration-line
        (underline, overline, text, line-through), then by highlight
        overlay (originating, ::grammar-error, ::spelling-error,
        ::target-text, ::selection).
    */
    *, *::selection {
        text-decoration-skip-ink: none;
        text-decoration-skip: none;
    }
    main {
        font-size: 7em;
        margin: 0.5em;
        width: min-content;
        height: 0.25em;
        background: #707070C0;
        color: #E03838C0;
        text-decoration: #C0C000C0 solid line-through;
    }
    main::selection {
        background: #38E038C0;
        text-decoration: #3838E0C0 solid underline;
        color: #663399C0;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
