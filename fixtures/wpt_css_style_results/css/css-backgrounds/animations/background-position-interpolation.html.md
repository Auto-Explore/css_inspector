# css/css-backgrounds/animations/background-position-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-position-interpolation.html"
}
```

## style[0]

```css

.parent {
  background-position: 60px 60px;
}
.target {
  width: 120px;
  height: 120px;
  display: inline-block;
  border: 2px solid black;
  background-repeat: no-repeat;
  background-image: radial-gradient(20px circle at 20px 20px, red 18px, transparent),
                    radial-gradient(20px circle at 20px 20px, yellow 18px, transparent),
                    radial-gradient(20px circle at 20px 20px, lime 18px, transparent),
                    radial-gradient(20px circle at 20px 20px, blue 18px, transparent);
  background-position: 40px 40px;
}
.expected {
  margin-right: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
