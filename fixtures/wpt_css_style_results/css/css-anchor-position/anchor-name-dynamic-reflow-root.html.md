# css/css-anchor-position/anchor-name-dynamic-reflow-root.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-dynamic-reflow-root.html"
}
```

## style[0]

```css

body { margin: 0 }
.tabs {
  display: flex;
}
.tab {
  height: 20px;
  width: 100px;
}
.cur {
  anchor-name: --current-tab;
}
.highlight {
  position: absolute;
  width: anchor-size(width);
  left: anchor(left);
  right: anchor(right);
  height: 100px;
  background: green;
  top: anchor(bottom);
  position-anchor: --current-tab;
  transform: scaleY(1); /* This is important to test the bug */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
