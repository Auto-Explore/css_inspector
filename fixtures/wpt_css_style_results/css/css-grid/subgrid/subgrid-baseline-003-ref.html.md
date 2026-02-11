# css/css-grid/subgrid/subgrid-baseline-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-003-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 Ahem; padding:0; margin:0;
}

.grid {
  display: grid;
  grid-template-columns: auto auto;
  max-width: 100px;
  place-items: baseline start;
}

.first {
  font-size: 3em;
}

.second {
  font-size: 2em;
}
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “place-items”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
