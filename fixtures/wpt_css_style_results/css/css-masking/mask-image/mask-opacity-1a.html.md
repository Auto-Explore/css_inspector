# css/css-masking/mask-image/mask-opacity-1a.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-opacity-1a.html"
}
```

## style[0]

```css

      div {
        position: absolute;
        left: 10px;
        top: 10px;
        background-color: rgb(255,255,0);
        width: 100px;
        height: 100px;
        filter: invert(100%);
        mask-image: url(support/blue-100x50-transparent-100x50.png);
        opacity: 0.5;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
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
