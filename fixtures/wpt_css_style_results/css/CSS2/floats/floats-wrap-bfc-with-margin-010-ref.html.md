# css/CSS2/floats/floats-wrap-bfc-with-margin-010-ref.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/floats-wrap-bfc-with-margin-010-ref.html"
}
```

## style[0]

```css

body {
  position: relative;
}
.wrapper {
  position: absolute;
  width: 100px;
  border: 5px solid;
  top: 75px;
}
.float {
  position: absolute;
  width: 50px;
  height: 50px;
  left: 0;
  top: 0;
  background: cyan;
}
.bfc {
  position: absolute;
  height: 50px;
  background: green;
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
