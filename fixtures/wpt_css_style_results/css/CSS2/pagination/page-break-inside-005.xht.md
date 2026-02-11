# css/CSS2/pagination/page-break-inside-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/page-break-inside-005.xht"
}
```

## style[0]

```css

    /* Most permissible breaking possible */
    * { widows: 1; orphans: 1; widows: 0; orphans: 0; }

    /* Leave 2.5 lines at bottom to play with, independent of page size. */
    html, body {
      height: 100%;
      margin: 0;
      line-height: 1;
    }
    .spacer1 {
      height: 35%;
    }
    .spacer2 {
      height: 15%;
      margin-top: -2.5em;
    }

    .avoid {
      page-break-inside: avoid;
      height: 200%;
    }
    .test div {
      white-space: pre;
      /* Make test text more visible */
      color: blue;
      font-weight: bold;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
