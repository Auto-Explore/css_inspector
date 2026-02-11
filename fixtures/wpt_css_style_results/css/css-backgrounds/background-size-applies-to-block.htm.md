# css/css-backgrounds/background-size-applies-to-block.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size-applies-to-block.htm"
}
```

## style[0]

```css

            span
            {
                display: block;
                height: 2in;
                width: 2in;
                border: 20px solid black;
                padding: 20px;
                background: url("support/black_color.png") no-repeat red;
                background-size: 100% 100%;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
