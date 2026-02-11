# css/css-overflow/scrollbar-gutter-fixedpos-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-fixedpos-002.html"
}
```

## style[0]

```css

:root {
  scrollbar-gutter: stable both-edges;
}
body {
  margin: 0;
}
.fixed {
  width: 100px;
  height: 100px;
  background: green;
  position: fixed;
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
