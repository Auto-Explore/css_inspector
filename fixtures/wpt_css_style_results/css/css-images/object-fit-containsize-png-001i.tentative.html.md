# css/css-images/object-fit-containsize-png-001i.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-images/object-fit-containsize-png-001i.tentative.html"
}
```

## style[0]

```css

      img {
        border: 1px dashed gray;
        padding: 1px;
        image-rendering: pixelated;
        float: left;
        contain: size;
        object-position: top left;
      }
      br {
        clear: both;
      }
      .big {
        width: 32px;
        height: 48px;
      }
      .small {
        width: 8px;
        height: 8px;
      }

      .cover { object-fit: cover }
      .contain { object-fit: contain }
      .fill { object-fit: fill }
      .none { object-fit: none }
      .scaledown { object-fit: scale-down }

    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
