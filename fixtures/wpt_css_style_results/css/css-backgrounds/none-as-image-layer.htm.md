# css/css-backgrounds/none-as-image-layer.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/none-as-image-layer.htm"
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
                background-image: url("support/blue_color.png"), none, url("support/green_color.png");
                background-repeat: no-repeat, repeat, no-repeat;
                background-position: 30px 30px, 60px 60px, 90px 90px;
            }
            #reference
            {
                background-image: url("support/blue_color.png"), url("support/green_color.png");
                background-repeat: no-repeat, no-repeat;
                background-position: 30px 30px, 90px 90px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
