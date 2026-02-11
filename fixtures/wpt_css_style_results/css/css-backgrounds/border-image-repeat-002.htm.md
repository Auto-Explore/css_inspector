# css/css-backgrounds/border-image-repeat-002.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-repeat-002.htm"
}
```

## style[0]

```css

            div
            {
                border: 40px double red;
                border-image-repeat: round;
                border-image-slice: 27;
                border-image-source: url("support/blue-and-red-diamonds-81x81.png");
                height: 102px;
                margin: 70px;
                width: 102px;
            }

  /*
    102px divided by 40px == 2.55
    Rounding of 2.55 == 3
    So, there must be at least 3 blue squares
    on each horizontal side and on each vertical side.
    102px divided by 3 == 34px
    The 6 blue squares at top and bottom edges
    will each be 34px wide by 40px tall.
    The remaining 6 blue squares at left and right edges
    will each be 40px wide by 34px tall.
  */

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
