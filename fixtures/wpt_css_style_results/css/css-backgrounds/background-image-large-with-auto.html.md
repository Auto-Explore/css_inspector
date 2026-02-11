# css/css-backgrounds/background-image-large-with-auto.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-image-large-with-auto.html"
}
```

## style[0]

```css

    .wide-div {
      background-image: url(support/green-1000x10.png);
      background-repeat: no-repeat;
      background-size: 10000px auto;
      width: 1000px;
      height: 100px;
      border: 1px solid black;
    }

    .high-div {
      background-image: url(support/green-10x1000.png);
      background-repeat: no-repeat;
      background-size: auto 10000px;
      width: 100px;
      height: 1000px;
      border: 1px solid black;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
