# css/compositing/mix-blend-mode/reference/mix-blend-mode-parent-element-overflow-scroll-blended-position-fixed-ref.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/reference/mix-blend-mode-parent-element-overflow-scroll-blended-position-fixed-ref.html"
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
                overflow:scroll;
            }

            .blended {
                background: blue;/*rgb(0,0,255);*/
                width: 100px;
                height: 100px;
                position: fixed;
            }

            .scrollableContent {
                width: 200px;
                height: 200px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
