# css/compositing/mix-blend-mode/mix-blend-mode-blended-element-overflow-scroll.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-blended-element-overflow-scroll.html"
}
```

## style[0]

```css

            .parent {
                background: yellow;/*rgb(0,255,255);*/
                width: 100px;
                height: 100px;
                position: relative;
                z-index: 1;
                overflow: hidden;
            }
            .blended {
                background: red;/*rgb(255,0,0);*/
                width: 150px;
                height: 150px;
                overflow:scroll;
                mix-blend-mode: difference;
            }
            .scrollingContent {
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
