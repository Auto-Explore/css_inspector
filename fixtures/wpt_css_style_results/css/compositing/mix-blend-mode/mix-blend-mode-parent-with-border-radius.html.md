# css/compositing/mix-blend-mode/mix-blend-mode-parent-with-border-radius.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-parent-with-border-radius.html"
}
```

## style[0]

```css

            body {
                background: lightgray;
            }
            .parent {
                position: absolute;
                z-index: 1;
                width: 100px;
                height: 100px;
                background: #F00;
                border-radius: 50px;
            }
            .blended {
                background: #FF0;
                width: 100px;
                height: 100px;
                mix-blend-mode: difference;
            }

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
