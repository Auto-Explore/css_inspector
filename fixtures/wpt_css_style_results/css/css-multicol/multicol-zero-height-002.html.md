# css/css-multicol/multicol-zero-height-002.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-zero-height-002.html"
}
```

## style[0]

```css

.multicol {
  column-width: 100px;
  inline-size: 300px;
  block-size: 0;
}

.child {
  font: 100px/1 Ahem;
  color: green;
  inline-size: 100px;
  block-size: 100px; /* The define block-size is required to reproduce the bug.*/
  outline: 3px solid green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
