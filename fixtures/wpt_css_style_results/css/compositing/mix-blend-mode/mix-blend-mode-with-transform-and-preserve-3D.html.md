# css/compositing/mix-blend-mode/mix-blend-mode-with-transform-and-preserve-3D.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-with-transform-and-preserve-3D.html"
}
```

## style[0]

```css

            div {
                height: 150px;
                width: 150px;
            }
            .container {
                position: relative;
                z-index: 1;
                background-color: lime;/*rgb(0,255,0);*/
            }
            .transformed {
                transform-style: preserve-3d;
                transform: rotateY(50deg);
                background-color: aqua;/*rgb(0,255,255);*/
                mix-blend-mode: difference;
            }
            .child {
                transform-origin: top left;
                transform: rotateX(40deg);
                background-color: red;/*rgb(255,0,0);*/
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
