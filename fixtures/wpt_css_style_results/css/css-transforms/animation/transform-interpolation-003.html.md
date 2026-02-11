# css/css-transforms/animation/transform-interpolation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-interpolation-003.html"
}
```

## style[0]

```css

.target {
  color: white;
  width: 100px;
  height: 100px;
  background-color: black;
  display: inline-block;
  overflow: hidden;
}
.expected {
  background-color: green;
}
.target > div {
  width: 10px;
  height: 10px;
  display: inline-block;
  background: orange;
  margin: 1px;
}
.test {
  overflow: hidden;
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
