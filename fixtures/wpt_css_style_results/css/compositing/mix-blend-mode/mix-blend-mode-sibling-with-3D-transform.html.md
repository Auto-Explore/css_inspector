# css/compositing/mix-blend-mode/mix-blend-mode-sibling-with-3D-transform.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-sibling-with-3D-transform.html"
}
```

## style[0]

```css

            .parent {
                background: aqua;/*rgb(0,255,255);*/
                width: 140px;
                height: 140px;
                padding-top: 1px;
                position: relative;
                z-index: 1;
            }
            .blended {
                background: fuchsia;/*rgb(255,0,255);*/
                margin-top: -200px;
                margin-left: 20px;
                width: 200px;
                height: 200px;
                mix-blend-mode: difference;
            }
            .siblingOfBlended {
                background: red;/*rgb(255,0,0);*/
                margin-top: 20px;
                margin-left: 20px;
                width: 200px;
                height: 200px;
                transform: perspective(600px) translateZ(-200px);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
