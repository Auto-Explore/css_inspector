# css/css-backgrounds/background-color-border-box.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-color-border-box.htm"
}
```

## style[0]

```css

            #test
            {
                width: 250px;
                height: 250px;
                border: 10px dashed purple;
                padding: 10px;
                background-image: url("support/blue_color.png"), url("support/orange_color.png"), url("support/green_color.png");
                background-repeat: no-repeat;
                background-clip: padding-box, content-box, border-box;
                background-position: 30px 30px, 60px 60px, 90px 90px;
                background-color: black;
            }
            #parent
            {
                width: 290px;
                background: red;
            }
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-clip”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
