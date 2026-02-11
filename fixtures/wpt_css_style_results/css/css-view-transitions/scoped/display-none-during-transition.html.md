# css/css-view-transitions/scoped/display-none-during-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/display-none-during-transition.html"
}
```

## style[0]

```css

  #target {
    background-color: teal;
    height: 100px;
    width: 100px;
    position: relative;
    view-transition-name: target;
  }

  .hidden {
    display: none;
  }

  ::view-transition-group(*) {
    animation: unset;
  }

  ::view-transition-old(target) {
    animation: -ua-view-transition-fade-out 300s;
  }

  ::view-transition-new(target) {
    animation: -ua-view-transition-fade-in 300s;
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
