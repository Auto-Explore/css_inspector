# css/css-grid/subgrid/repeat-auto-fill-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/repeat-auto-fill-005.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:10px/1 monospace; padding:0; margin:0;
}

.grid {
  display: inline-grid;
  grid-auto-columns: 20px;
  background: lightgrey;
}

.subgrid {
  display: grid;
  grid-column: 2 / span 4;
  background: black;
}

item {
  min-width: 0;
  min-height: 10px;
  grid-column:x -2 / x -1;
  background: grey;
}

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
