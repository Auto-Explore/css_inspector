# css/css-view-transitions/support/dialog-in-rtl-iframe-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/support/dialog-in-rtl-iframe-child.html"
}
```

## style[0]

```css

      body {
        /* We need a background due to https://crbug.com/1414158. */
        background-color: white;
        height: 200vh;
      }

      ::view-transition-new(*) {
        animation: unset;
        opacity: 1;
      }
      ::view-transition-old(*) {
        animation-duration: 30s;
        opacity: 0;
      }

      dialog {
        width: 50dvw;
        height: 50dvh;
        box-sizing: border-box;
        background-color: limegreen;
        border: 1px solid black;
        outline: none;
      }

    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
