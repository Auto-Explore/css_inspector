# css/css-pseudo/events-on-pseudo-element.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/events-on-pseudo-element.tentative.html"
}
```

## style[0]

```css

  @keyframes slide-in {
    from {
      transform: translateX(100%) scaleX(3);
    }

    to {
      transform: translateX(0) scaleX(1);
    }
  }

  div {
    scroll-marker-group: after links;
    overflow: scroll;
    height: 200px;

    li {
      height: 200px;
      background: purple;
      border: 1px solid black;

      &::scroll-marker {
        content: "S";
        padding: 10px;
        color: white;
        background: red;
        transition-property: transform, background;
        transition-duration: 2s;
        transition-delay: 1s;
        animation-duration: 2s;
        animation-name: slide-in;
      }

      &::scroll-marker:hover {
        background: blue;
        transform: rotate(90deg);
      }
    }
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
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
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
