# css/CSS2/linebox/inline-formatting-context-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/inline-formatting-context-011.xht"
}
```

## style[0]

```css

            div
            {
                border-top: solid orange;
            }
            span
            {
                border: 20px solid blue;
                display: inline-block;
                font: 50px Ahem;
            }
            #span1
            {
                color: blue;
                line-height: 1em;
            }
            #span2
            {
                line-height: 10px;
                vertical-align: top;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
