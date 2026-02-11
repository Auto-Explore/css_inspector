# css/css-contain/contain-html-overflow-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-html-overflow-002.html"
}
```

## style[0]

```css

html, body, p, div {
    margin: 0;
    width: 200px;
    height: 200px;
}
div { background: red; }
body {
    overflow: hidden;
}
html {
    height: 400px;
    contain: paint;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
