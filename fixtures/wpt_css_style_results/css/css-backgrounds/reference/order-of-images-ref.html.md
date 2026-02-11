# css/css-backgrounds/reference/order-of-images-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/order-of-images-ref.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      position: relative;
      width: 100px;
    }

  div#outer-black
    {
      background-color: black;
      left: 90px;
      top: 90px;
    }

  div#middle-orange
    {
      background-color: orange;
      bottom: 30px;
      right: 30px;
    }

  div#inner-blue
    {
      background-color: blue;
      bottom: 30px;
      right: 30px;
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
