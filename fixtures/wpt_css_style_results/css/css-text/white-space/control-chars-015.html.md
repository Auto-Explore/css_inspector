# css/css-text/white-space/control-chars-015.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/control-chars-015.html"
}
```

## style[0]

```css

div {
  font-size: 4em;
}
div::after { content: "\0015" } /* Injecting via CSS, to avoid any mangling by the html parser */
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
