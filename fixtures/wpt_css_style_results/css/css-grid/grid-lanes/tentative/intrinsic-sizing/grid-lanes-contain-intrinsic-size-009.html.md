# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-009.html"
}
```

## style[0]

```css

.cis-both {
  contain-intrinsic-size: 55px 66px;
}

.cis-width {
  contain-intrinsic-size: 55px none;
}

.cis-height {
  contain-intrinsic-size: none 66px;
}

select {
  padding: 0;
}

.test {
  display: grid-lanes;
  contain: size;
  background: lightblue;
  border: 1px solid blue;
  margin: 5px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
