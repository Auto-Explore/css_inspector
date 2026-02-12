# css/css-backgrounds/background-size-029.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size-029.html"
}
```

## style[0]

```css

        div {
            background-color: red;
            background-image: url(support/100x100-blue-and-orange.png);
            background-repeat: round repeat;
            background-size: 52px auto;
            height: 180px;
            width: 180px;

            /*
            Background positioning area is 180px wide by 180px tall.
            The width of the background image is 52px.
            But, because background-repeat is set to round repeat, the
            width is rescaled as following:
            Newest width = 180px / (round [180px / 52px]);
            Newest width = 180px / (round [3.46]);
            Newest width = 180px / (3);
            Newest width = 60px;

            Then the height is rescaled to from 100px to 60px to keep
            the original aspect ratio.
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
