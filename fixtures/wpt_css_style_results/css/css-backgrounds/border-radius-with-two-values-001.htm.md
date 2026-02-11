# css/css-backgrounds/border-radius-with-two-values-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-with-two-values-001.htm"
}
```

## style[0]

```css

            div
            {
                width: 2in;
                height: 2in;
                border: 0.2in solid red;
            }
            #reference
            {
                border-top-left-radius : 10px;
                border-top-right-radius : 50px;
                border-bottom-right-radius : 10px;
                border-bottom-left-radius : 50px;
            }
            #test
            {
                border-radius: 10px 50px;
                border-color: green;
                margin-top: -2.4in;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
