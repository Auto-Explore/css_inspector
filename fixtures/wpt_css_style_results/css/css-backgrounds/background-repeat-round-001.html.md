# css/css-backgrounds/background-repeat-round-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-round-001.html"
}
```

## style[0]

```css

        div {
            background-color: red;
            background-image: url(support/cat.png);  /* 98px wide by 99px tall */
            background-repeat: round;  /* round round */
            height: 220px;
            width: 220px;

            /*
            Background positioning area is 220px wide by 220px tall.
            So, the height of the image is rounded to 110px, [220px / rounded (220px / 99px)]
            and the width of the image is rounded to 110px, [220px / rounded (220px / 98px)].
            */
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
