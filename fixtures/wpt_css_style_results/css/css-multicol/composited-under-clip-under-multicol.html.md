# css/css-multicol/composited-under-clip-under-multicol.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/composited-under-clip-under-multicol.html"
}
```

## style[0]

```css

.columns {
  columns: 2;
  column-gap: 20px;
  width: 220px;
  height: 100px;
}
.clip {
  height: 100px;
  overflow: hidden;
}
.composited {
  will-change: transform;
  margin-top: -20px;
  margin-left: -20px;
  border: 20px solid red;
  width: 200px;
  height: 200px;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
