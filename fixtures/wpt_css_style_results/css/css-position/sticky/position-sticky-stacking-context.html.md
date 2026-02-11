# css/css-position/sticky/position-sticky-stacking-context.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-stacking-context.html"
}
```

## style[0]

```css

.indicator {
  position: absolute;
  background-color: green;
  z-index: 1;
}

.sticky {
  position: sticky;
  z-index: 0;
}

.child {
  position: relative;
  background-color: red;
  z-index: 2;
}

.box {
  width: 200px;
  height: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
