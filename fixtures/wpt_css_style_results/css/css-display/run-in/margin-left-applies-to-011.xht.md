# css/css-display/run-in/margin-left-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/margin-left-applies-to-011.xht"
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
                border-left: 10px solid blue;
            }
            div div
            {
                border-left: 10px solid orange;
                display: run-in;
                margin-left: 50px;
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
