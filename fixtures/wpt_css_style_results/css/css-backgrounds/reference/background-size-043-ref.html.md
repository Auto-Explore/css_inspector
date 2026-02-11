# css/css-backgrounds/reference/background-size-043-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/background-size-043-ref.html"
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

  div#inner-orange
    {
      background-color: orange;
      border: blue solid 16px;
      height: 368px; /*  400px - 2 * 16px == 368px  */
      margin: 0px auto;
      /* the orange block is horizontally centered inside div#black-walls */
      width: 168px;  /*  200px - 2 * 16px == 168px  */
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
