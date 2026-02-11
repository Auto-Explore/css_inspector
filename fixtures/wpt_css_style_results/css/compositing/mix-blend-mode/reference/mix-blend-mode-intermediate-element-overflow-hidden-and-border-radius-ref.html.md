# css/compositing/mix-blend-mode/reference/mix-blend-mode-intermediate-element-overflow-hidden-and-border-radius-ref.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/reference/mix-blend-mode-intermediate-element-overflow-hidden-and-border-radius-ref.html"
}
```

## style[0]

```css

            .parent {
                background: aqua;
                width: 150px;
                height: 150px;
                position: relative;
                z-index: 1;
            }
            .blended {
                background: yellow;
                width: 150px;
                height: 150px;
                border-radius: 2em 2em;
            }
            .blended1 {
                background: yellow;
                width: 150px;
                height: 75px;
                margin-top: -75px;
            }
            .child1 {
                background: aqua;
                width: 150px;
                height: 75px;
                border-radius: 0 0 2em 2em;
            }
            .siblingOfBlended {
                background: yellow;
                width: 150px;
                height: 150px;
                overflow: hidden;
                border-radius: 2em 2em;
            }
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
