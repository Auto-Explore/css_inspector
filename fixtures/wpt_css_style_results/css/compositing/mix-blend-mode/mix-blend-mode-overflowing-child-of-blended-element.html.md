# css/compositing/mix-blend-mode/mix-blend-mode-overflowing-child-of-blended-element.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-overflowing-child-of-blended-element.html"
}
```

## style[0]

```css

            .parent {
                position: absolute;
                z-index: 1;
                background: #00F;
                width: 100px;
                height: 100px;
                margin: 10px;
            }
            .blender {
                background: #777;
                width: 120px;
                height: 120px;
                margin: -10px;
                mix-blend-mode: difference;
            }
            .child {
                background: #0FF;
                width: 120px;
                height: 120px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
