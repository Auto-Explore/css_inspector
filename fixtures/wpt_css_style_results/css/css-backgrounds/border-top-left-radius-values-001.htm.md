# css/css-backgrounds/border-top-left-radius-values-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-top-left-radius-values-001.htm"
}
```

## style[0]

```css

            body
            {
                margin: 10px;
            }
            #test
            {
                height: 288px;
                width: 384px;
                border: 2px solid;
                border-top-left-radius: 50px 80px;
            }
            #reference
            {
                position: absolute;
                border: 2px solid blue;
                border-left: none;
                border-top: none;
                left: 10px;
                top: 10px;
                width: 50px;
                height: 80px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
