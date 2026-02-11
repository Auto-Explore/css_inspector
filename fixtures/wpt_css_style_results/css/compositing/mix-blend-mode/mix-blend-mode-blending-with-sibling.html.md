# css/compositing/mix-blend-mode/mix-blend-mode-blending-with-sibling.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-blending-with-sibling.html"
}
```

## style[0]

```css

            div {
                width: 100px;
                height: 100px;
            }
            .container {
                position: fixed;
                z-index: 0;
                background: #FFF;
            }
            .blender {
                background: #FF0;
                mix-blend-mode: difference;
            }
            .sibling {
                margin-top: -100px;
                background: #F00;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
