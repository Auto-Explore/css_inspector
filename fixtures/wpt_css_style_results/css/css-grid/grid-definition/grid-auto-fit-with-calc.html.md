# css/css-grid/grid-definition/grid-auto-fit-with-calc.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-fit-with-calc.html"
}
```

## style[0]

```css

  .grid {
    display: grid;
    width: 100px;
    grid-template-columns: repeat(auto-fit, minmax(calc(100% - 10px), calc(100% - 100px)));
    background-color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
