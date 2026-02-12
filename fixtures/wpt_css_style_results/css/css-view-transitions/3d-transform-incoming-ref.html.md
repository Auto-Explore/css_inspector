# css/css-view-transitions/3d-transform-incoming-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/3d-transform-incoming-ref.html"
}
```

## style[0]

```css

div { box-sizing: border-box; will-change: transform }
.wrap_perspective {
  perspective: 100px;
  width: max-content;
  transform: translate(200px);
}
.rotatex {
  transform-style: preserve-3d;
  transform: rotateX(20deg);
  background: blue;
}
.shared {
  width: 100px;
  height: 100px;
  view-transition-name: shared;
}
body { background: pink }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
