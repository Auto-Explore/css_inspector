# css/css-box/margin-trim/computed-margin-values/flexbox-column-inline-start.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-column-inline-start.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    width: min-content;
    margin-trim: inline-start;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(1) {
    margin-inline-start: 10px;
}
item:nth-child(2) {
    margin-inline-start: -10px;
}
item:nth-child(3) {
    margin-inline-start: 30%;
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
