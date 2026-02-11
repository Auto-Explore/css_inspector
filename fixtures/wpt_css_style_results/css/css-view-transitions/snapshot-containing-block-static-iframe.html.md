# css/css-view-transitions/snapshot-containing-block-static-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/snapshot-containing-block-static-iframe.html"
}
```

## style[0]

```css

#inner {
  width: 400px;
  height: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  body {
    height: 200vh;
    overflow: hidden;
  }
  ::view-transition {
    position: static;
    display: block;
    width: 100%;
    height: 100%;
    background: limegreen;
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
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
