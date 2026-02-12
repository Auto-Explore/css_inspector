# css/css-viewport/zoom/resources/leaf.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/resources/leaf.html"
}
```

## style[0]

```css

body {
  background-color: aqua;
  --target-width: 32px;
  --target-height: 24px;
  --scale: 1;
  margin: calc(18px * var(--scale));
}
#target {
  width: calc(var(--target-width) * var(--scale));
  height: calc(var(--target-height) * var(--scale));
  background-color: hotpink;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
