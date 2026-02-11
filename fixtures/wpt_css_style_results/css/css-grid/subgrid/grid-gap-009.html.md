# css/css-grid/subgrid/grid-gap-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/grid-gap-009.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:24px/1 Ahem; padding:0; margin:0;
}

.grid {
  display:inline-grid;
  grid:100px auto/200px auto;
  border:3px solid;
  background:grey;
  width:300px;
  height:400px;
}
.subgrid {
  display: grid;
  grid: subgrid / 50px;
  row-gap: 10%;
  grid-area: span 2/span 2;
  background:lightgrey;
}
span { background:cyan; }

.hl { writing-mode: horizontal-tb; direction:ltr; }
.vlr { writing-mode: vertical-lr; direction:rtl; }
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-area”.",
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
