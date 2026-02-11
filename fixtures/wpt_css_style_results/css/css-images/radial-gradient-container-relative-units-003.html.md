# css/css-images/radial-gradient-container-relative-units-003.html

```json
{
  "format_version": 3,
  "file": "css/css-images/radial-gradient-container-relative-units-003.html"
}
```

## style[0]

```css

  #container { container-type: size; width: 100px; height: 100px; }
  #inner {
    width: 100%;
    height: 100%;
    background-image: radial-gradient(50cqi 50px ellipse at 50px 50px, green, blue);
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
