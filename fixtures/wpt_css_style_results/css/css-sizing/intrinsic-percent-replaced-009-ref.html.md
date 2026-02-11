# css/css-sizing/intrinsic-percent-replaced-009-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/intrinsic-percent-replaced-009-ref.html"
}
```

## style[0]

```css

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
}

body {
  padding: 20px;
  position: relative;
}

.parent {
  border: 3px solid green;
  position: absolute;
  height: 60vh;
}

.parent img {
  height: calc(60vh - 6px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
