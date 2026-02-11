# css/css-backgrounds/background_properties_greater_than_images.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background_properties_greater_than_images.htm"
}
```

## style[0]

```css

            div
            {
                margin: 10px;
                width: 250px;
                height: 250px;
                border: thick solid black;
            }
            #test
            {
                background-image: url("support/aqua_color.png"), url("support/orange_color.png"), url("support/green_color.png");
                background-position: right bottom, right top, left bottom, left center, right center;
                background-repeat: no-repeat, no-repeat, repeat-x, repeat, repeat-y;
            }
            #reference
            {
                background-image: url("support/aqua_color.png"), url("support/orange_color.png"), url("support/green_color.png");
                background-position: right bottom, right top, left bottom;
                background-repeat: no-repeat, no-repeat, repeat-x;
            }
        
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
