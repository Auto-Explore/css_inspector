# css/css-images/color-stop-currentcolor.html

```json
{
  "format_version": 3,
  "file": "css/css-images/color-stop-currentcolor.html"
}
```

## style[0]

```css

     body {
         background: linear-gradient(to right, currentcolor, limegreen);
     }

     div {
         width: 100vw;
         height: 100vh;
         color: limegreen;
         background: inherit;
     }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
