# css/css-flexbox/flex-aspect-ratio-img-column-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-aspect-ratio-img-column-002.html"
}
```

## style[0]

```css

            #reference-overlapped-red {
                position: absolute;
                background-color: red;
                width: 100px;
                height: 100px;
                z-index: -1;
            }

            #constrained-flex {
                display: flex;
                flex-direction: column;
                height: 10px;
            }

            img {
                min-width: 0;
                min-height: 0;
                flex: none;
                height: 100px;
                align-self: flex-start;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
