# css/css-grid/subgrid/line-names-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-003.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:24px/1 monospace; padding:0; margin:0;
}

.grid {
  display: grid;
  grid: auto / [a] 50px 50px [a] 50px 50px [a];
  padding: 20px 10px;
}

.subgrid {
  display: grid;
  grid: 50px / subgrid;
  grid-column: span 4;
}

x { background: grey; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
