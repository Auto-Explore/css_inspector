# css/css-box/margin-trim/flex-column-grow.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-column-grow.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    height: 100px;
    flex-direction: column;
    margin-trim: block;
}
item {
    display: block;
    width: 100px;
    height: 50px;
    margin-block: 25px;
    flex: 1 0 auto;
    background-color: green;
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
