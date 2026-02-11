# css/css-view-transitions/update-callback-called-once.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/update-callback-called-once.html"
}
```

## style[0]

```css

    #target {
        width: 100px;
        height: 100px;
        background-color: red;
    }

    ::view-transition-group(*),
    ::view-transition-image-pair(*),
    ::view-transition-old(*),
    ::view-transition-new(*) {
      animation-play-state: paused;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
