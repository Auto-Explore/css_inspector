# css/css-display/run-in/padding-bottom-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/padding-bottom-applies-to-011.xht"
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
                border-bottom: 10px solid blue;
                display: run-in;
                padding-bottom: 50px;
            }
            div div
            {
                border-bottom: 10px solid orange;
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
