# css/compositing/mix-blend-mode/mix-blend-mode-blended-element-with-transparent-pixels.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-blended-element-with-transparent-pixels.html"
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
                width: 120px;
                height: 120px;
                background: transparent;
                mix-blend-mode: difference;
                margin: -10px;
            }
            .blendedChild {
                width: 120px;
                height: 120px;
                background: #0FF;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
