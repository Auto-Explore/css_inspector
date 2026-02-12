# css/compositing/mix-blend-mode/mix-blend-mode-border-image.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-border-image.html"
}
```

## style[0]

```css

            .parent {
                width: 120px;
                height: 120px;
                background: #FF0;
                position: fixed;
                z-index: 1;
            }

            .child {
                width: 100px;
                height: 100px;
                background: #F00;
                mix-blend-mode: difference;
                border-width: 10px;
                border-image: url('support/red_square.svg') 10 repeat;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
