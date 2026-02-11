# css/css-display/run-in/margin-bottom-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/margin-bottom-applies-to-011.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-bottom: 10px solid blue;
            }
            div div
            {
                border-bottom: 10px solid orange;
                display: run-in;
                height: 200px;
                margin-bottom: 50px;
                width: 200px;
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
