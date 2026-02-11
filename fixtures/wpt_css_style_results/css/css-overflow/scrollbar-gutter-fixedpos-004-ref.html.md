# css/css-overflow/scrollbar-gutter-fixedpos-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-fixedpos-004-ref.html"
}
```

## style[0]

```css

:root {
  scrollbar-gutter: stable both-edges;
  writing-mode: vertical-lr;
}
body {
  margin: 0;
}
.box {
  width: 100px;
  height: 100px;
  background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-gutter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
