# css/css-view-transitions/new-content-captures-root-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-captures-root-ref.html"
}
```

## style[0]

```css

.box {
  background: lightgreen;
  width: 100px;
  height: 100px;
  contain: paint;
  position: absolute;
  will-change: transform;
}
#e1 {
  top: 10px;
  left: 30px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
