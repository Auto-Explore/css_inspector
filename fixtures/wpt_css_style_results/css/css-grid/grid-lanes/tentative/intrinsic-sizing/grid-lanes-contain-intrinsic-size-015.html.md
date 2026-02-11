# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-015.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-015.html"
}
```

## style[0]

```css

.panel {
  display: grid-lanes;
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
/* Size containment: the panel's used size is independent from its child's size */
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
