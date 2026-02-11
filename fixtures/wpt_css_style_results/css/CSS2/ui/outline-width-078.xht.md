# css/CSS2/ui/outline-width-078.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/ui/outline-width-078.xht"
}
```

## style[0]

```css

            div
            {
                font: 20px/1 Ahem;
                height: 1em;
                width: 300px;
            }
            #test
            {
                margin: 1em 0 0 1em;
                outline: solid green;
                outline-width: 1em;
                outline-width: -1ex;
            }
            #reference
            {
                border: solid green 1em;
                margin-top: 25px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
