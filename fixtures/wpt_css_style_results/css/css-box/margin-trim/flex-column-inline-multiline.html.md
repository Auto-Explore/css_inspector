# css/css-box/margin-trim/flex-column-inline-multiline.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-column-inline-multiline.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    height: 100px;
    flex-direction: column;
    flex-wrap: wrap;
    margin-trim: inline;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-inline-start: 10px;
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
