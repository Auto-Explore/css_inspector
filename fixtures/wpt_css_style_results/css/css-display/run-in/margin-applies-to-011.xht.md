# css/css-display/run-in/margin-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/margin-applies-to-011.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border: 10px solid blue;
                position: absolute;
            }
            #test
            {
                border: 10px solid orange;
                display: run-in;
                height: 200px;
                width: 200px;
                margin: 50px;
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
