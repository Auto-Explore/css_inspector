# css/css-paint-api/column-count-crash.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/column-count-crash.https.html"
}
```

## style[0]

```css

#output {
  width: 100px;
  height: 100px;
}
body {
  -webkit-mask-box-image-source: url(resources/dot.png);
  column-count: 3;
}
.test-wait {
  column-count: 600;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
