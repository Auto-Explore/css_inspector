# css/css-box/margin-trim/grid-inline-end-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/grid-inline-end-ref.html"
}
```

## style[0]

```css

grid {
    display: inline-grid;
    border: 1px solid black;
    grid-template-columns: auto auto;
    margin-trim: inline-end;
}

item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
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
