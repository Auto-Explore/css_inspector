# css/css-transforms/transform-transformed-tfoot-contains-fixed-position.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-transformed-tfoot-contains-fixed-position.html"
}
```

## style[0]

```css

      table, tfoot, tr, td {
        margin: 0;
        padding: 0;
        border-spacing: 0;
      }
      .pad {
        height: 50px;
      }
      .container {
        transform: translateX(20px) rotate(45deg);
        transform-origin: left;
      }
      .fixed {
        position: fixed;
        top: 15px;
        left: 10px;
        background-color: lightblue;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
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
