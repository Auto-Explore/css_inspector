# css/css-view-transitions/transformed-element-scroll-transform.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/transformed-element-scroll-transform.html"
}
```

## style[0]

```css

        :root {
            view-transition-name: none;
        }

        ::view-transition-group(*) {
            animation-duration: 10s;
        }

        #target {
            width: 100px;
            height: 100px;
            background: green;
            margin: 300px;
            view-transition-name: target;
            rotate: 90deg;
        }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
