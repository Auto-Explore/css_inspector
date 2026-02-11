# css/compositing/mix-blend-mode/reference/mix-blend-mode-blended-element-overflow-hidden-and-border-radius-ref.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/reference/mix-blend-mode-blended-element-overflow-hidden-and-border-radius-ref.html"
}
```

## style[0]

```css

            .parent {
                background: aqua;/*rgb(0,255,255);*/
                width: 150px;
                height: 150px;
                position: relative;
                z-index: 1;
            }
            .blended {
                width: 150px;
                height: 150px;
                overflow: hidden;
                border-radius: 2em 2em;
            }
            .childOfBlended {
                background: lime;/*rgb(0,255,0);*/
                width: 150px;
                height: 150px;
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
