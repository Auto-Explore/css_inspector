# css/compositing/mix-blend-mode/mix-blend-mode-both-parent-and-blended-with-3D-transform.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-both-parent-and-blended-with-3D-transform.html"
}
```

## style[0]

```css

            .parent {
                background: aqua;/*rgb(0,255,255);*/
                width: 140px;
                height: 140px;
                padding-top: 1px;
                transform:rotateX(20deg);
            }
            .blended {
                background: fuchsia;/*rgb(255,0,255);*/
                margin-top: 20px;
                margin-left: 20px;
                width: 200px;
                height: 200px;
                transform:rotateX(20deg);
                mix-blend-mode: difference;
            }
            p {
                will-change: transform;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
