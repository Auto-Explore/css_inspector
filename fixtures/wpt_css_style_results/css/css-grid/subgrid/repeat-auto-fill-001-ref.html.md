# css/css-grid/subgrid/repeat-auto-fill-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/repeat-auto-fill-001-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:10px/1 monospace; padding:0; margin:0;
}

.grid {
  display: inline-grid;
  grid-auto-columns: 15px;
  border: 1px solid;
}

.subgrid {
  display: grid;
  grid-template-columns: subgrid;
  grid-column: 3 / span 4;
  grid-auto-rows: 8px;
  background: grey;
}

.subgrid > :nth-child(2n)   {  background: black; }
.subgrid > :nth-child(2n+1) {  background: pink; }

  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
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
