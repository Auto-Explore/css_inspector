# css/css-grid/abspos/positioned-grid-items-020.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-020.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    grid-auto-columns: 9px 14px 22px;
    width: 100px;
    height: 100px;
    position: relative;
    gap: 2px;
    background-color: green;
  }

  .absolute {
    position: absolute;
    width: 100%;
    height: 100%;
    grid-row: 1 / 2;
    background-color: green;
  }

  #item {
    grid-column: 5 / span 2;
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
