# css/css-images/linear-gradient-non-square.html

```json
{
  "format_version": 3,
  "file": "css/css-images/linear-gradient-non-square.html"
}
```

## style[0]

```css

.a,
.b,
.c,
.d {
  width: 200px;
  height: 100px;
}
.a {
  background: linear-gradient(to right bottom, black 50%, lightgray 50%);
}
.b {
  background: linear-gradient(to left bottom, black 50%, lightgray 50%);
}
.c {
  background: linear-gradient(to left top, black 50%, lightgray 50%);
}
.d {
  background: linear-gradient(to right top, black 50%, lightgray 50%);
}
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
