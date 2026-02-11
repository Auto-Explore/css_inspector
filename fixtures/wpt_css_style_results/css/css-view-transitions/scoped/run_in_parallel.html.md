# css/css-view-transitions/scoped/run_in_parallel.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/run_in_parallel.html"
}
```

## style[0]

```css

  .block {
    background-color: blue;
    position: relative;
    height: 100px;
    width: 100px;
    margin: 50px;
  }

  #target1 {
    view-transition-name: a;
  }

  #target2 {
    view-transition-name: b;
  }

  ::view-transition-group(*),
  ::view-transition-image-pair(*),
  ::view-transition-old(*) {
    animation: unset;
  }

  ::view-transition-old(*) {
    opacity: 0;
  }

  @keyframes stylize {
    from {
      opacity: 0.5;
    }
    to {
      opacity: 1.0;
    }
  }
  ::view-transition-new(*) {
    animation: stylize 1s paused;
  }

```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
