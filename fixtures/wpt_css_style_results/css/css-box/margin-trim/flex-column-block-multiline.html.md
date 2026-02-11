# css/css-box/margin-trim/flex-column-block-multiline.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-column-block-multiline.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    height: 100px;
    flex-wrap: wrap;
    flex-direction: column;
    margin-trim: block;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(odd) {
    margin-block-start: 25px;
}
item:nth-child(even) {
    margin-block-end: 25px;
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
