# css/css-box/margin-trim/grid-trim-ignores-collapsed-tracks.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/grid-trim-ignores-collapsed-tracks.html"
}
```

## style[0]

```css

grid {
    display: inline-grid;
    grid-template-rows: repeat(auto-fit, 250px);
    grid-template-columns: repeat(auto-fit, 250px);
    margin-trim: block-start inline-start;
}
item {
    display: block;
    background-color: green;
    grid-column: 2;
    grid-row: 2;
    margin-inline-start: 30px;
    margin-block-start: 50px;
    width: 100px;
    height: 100px;
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
