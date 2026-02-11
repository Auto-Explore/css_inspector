# css/css-display/run-in/margin-top-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/margin-top-applies-to-011.xht"
}
```

## style[0]

```css

            div
            {
                height: 200px;
                width: 200px;
            }
            #div1
            {
                border-top: 10px solid blue;
            }
            div div
            {
                border-top: 10px solid orange;
                display: run-in;
                margin-top: 50px;
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
