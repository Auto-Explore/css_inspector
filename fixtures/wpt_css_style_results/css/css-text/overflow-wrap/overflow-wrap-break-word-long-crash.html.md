# css/css-text/overflow-wrap/overflow-wrap-break-word-long-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text/overflow-wrap/overflow-wrap-break-word-long-crash.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
}
body {
  overflow-wrap: break-word;
  width: 2147483648px;
}
div {
  /* Double the width in case CSS parser clamps the body width. */
  width: 200%;
}
span {
  border-left: 2147483648px solid;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
