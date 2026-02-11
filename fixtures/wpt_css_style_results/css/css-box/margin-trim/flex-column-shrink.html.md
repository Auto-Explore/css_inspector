# css/css-box/margin-trim/flex-column-shrink.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-column-shrink.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    width: min-content;
    height: 100px;
    margin-trim: block;
}
item {
    display: block;
    background-color: green;
    margin-block: 25px;
    flex: 0 1 auto;
    width: 100px;
    height: 150px;
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
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
