# css/css-box/margin-trim/grid-block-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/grid-block-end.html"
}
```

## style[0]

```css

grid {
    display: inline-grid;
    border: 1px solid black;
    grid-template-columns: auto auto;
    margin-trim: block-end;
}

item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-block-end: 10px;
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
