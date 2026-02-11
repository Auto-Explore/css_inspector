# css/css-pseudo/outside-marker-paint-order.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/outside-marker-paint-order.html"
}
```

## style[0]

```css

li {
  margin-left: 100px;
  text-indent: -100px;
  font: 20px/1 Ahem;
  color: red;
}

x { color: grey; }
.before::before { content: "XXXXXXXX"; color: grey; }
.after::after { content: "XXXXXXXX"; color: grey; }
.both::before,.both::after  {content: "XXXX"; color: grey; }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
