# css/compositing/mix-blend-mode/reference/mix-blend-mode-sibling-with-3D-transform-ref.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/reference/mix-blend-mode-sibling-with-3D-transform-ref.html"
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
            }
            .siblingOfBlended {
                background: blue;/*rgb(0,0,255);*/
                margin-top: 20px;
                margin-left: 20px;
                width: 200px;
                height: 200px;
                transform: perspective(600px) translateZ(-200px);
            }
            .intersectionOfBlended {
                background: yellow;/*rgb(255,255,0);*/
                margin-top: -200px;
                margin-left: 20px;
                width: 120px;
                height: 120px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
