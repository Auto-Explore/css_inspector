# css/css-view-transitions/massive-element-on-top-of-viewport-offscreen-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/massive-element-on-top-of-viewport-offscreen-ref.html"
}
```

## style[0]

```css

.target_at_bottom_edge {
  position: fixed;
  inset-block-end: 0;
  inset-inline-start: 0;
}

.target {
  contain: paint;
  inline-size: 100px;
  block-size: 40000px;
  view-transition-name: target;
}

.top {
  inline-size: 100%;
  block-size: 100px;
  background: lightblue;
}

.middle {
  inline-size: 100%;
  block-size: 39800px;
  background: green;
}

.bottom {
  inline-size: 100%;
  block-size: 100px;
  background: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
