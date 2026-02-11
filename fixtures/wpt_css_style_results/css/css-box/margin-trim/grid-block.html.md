# css/css-box/margin-trim/grid-block.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/grid-block.html"
}
```

## style[0]

```css

grid {
    display: inline-grid;
    grid-template-columns: auto auto;
    margin-trim: block;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
.block-start-margin {
    margin-block-start: 10px;
}
.block-end-margin {
    margin-block-end: 20px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “margin-trim”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
