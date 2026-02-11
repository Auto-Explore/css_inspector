# css/css-flexbox/box-sizing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/box-sizing-001.html"
}
```

## style[0]

```css

.flexbox {
    border: 2px solid orange;
}
.h3 {
    height: 300px;
}
.w3 {
    width: 300px;
}
.h4 {
    height: 400px;
}
.w4 {
    width: 400px;
}
.border-box, .flexbox > div {
    box-sizing: border-box;
    height: 100px;
    width: 100px;
    border: 2px solid lightblue;
    border-top-width: 4px;
    padding: 3px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
