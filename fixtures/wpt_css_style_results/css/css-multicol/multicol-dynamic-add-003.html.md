# css/css-multicol/multicol-dynamic-add-003.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-dynamic-add-003.html"
}
```

## style[0]

```css

  .columns {
    columns: 2;
    column-fill: auto;
    width: 100px;
    height: 10px;
  }
  #grid {
    display: grid;
    grid: 20px 20px / 40px 40px;
  }
  #grid::before, #grid::after {
    content: "";
    grid-row: 1 / 3;
  }
  nav {
    grid-row: 1 / 3;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
