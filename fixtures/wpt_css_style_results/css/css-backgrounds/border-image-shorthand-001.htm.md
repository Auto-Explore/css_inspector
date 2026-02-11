# css/css-backgrounds/border-image-shorthand-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-shorthand-001.htm"
}
```

## style[0]

```css

            div
            {
                border: 10px double red;
                height: 150px;
                margin: 50px 0px 100px 50px;
                width: 150px;
            }

            #test
            {
                border-image: url("support/blue-and-red-diamonds-81x81.png") 30 / 4 / 2 round stretch;
            }

            #reference
            {
                border-image-outset: 2;
                border-image-repeat: round stretch;
                border-image-slice: 30;
                border-image-source: url("support/blue-and-red-diamonds-81x81.png");
                border-image-width: 4;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
