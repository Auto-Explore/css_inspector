# css/css-grid/subgrid/repeat-auto-fill-006.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/repeat-auto-fill-006.html"
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
  writing-mode: vertical-lr;
}

item {
  min-width: 0;
  min-height: 10px;
  grid-row:x -2 / x -1;
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
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
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
