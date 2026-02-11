# css/css-ui/animation/outline-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/animation/outline-width-interpolation.html"
}
```

## style[0]

```css

.parent {
  outline: solid transparent;
  outline-width: 30px;
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  margin: 18px;
  outline: solid orange;
  outline-width: 10px;
  opacity: 0.5;
}
.expected {
  background-color: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
