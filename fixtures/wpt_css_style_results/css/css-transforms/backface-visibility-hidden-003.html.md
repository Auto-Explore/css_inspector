# css/css-transforms/backface-visibility-hidden-003.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/backface-visibility-hidden-003.html"
}
```

## style[0]

```css

.outer {
  transform-style: preserve-3d;
  width: 100px;
  height: 100px;
  background-color: red;
}

.inner {
  backface-visibility: hidden;
}

.positioned {
  position: absolute;
  top: -100px;
  z-index: -1;
  width: 100px;
  height: 100px;
  background-color: green;
}

.sibling {
  width: 100px;
  height: 100px;
  background-color: blue;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
