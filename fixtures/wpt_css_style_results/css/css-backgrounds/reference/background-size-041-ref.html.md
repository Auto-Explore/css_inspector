# css/css-backgrounds/reference/background-size-041-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/background-size-041-ref.html"
}
```

## style[0]

```css

  div#black-walls
    {
      border: solid 40px;
      border-color: transparent black;
      height: 400px;
      width: 400px;
    }

  div#gradient
    {
      background-image: linear-gradient(orange, blue);
      height: 100%; /* computes to 400px */
      margin: 0px auto;
      /* the gradient block is horizontally centered inside div#black-walls */
      width: 200px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
