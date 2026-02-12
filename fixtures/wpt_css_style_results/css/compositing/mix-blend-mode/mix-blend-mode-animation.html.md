# css/compositing/mix-blend-mode/mix-blend-mode-animation.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-animation.html"
}
```

## style[0]

```css

            div {
                width: 100px;
                height: 100px;
                background: #FF0;
            }

            @keyframes changeOpacity
            {
                from { opacity: 0.9; }
                50% { opacity: 0.9; }
                to { opacity: 0.1; }
            }

            #blender {
                background: #F00;
                mix-blend-mode: difference;
                animation: changeOpacity 10s;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
