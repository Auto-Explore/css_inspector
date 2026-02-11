# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-015-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-015-ref.html"
}
```

## style[0]

```css

.panel {
  display: grid;
  background: red;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 12px;
  overflow: visible;
  position: relative;
  width: min-content;
}
.box {
  background: green;
  padding: 8px 10px;
  border-radius: 6px;
}
.panel.size {
  contain: size;
}
.panel.size .box.large {
  width: 600px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
