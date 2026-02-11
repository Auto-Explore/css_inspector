# css/css-box/margin-trim/computed-margin-values/flexbox-row-block-start.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-row-block-start.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    margin-trim: block-start;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-inline: 10px;
}
item:nth-child(1) {
    margin-block-start: 10px;
}
item:nth-child(2) {
    margin-block-start: -10px;
}
item:nth-child(3) {
    margin-block-start: 30%;
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
