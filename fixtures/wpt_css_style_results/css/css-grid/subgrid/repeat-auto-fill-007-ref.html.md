# css/css-grid/subgrid/repeat-auto-fill-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/repeat-auto-fill-007-ref.html"
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
  grid: subgrid / auto;
  grid-column: 2 / span 4;
  background: black;
  writing-mode: vertical-rl;
}

item {
  min-width: 0;
  min-height: 10px;
  background: grey;
  writing-mode: horizontal-tb;
}

  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
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
