# css/css-grid/subgrid/subgrid-baseline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-001.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 Ahem; padding:0; margin:0;
}

.grid {
  display: grid;
  grid: 20px repeat(4, auto) 30px / 30px repeat(4, auto) 20px;
  place-content: start;
  place-items: baseline start;
  border: 1px solid;
  text-decoration: underline blue;
}

.subgrid {
  display: grid;
  grid: subgrid / auto;
  background: lightgrey;
  grid-column: 2 / span 4;
  grid-row: 3 / span 2;
  min-width: 10px;
  min-height: 0;
  background: yellow;
  place-content: inherit;
  place-items: inherit;
  padding-top: 20px;
}

x {
  min-width: 20px;
  min-height: 10px;
  font-size: 8em;
  background: silver;
}

  
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “place-items”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
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
    }
  ],
  "warnings": 0
}
```
