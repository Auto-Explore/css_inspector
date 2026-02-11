# css/css-properties-values-api/font-size-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/font-size-animation.html"
}
```

## style[0]

```css

    @keyframes font_size_animation {
        from {
            font-size: 10px;
            width: 10em;
            --length: 10em;
        }
        to {
            font-size: 20px;
            width: 20em;
            --length: 20em;
        }
    }
    #target1 {
        font-size: 1px;
        animation: font_size_animation 10s -5s linear paused;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
