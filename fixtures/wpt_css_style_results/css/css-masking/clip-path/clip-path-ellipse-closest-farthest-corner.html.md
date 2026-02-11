# css/css-masking/clip-path/clip-path-ellipse-closest-farthest-corner.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-ellipse-closest-farthest-corner.html"
}
```

## style[0]

```css

        .test {
            position: absolute;
            left: 150px;
            top: 100px;
            width: 250px;
            height: 275px;
            background-color: red;
            clip-path: ellipse(farthest-corner closest-corner at 175px 100px);
        }

        .inner {
            position: absolute;
            top: -60px;
            left: -110px;
            width: 550px;
            height: 420px;
            background-color: green;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
