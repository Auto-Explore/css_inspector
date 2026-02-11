# css/css-grid/subgrid/line-names-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-004-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:24px/1 monospace; padding:0; margin:0;
}

.grid {
  display: grid;
  grid: auto / [a] 50px 50px 50px 50px [b] 50px 50px [a b];
  padding: 20px 10px;
}

.subgrid {
  display: grid;
  grid-template-columns:  subgrid [x] [b] [] [] [b];
  grid-auto-rows: 10px;
  grid-column: 2 / span 4;
}

x { background: grey; }

  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-template-columns”.",
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
