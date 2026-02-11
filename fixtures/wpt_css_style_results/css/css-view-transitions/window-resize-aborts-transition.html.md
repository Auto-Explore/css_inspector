# css/css-view-transitions/window-resize-aborts-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/window-resize-aborts-transition.html"
}
```

## style[0]

```css

      html {background-color: red;}
      html.new {background-color: limegreen;}

      /* Set a no-op animation to show the old snapshot indefinitely. */
      html::view-transition-group(*) {animation-duration: 10s;}
      html::view-transition-new(*) {animation: unset;opacity: 0;}
      html::view-transition-old(*) {animation-duration: 10s;opacity: 1;}
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
