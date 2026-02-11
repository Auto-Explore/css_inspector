# css/css-grid/subgrid/grid-gap-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/grid-gap-007.html"
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

.hl { writing-mode: horizontal-tb; direction:ltr; }
.vrl { writing-mode: vertical-rl; direction:ltr; }
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
