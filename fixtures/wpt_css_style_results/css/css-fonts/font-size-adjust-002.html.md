# css/css-fonts/font-size-adjust-002.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-size-adjust-002.html"
}
```

## style[0]

```css

  div {
    position: absolute;
    font: 40px/40px Ahem;
    color: blue;
  }
  #test {
    color: orange;
    font-size-adjust: 0.2;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
