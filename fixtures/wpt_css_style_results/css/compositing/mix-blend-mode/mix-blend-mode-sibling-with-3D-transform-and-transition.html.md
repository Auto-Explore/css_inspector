# css/compositing/mix-blend-mode/mix-blend-mode-sibling-with-3D-transform-and-transition.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-sibling-with-3D-transform-and-transition.html"
}
```

## style[0]

```css

            .parent {
                position: relative;
                z-index: 1;
                float: left;
                margin-left: 10px;
            }
            .blended {
                background: blue;
                margin-top: -120px;
                width: 140px;
                position: relative;
                z-index: 1;
                height: 140px;
                mix-blend-mode: difference;
            }
            .siblingOfBlended {
                background: aqua;
                width: 100px;
                height: 100px;
                margin-top: 20px;
                margin-left: 20px;
                transition: transform 2s;
            }
            .rotated {
                transform: rotateX(60deg) rotateY(10deg) rotateZ(90deg);
            }

            .ref .blended {
                mix-blend-mode: normal;
            }
            .ref .siblingOfBlended {
                z-index: 2;
                position: relative;
                background: lime;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
