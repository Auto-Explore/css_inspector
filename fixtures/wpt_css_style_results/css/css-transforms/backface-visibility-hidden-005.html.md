# css/css-transforms/backface-visibility-hidden-005.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/backface-visibility-hidden-005.html"
}
```

## style[0]

```css

.background {
  width: 100px;
  height: 100px;
  background-color: green;
}

.flipper {
  backface-visibility:hidden;
  transform: rotateY(180deg);
  width: 100px;
  top: -100px;
  margin-top: -100px;
}

.scroll {
  overflow-y: scroll;
  width: 100px;
  height: 100px;
}

.inner {
  height: 900px;
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
