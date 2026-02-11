# css/CSS2/floats-clear/clear-on-replaced-element.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/clear-on-replaced-element.html"
}
```

## style[0]

```css

#wrapper {
  width: 90px;
  border: 5px solid green;
  background-image: linear-gradient(
    to bottom,
    red 30px, green 30px,
    green 50px, red 50px,
    red 60px, green 60px,
    green 70px, red 70px,
    red 80px, green 80px
  )
}
#wrapper > * {
  display: block;
  width: 100%;
  background: green content-box;
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
