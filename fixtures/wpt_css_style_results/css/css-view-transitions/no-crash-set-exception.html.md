# css/css-view-transitions/no-crash-set-exception.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/no-crash-set-exception.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: shared;
}

html::view-transition,
html::view-transition-group(shared),
html::view-transition-image-pair(shared),
html::view-transition-old(shared),
html::view-transition-new(shared) {
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
