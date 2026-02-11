# css/css-view-transitions/transform-origin-view-transition-group-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/transform-origin-view-transition-group-ref.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 150px;
  background: green;
  position: fixed;
  top: 200px;
  left: 200px;
  transform: rotate(90deg);
  view-transition-name: target;
  clip-path: inset(1px 1px 1px 1px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
