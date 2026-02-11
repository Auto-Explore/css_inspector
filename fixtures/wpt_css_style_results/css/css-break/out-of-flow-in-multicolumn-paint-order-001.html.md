# css/css-break/out-of-flow-in-multicolumn-paint-order-001.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-paint-order-001.html"
}
```

## style[0]

```css

.multicol {
  columns: 2;
  column-gap: 0;
  column-fill: auto;
  width: 100px;
  height: 100px;
}

.abs-cb {
  position: relative;
  width: 50px;
  height: 200px;
}

.abs {
  position: absolute;
  width: 50px;
  height: 200px;
  left: 0;
  top: 0;
}

.outer {
  background: green;
}

.inner {
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
