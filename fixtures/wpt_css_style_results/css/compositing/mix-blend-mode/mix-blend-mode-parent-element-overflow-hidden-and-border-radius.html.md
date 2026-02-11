# css/compositing/mix-blend-mode/mix-blend-mode-parent-element-overflow-hidden-and-border-radius.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-parent-element-overflow-hidden-and-border-radius.html"
}
```

## style[0]

```css

            .parent {
                background: red;
                width: 140px;
                height: 140px;
                position: relative;
                z-index: 1;
                overflow: hidden;
                border-radius: 1em 5em;
            }
            .blended {
                background: yellow;
                width: 200px;
                height: 200px;
                mix-blend-mode: difference;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
