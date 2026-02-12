# css/css-ui/cursor-auto-002-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-auto-002-manual.html"
}
```

## style[0]

```css

a {
  cursor: url("support/cursors/fail.png"), help; /* Override UA styles, regardless of specificity */
  cursor: auto !important; /* Override UA styles, regardless of specificity */
  color: blue;
  text-decoration: none; /* Having the link not standout as being a link, to avoid distracting the tester */
}
p {
  cursor: text;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
