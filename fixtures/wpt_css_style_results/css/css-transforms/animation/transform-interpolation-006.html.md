# css/css-transforms/animation/transform-interpolation-006.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-interpolation-006.html"
}
```

## style[0]

```css

.parent {
  transform: translate(30px);
}
.target {
  color: white;
  width: 100px;
  height: 100px;
  background-color: black;
  display: inline-block;
  overflow: hidden;
  transform: translate(10px);
}
.expected {
  background-color: green;
}
.parent {
  transform: 30px;
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
