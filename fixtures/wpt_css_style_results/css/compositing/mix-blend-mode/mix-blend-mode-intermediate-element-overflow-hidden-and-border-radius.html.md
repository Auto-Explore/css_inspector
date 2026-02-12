# css/compositing/mix-blend-mode/mix-blend-mode-intermediate-element-overflow-hidden-and-border-radius.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-intermediate-element-overflow-hidden-and-border-radius.html"
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
                background: fuchsia;
                width: 150px;
                height: 75px;
                margin-top: -75px;
                mix-blend-mode: difference;
            }
            .siblingOfBlended {
                background: yellow;/*rgb(255,255,0);*/
                width: 150px;
                height: 150px;
                overflow: hidden;
                border-radius: 2em 2em;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
