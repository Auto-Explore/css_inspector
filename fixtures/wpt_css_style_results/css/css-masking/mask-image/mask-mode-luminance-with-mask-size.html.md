# css/css-masking/mask-image/mask-mode-luminance-with-mask-size.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-mode-luminance-with-mask-size.html"
}
```

## style[0]

```css

      div {
        background-color: blue;
        position: absolute;

        width: 100px;
        height: 100px;

        top: 10px;
        left: 10px;

        mask-mode: luminance;
        mask-size: 100px 50px;
        mask-image: url(support/transparent-100x50-blue-100x50.png);
      }

    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
