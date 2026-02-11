# css/css-masking/clip-path/clip-path-circle-closest-corner.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-circle-closest-corner.html"
}
```

## style[0]

```css

    body {
        padding: 0;
        margin: 0;
    }

    .test {
        position: absolute;
        left: 150px;
        top: 100px;
        width: 350px;
        height: 450px;
        background-color: red;
        clip-path: circle(closest-corner at 150px 200px);
    }

    .inner {
        position: absolute;
        top: -60px;
        left: -110px;
        width: 520px;
        height: 520px;
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
