# css/css-values/min-length-percent-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/min-length-percent-001.html"
}
```

## style[0]

```css

html, body { margin: 0px; padding: 0px; }
#parent { width: 400px; }
#target {
  width: min(300px, 25% + 100px, 50px + 50%);
  height: 200px;
  background: green;
}
#fail {
  width: 200px;
  height: 200px;
  position: absolute;
  z-index: -1;
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
