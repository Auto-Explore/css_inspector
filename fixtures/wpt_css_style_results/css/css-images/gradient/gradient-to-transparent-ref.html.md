# css/css-images/gradient/gradient-to-transparent-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-to-transparent-ref.html"
}
```

## style[0]

```css

.test {
  display: inline-block;
  width: 25px;
  height: 100px;
  margin: 10px;
}
.b {
  background: linear-gradient(rgba(255 0 0 / 0), 75%, red);
}
.e {
  background: linear-gradient(blue, 75%, rgba(0 0 255 / 0));
}
```

```json
{
  "errors": 2,
  "messages": [
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
