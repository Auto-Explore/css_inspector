# css/css-masking/mask-image/mask-position-4d.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-4d.html"
}
```

## style[0]

```css

      div {
        width: 100px;
        height: 100px;
      }

      #outer {
        border: 1px solid black;
      }

      #inner {
        background-color: purple;
        mask: url(support/50x50-opaque-blue.svg) left no-repeat;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “mask”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
