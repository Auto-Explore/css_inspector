# css/css-box/margin-trim/computed-margin-values/grid-block-end-item-spans-multiple-rows.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/grid-block-end-item-spans-multiple-rows.html"
}
```

## style[0]

```css

grid {
    display: grid;
    border: 1px solid black;
    grid-template-columns: auto auto;
    margin-trim: block-end;
}
item {
    display: block;
    width: 50px;
    height: 50px;
    margin-bottom: 10px;
}
.row-two {
    grid-row: 2;
    background-color: green;
}
.span-two-rows {
    grid-row: span 2;
    background-color: blue;
    height: 90px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “margin-trim”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
