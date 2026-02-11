# css/css-display/run-in/padding-top-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/padding-top-applies-to-011.xht"
}
```

## style[0]

```css

            div
            {
                width: 200px;
            }
            #div1
            {
                border-top: 10px solid blue;
                display: run-in;
                padding-top: 50px;
            }
            div div
            {
                border-top: 10px solid orange;
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
