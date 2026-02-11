# css/css-transforms/backface-visibility-hidden-004.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/backface-visibility-hidden-004.html"
}
```

## style[0]

```css

.outer {
  transform-style: preserve-3d;
  transform: rotateX(180deg);
  width: 100px;
  height: 100px;
}

.card {
  width: 100px;
  height: 100px;
  position: absolute;
  backface-visibility: hidden;
}

.front {
  background-color: red;
}

.back {
  background-color: green;
  transform: rotateX(180deg);
}

.inner {
  position: fixed;
  width: 100px;
  height: 100px;
  background-color: red;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
