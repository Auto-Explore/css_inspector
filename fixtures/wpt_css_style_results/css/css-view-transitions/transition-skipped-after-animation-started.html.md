# css/css-view-transitions/transition-skipped-after-animation-started.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/transition-skipped-after-animation-started.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: target;
}

::view-transition-group(target) {
  animation-duration: 300s;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
