# css/css-view-transitions/fractional-box-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fractional-box-ref.html"
}
```

## style[0]

```css

.box {
  margin: 15px;
  background: green;
  width: 100.125px;
  height: 50.875px;
}
.shift {
  position: relative;
  left: 0.4px;
}
body { background: lightpink }
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
