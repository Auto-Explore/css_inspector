# css/css-grid/subgrid/crashtests/contain-strict-subgrid.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/crashtests/contain-strict-subgrid.html"
}
```

## style[0]

```css


#gridcontainer {
  display: grid;
  contain: strict;
}

.item {
  display: grid;
  grid: subgrid [x] / subgrid [x];
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
