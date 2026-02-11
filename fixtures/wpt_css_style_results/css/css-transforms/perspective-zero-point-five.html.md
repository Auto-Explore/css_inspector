# css/css-transforms/perspective-zero-point-five.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-zero-point-five.html"
}
```

## style[0]

```css

#cover-me {
  width: 100px;
  height: 100px;
  background: red;
  position: relative;
  margin-bottom: -100px;
}
#test {
  background: green;
  transform-origin: top left;
  width: 50px;
  height: 50px;
  /* perspective(0.5px) should be treated as perspective(1px), which should
   * cause this box to be much larger. */
  transform: perspective(0.5px) translateZ(0.5px);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
