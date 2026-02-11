# css/css-backgrounds/background-rounded-image-clip-002.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-rounded-image-clip-002.html"
}
```

## style[0]

```css

.wrapper {
  width: 200px;
  height: 200px;
  background: red;
}
.wrapper > * {
  position: absolute;
  width: 100px;
  height: 100px;
  border: 50px solid green;
}
.round-border {
  border-radius: 100px 150px 200px 250px / 350px;
}
.background {
  border-color: transparent;
  background: linear-gradient(green, green) padding-box;
  transform: scale(1.05);
}
```

```json
{
  "errors": 2,
  "messages": [
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
