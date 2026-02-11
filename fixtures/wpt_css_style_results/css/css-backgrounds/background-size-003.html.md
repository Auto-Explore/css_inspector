# css/css-backgrounds/background-size-003.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size-003.html"
}
```

## style[0]

```css

            .test
            {
                width: 100px;
                height: 100px;
                background-repeat: no-repeat;
                background-image: url("support/red_color.png");
                background-size: 50px auto;
            }
            .reference
            {
                margin-top: -100px;
                background: black;
                width: 50px;
                height: 50px;
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
