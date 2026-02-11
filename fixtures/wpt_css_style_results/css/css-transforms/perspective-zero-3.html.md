# css/css-transforms/perspective-zero-3.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-zero-3.html"
}
```

## style[0]

```css

#outer {
  width: 100px;
  height: 100px;
  background: red;
  perspective: 0;
  perspective-origin: top left;
}
#inner {
  width: 50px;
  height: 50px;
  background: green;
  /* perspective: 0 should be treated as perspective(1px), which should
   * cause this box to be much larger. */
  transform: translateZ(0.5px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “perspective-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
