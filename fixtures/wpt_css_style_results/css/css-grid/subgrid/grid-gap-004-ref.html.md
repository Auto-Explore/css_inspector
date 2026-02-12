# css/css-grid/subgrid/grid-gap-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/grid-gap-004-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:24px/1 Ahem; padding:0; margin:0;
}

.grid {
  display:inline-grid;
  grid:100px auto/100px auto;
  gap:20px;
  border:3px solid;
  background:yellow;
}
span { background:cyan; }
span:nth-child(2n+1) { background:grey; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
