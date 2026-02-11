# css/css-backgrounds/background-size-025.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size-025.html"
}
```

## style[0]

```css

        div {
            background-color: red;
            background-image: url("support/100x100-blue-and-orange.png");
            background-repeat: round;  /* round round */
            background-size: auto 61px;
            height: 210px;
            width: 210px;

            /*
            Background positioning area is 210px wide by 210px tall.
            The set background size height is 61px. But because
            background repeat is round, then the background image
            is rescaled as follows:
            Newest height = 210px / (round [210px / 61px]);
            Newest height = 210px / (round [3.44]);
            Newest height = 210px / (3);
            Newest height = 70px;

            The width is rescaled to 70px to keep the original aspect ratio.
            */
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
