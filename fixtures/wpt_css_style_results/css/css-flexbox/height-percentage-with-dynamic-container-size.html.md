# css/css-flexbox/height-percentage-with-dynamic-container-size.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/height-percentage-with-dynamic-container-size.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  background: cyan;
  height: 200px;
}
.changed .container {
  height: auto;
}
.flex {
  display: flex;
}
.content {
  height: 100px;
  width: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
