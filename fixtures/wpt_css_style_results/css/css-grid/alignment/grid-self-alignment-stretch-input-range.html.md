# css/css-grid/alignment/grid-self-alignment-stretch-input-range.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-self-alignment-stretch-input-range.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

input {
  margin:0;
  padding:0;
  border:none;
}

.grid {
  display: inline-grid;
  grid: 80px / 300px;
  place-items: stretch;
  border:1px solid;
}

.zero { grid:0/0; }

.none input { -webkit-appearance:none; }

    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
