# css/css-box/margin-trim/grid-inline.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/grid-inline.html"
}
```

## style[0]

```css

grid {
    display: inline-grid;
    grid-template-columns: auto auto;
    margin-trim: inline;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(odd) {
    margin-inline-start: 10px;
}
item:nth-child(even) {
    margin-inline-end: 20px;
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
