# css/css-images/gradient/gradient-longer-hue-hsl-013.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-longer-hue-hsl-013.html"
}
```

## style[0]

```css

div {
  margin: 20px 0px 20px 50px;
  height: 40px;
  width: 100px;
  position: relative;
}
/* the colors here will have to be converted to the interpolation color space */
.test1 {
  background: linear-gradient(to right in hsl longer hue, red, black);
}
.test2 {
  background: linear-gradient(to right in hsl longer hue, red, white);
}
.test3 {
  background: linear-gradient(to right in hsl longer hue, red, transparent);
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
