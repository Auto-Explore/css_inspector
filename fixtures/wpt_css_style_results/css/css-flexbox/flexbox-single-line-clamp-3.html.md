# css/css-flexbox/flexbox-single-line-clamp-3.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-single-line-clamp-3.html"
}
```

## style[0]

```css

.container {
  display: flex;
  background: gray;
  min-height: 80px;

  /* Don't let (default) align-content:stretch save us
     by stretching the line to fit the container! The point
     here is that the line should already be clamped to the
     container's min-height. */
  align-content: flex-start;
}
.panel {
  background: lightblue;
  width: 150px;
}
.contents {
  height: 10px;
  width: 10px;
  background: purple;
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
