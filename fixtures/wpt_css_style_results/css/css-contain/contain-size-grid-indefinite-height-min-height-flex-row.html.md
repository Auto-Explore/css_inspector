# css/css-contain/contain-size-grid-indefinite-height-min-height-flex-row.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-grid-indefinite-height-min-height-flex-row.html"
}
```

## style[0]

```css

grid {
    display: grid;
    min-height: 100px;
    grid-template-rows: 1fr;
    contain: size;
}
.absolute {
    position: absolute;
}
.align-last-baseline{
    align-self: last baseline;
}
.item {
    width: 100px;
    height: 50px;
    background-color: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
