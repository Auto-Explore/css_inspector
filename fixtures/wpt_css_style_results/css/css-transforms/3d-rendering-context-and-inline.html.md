# css/css-transforms/3d-rendering-context-and-inline.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/3d-rendering-context-and-inline.html"
}
```

## style[0]

```css


.outer {
  display: block;
  width: 100px;
  height: 100px;
  transform-style: preserve-3d;
  transform: rotateX(90deg);
}

.middle {
  display: inline;
}

.inner {
  display: block;
  width: 100px;
  height: 100px;
  transform: rotateX(-90deg);
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
