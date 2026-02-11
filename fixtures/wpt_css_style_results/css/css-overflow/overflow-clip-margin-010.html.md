# css/css-overflow/overflow-clip-margin-010.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-010.html"
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

  overflow: clip;
  overflow-clip-margin: 20px;
  border-radius: 0px 15px 25px 35px;
}
.child {
  width: 400px;
  height: 400px;
  margin: -200px;
  background: lightblue;
  opacity: 0.8;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
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
  "warnings": 1
}
```
