# css/css-flexbox/columns-height-set-via-top-bottom.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/columns-height-set-via-top-bottom.html"
}
```

## style[0]

```css

.container {
    position: relative;
    height: 100px;
    width: 100px;
    border: 2px solid orange;
}
.flexbox {
    flex-direction: column;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    padding: 10px;
}
.flexbox > :nth-child(1) {
    background-color: lightblue;
}
.flexbox > :nth-child(2) {
    background-color: lightgreen;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
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
