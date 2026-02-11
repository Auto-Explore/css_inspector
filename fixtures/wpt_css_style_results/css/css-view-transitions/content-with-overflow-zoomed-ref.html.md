# css/css-view-transitions/content-with-overflow-zoomed-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-with-overflow-zoomed-ref.html"
}
```

## style[0]

```css

.target {
  width: 80px;
  height: 80px;
  background: blue;
  view-transition-name: target;
  zoom: 1.5;
  border: 2px solid black;
}
.child {
  width: 200px;
  height: 200px;
  position: relative;
  top: 50px;
  left: 50px;
  background: green;
  zoom: 1.2;
}
body { background: lightpink; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
