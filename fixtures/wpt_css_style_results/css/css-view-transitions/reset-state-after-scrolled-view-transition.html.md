# css/css-view-transitions/reset-state-after-scrolled-view-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/reset-state-after-scrolled-view-transition.html"
}
```

## style[0]

```css

        html {
            background: lightblue;
        }
        body {
            background-color: lightgreen;
        }
        ::view-transition-group(*) {
            animation-duration: 2s;
        }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
