# css/css-grid/grid-flex-spanning-items-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-flex-spanning-items-001.html"
}
```

## style[0]

```css

#mygrid {
    display: grid;
    grid-template-columns: 1fr 30px;
    border: 10px solid fuchsia;
    width: min-content;
}
#item {
    grid-column: 1 / span 2;
}
#filler {
    width: 300px;
    height: 50px;
    background: aqua;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
