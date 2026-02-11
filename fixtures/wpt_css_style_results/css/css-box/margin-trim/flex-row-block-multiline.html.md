# css/css-box/margin-trim/flex-row-block-multiline.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-row-block-multiline.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: 100px;
    flex-wrap: wrap;
    margin-trim: block;
    border: 1px solid black;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-block: 10px;
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
