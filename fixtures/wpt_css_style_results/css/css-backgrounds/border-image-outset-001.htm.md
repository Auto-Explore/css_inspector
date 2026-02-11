# css/css-backgrounds/border-image-outset-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-outset-001.htm"
}
```

## style[0]

```css

            #test
            {
                background-color: blue;
                border: 10px double red;
                border-image-outset: 1 6 11 1;
                border-image-slice: 10;
                border-image-source: url("support/green_color.png");
                border-image-width: 1;
                height: 90px;
                margin: 70px;
                width: 40px;
            }
            #reference
            {
                background-color: orange;
                height: 100px;
                margin: 100px 50px;
                width: 50px;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-image-outset”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
