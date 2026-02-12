# css/css-grid/subgrid/subgrid-baseline-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-004.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 Ahem; padding:0; margin:0;
}

.grid {
  display: grid;
  grid-template-columns: 50px 50px;
  grid-template-rows: auto auto;
  place-items: baseline start;
}

.subgrid {
  display: grid;
  grid-template-columns: subgrid;
  grid-row: 1 / 3;
  grid-column: 2;

  padding-bottom: 20px;
  writing-mode: vertical-lr;
  place-items: start baseline;
}

.first {
  font-size: 3em;
  grid-row: 2;
}

.second {
  font-size: 2em;
  grid-column: 2;
  writing-mode: horizontal-tb;
}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
