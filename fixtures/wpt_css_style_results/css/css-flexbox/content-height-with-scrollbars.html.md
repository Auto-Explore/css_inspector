# css/css-flexbox/content-height-with-scrollbars.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/content-height-with-scrollbars.html"
}
```

## style[0]

```css

.flexbox {
    height: 100px;
    overflow: scroll;
}
.flexbox > div {
    flex: none;
}
.flexbox > :nth-child(1) {
    background-color: lightgreen;
}
.flexbox > :nth-child(2) {
    background-color: lightblue;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
