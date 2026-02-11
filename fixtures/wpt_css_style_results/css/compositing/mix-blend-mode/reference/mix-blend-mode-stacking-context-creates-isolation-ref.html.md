# css/compositing/mix-blend-mode/reference/mix-blend-mode-stacking-context-creates-isolation-ref.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/reference/mix-blend-mode-stacking-context-creates-isolation-ref.html"
}
```

## style[0]

```css

            .container{
                margin: 30px;
                width: 130px;
                height: 130px;
                float: left;
                background: yellow;/*rgb(255,255,0);*/
            }
            .simple{
                background: lime;/* rgb(0,255,0);*/
                width: 100px;
                height: 100px;
                position: fixed;
            }
            .mixed {
                background: red;/*rgb(255,0,0);*/
                margin-top: 20px;
                margin-left: 20px;
                width: 100px;
                height: 100px;
            }
            .overlap {
                background: black;/*rgb(0,0,0);*/
                width: 80px;
                height: 80px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
