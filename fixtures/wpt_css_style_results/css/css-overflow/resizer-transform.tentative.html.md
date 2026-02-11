# css/css-overflow/resizer-transform.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/resizer-transform.tentative.html"
}
```

## style[0]

```css

#resizeme {
  position: absolute;
  top: 200px;
  left: 200px;
  width: 100px;
  height: 100px;
  overflow: hidden;
  resize: both;
  background-color: green;
  transform-origin: 0 0;
  transform: rotate(90deg);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
