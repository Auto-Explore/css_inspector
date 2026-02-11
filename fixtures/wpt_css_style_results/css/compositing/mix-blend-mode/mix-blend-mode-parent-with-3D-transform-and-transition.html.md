# css/compositing/mix-blend-mode/mix-blend-mode-parent-with-3D-transform-and-transition.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-parent-with-3D-transform-and-transition.html"
}
```

## style[0]

```css

            .parent {
                background: yellow;
                width: 140px;
                height: 140px;
                position:relative;
                z-index: 1;
                margin: 10px;
                float: left;
                transition: transform 2s;

            }
            .rotated {
                transform: rotateX(60deg) rotateZ(10deg) rotateY(180deg);
            }
            .blended {
                background: red;
                width: 140px;
                height: 140px;
                mix-blend-mode: difference;
            }
            .ref {
                background: none;
            }
            .ref .blended {
                background: lime;
                mix-blend-mode: normal;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
