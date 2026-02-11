# css/compositing/mix-blend-mode/mix-blend-mode-overflowing-child.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-overflowing-child.html"
}
```

## style[0]

```css

            body {
                background: lightgray;
            }
            .container {
                position: absolute;
                z-index: 1;
                width: 100px;
                height: 100px;
                background: #0F0;
            }
            .blender {
                background: #0F0;
                margin: 50px;
                width: 100px;
                height: 100px;
                mix-blend-mode: difference;
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
