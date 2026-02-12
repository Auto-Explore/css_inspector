# css/css-grid/subgrid/placement-invalidation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/placement-invalidation-001.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
  padding: 0;
}
#grid {
  width: 200px;
  display: grid;
  background: lightgray;
  grid-template-columns: [start] auto [end] 1fr;
  grid-template-rows: 100px;
}
#subgrid {
  display: grid;
  grid-column: 1 / -1;
  grid-template-columns: subgrid;
}
#item {
  width: 50px;
  background: lightblue;
  border: 5px solid gray;
  grid-column: start / end;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
