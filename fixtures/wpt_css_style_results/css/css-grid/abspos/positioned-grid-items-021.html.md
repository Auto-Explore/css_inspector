# css/css-grid/abspos/positioned-grid-items-021.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-021.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    width: 100px;
    height: 100px;
    position: relative;
  }

  .absolute {
    position: absolute;
    width: 100%;
    height: 100%;
    grid-row: 1 / 2;
    background-color: green;
  }

  #item {
    grid-column: 1 / span 3;
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
