# css/css-box/margin-trim/flex-row-shrink.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-row-shrink.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: 100px;
    margin-trim: inline;
}
item {
    display: block;
    background-color: green;
    margin-inline: 25px;
    flex: 0 1 auto;
    width: 150px;
    height: 100px;
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
