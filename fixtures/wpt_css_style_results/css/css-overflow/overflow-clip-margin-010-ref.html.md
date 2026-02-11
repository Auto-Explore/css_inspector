# css/css-overflow/overflow-clip-margin-010-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-010-ref.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;

  position: relative;
  top: 30px;
  left: 30px;

  border: 5px solid green;

  border-radius: 0px 15px 25px 35px;
}
.clipper {
  width: 140px;
  height: 140px;

  margin: -20px;
  border-radius:0px 27.52px 40px 50px;
  overflow: clip;
}
.child {
  width: 400px;
  height: 400px;
  background: lightblue;
  opacity: 0.8;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
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
