# css/css-sizing/aspect-ratio/block-aspect-ratio-058.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/aspect-ratio/block-aspect-ratio-058.html"
}
```

## style[0]

```css

#reference-overlapped-green {
  position: absolute;
  background-color: green;
  width: 100px;
  height: 100px;
  z-index: -1;
}
.outer {
  width: min-content;
  max-height: 100px;
  background: red;
}
.target {
  aspect-ratio: 1/1;
  height: 100%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
