# css/css-backgrounds/background_color_padding_box.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background_color_padding_box.htm"
}
```

## style[0]

```css

            div div
            {
                width: 250px;
                height: 250px;
                border: 10px dashed purple;
                padding: 10px;
                background-image: url("support/blue_color.png"), url("support/orange_color.png"), url("support/white_color.png");
                background-repeat: no-repeat;
                background-clip: border-box, border-box, padding-box;
                background-position: 30px 30px, 60px 60px, 90px 90px;
                background-color: black;
            }
            #parent
            {
                width: 290px;
                background: green;
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
