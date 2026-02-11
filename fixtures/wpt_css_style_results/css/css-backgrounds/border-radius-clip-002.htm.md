# css/css-backgrounds/border-radius-clip-002.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-clip-002.htm"
}
```

## style[0]

```css

            #test-base
            {
                width: 98px;
                height: 98px;
                border: 10px double black;
                padding: 11px;
                border-radius: 40px;
                background: red url(support/swatch-red.png);
                background-clip: content-box;
            }
            #reference-cover
            {
                margin: -120px 0 40px 20px;
                width: 100px;
                height: 100px;
                border-radius: 20px;
                background-color: black;
            }

            #reference-base
            {
                width: 98px;
                height: 98px;
                margin-top: 62px;
                margin-left: 20px;
                border-radius: 20px;
                background: red url(support/swatch-red.png);
            }
            #test-cover
            {
                margin-top: -120px;
                border: 10px double black;
                padding: 10px;
                width: 100px;
                height: 100px;
                border-radius: 40px;
                background-color: black;
                background-clip: content-box;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
