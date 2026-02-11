# css/compositing/mix-blend-mode/mix-blend-mode-parent-element-overflow-scroll-blended-position-fixed.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-parent-element-overflow-scroll-blended-position-fixed.html"
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
                background: fuchsia;/*rgb(255,0,255);*/
                width: 100px;
                height: 100px;
                mix-blend-mode: multiply;
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
