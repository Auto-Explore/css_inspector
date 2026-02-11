# css/css-grid/subgrid/grid-gap-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/grid-gap-008-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:24px/1 Ahem; padding:0; margin:0;
}

.grid {
  display:inline-grid;
  grid:100px 375px/300px;
  border:3px solid;
  width: 300px;
  height: 500px;
  background:lightgrey;
  row-gap: 25px;
}
span {
  grid-row:2;
  background:cyan;
  width:50px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
