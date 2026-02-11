# css/CSS2/pagination/allowed-page-breaks-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/allowed-page-breaks-006.xht"
}
```

## style[0]

```css

    /* Most permissible breaking possible */
    * { widows: 1; orphans: 1; widows: 0; orphans: 0; }

    /* Leave 2.5 lines at bottom to play with, independent of page size. */
    html, body {
      height: 100%;
      line-height: 1;
    }
    .spacer1 {
      height: 70%;
    }
    .spacer2 {
      height: 30%;
      margin-top: -2.5em;
    }

    .test {
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
