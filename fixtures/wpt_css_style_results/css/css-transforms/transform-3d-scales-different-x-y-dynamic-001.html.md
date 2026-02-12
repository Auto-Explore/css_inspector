# css/css-transforms/transform-3d-scales-different-x-y-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-3d-scales-different-x-y-dynamic-001.html"
}
```

## style[0]

```css

:root {
  overflow: hidden;
}
.horizontal, .vertical {
  position: absolute;
  top: 0;
  left: 0;
}
.vertical {
  writing-mode: vertical-lr;
}
.test {
  transform-origin: 0 0;
  line-height: 20px;
  block-size: 20px;
  inline-size: max-content;
  border: 5px solid;
  border-color: blue orange;
  margin-inline-start: calc(30px * var(--index));
}
.horizontal > .test {
  transform: scale3d(var(--scale), 1, 1);
}
.vertical > .test {
  transform: scale3d(1, var(--scale), 1);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
