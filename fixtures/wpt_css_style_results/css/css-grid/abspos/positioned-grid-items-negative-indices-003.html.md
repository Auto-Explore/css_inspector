# css/css-grid/abspos/positioned-grid-items-negative-indices-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-negative-indices-003.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    margin: 4px;
    padding: 10px;
    width: 500px;
    height: 130px;
    position: relative;
  }

  #absolute {
    position: absolute;
    width: 100%;
    height: 100%;
    grid-column: -3 / span 6;
    grid-row: 1 / 2;
    background-color: lightblue;
  }

  #item {
    grid-column: -5 / span 8;
    background-color: hotpink;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
