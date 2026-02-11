# css/css-backgrounds/order-of-images.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/order-of-images.htm"
}
```

## style[0]

```css

            div
            {
                width: 250px;
                height: 250px;
                background-image: url("support/blue_color.png"), url("support/orange_color.png"), url("support/black_color.png");
                background-repeat: no-repeat;
                background-position: 30px 30px, 60px 60px, 90px 90px;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
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
