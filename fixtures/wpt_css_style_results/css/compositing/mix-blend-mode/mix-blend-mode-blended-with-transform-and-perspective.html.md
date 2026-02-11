# css/compositing/mix-blend-mode/mix-blend-mode-blended-with-transform-and-perspective.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-blended-with-transform-and-perspective.html"
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
                transform: perspective(600px) translateZ(-200px);
                background-color: red;/*rgb(255,0,0);*/
                mix-blend-mode: difference;
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
