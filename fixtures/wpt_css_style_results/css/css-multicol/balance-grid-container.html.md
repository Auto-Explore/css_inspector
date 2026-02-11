# css/css-multicol/balance-grid-container.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/balance-grid-container.html"
}
```

## style[0]

```css

  :root {
    font: 16px/1.25 sans-serif;
  }
  .two-columns {
    column-count: 2;
    width: 550px;
  }
  .two-column-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-auto-rows: auto;
  }
  .grid-col-1 {
    grid-column: 1;
  }
  .grid-col-2 {
    grid-column: 2;
  }
  .keep-together {
    display: inline-block;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
