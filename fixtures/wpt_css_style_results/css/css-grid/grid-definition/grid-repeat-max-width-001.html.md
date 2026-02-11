# css/css-grid/grid-definition/grid-repeat-max-width-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-repeat-max-width-001.html"
}
```

## style[0]

```css

.wrapper {
    width: 190px;
    display: grid;
    justify-content: start;
}
.grid {
    max-width: 200px;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(0, 100px));
    background: red;
}
.item {
    background: green;
    width: 100px;
    height: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
