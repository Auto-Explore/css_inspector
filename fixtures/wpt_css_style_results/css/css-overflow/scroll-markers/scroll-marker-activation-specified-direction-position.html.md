# css/css-overflow/scroll-markers/scroll-marker-activation-specified-direction-position.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-activation-specified-direction-position.html"
}
```

## style[0]

```css

  .toc {
    overflow: auto;
    overscroll-behavior: contain;
    height: 150px;
    scroll-marker-group: before links;
    counter-reset: --section;
    padding-left: 200px;

    &::scroll-marker-group {
      position: absolute;
      width: 200px;
    }

    & div {
      counter-increment: --section;
      scroll-snap-align: end;

      &::scroll-marker {
        font-size: 1rem;
        display: block;
        content: counter(--section) ". " attr(data-label);
      }

      &::scroll-marker:target-current {
        color: ActiveText;
      }
    }
  }
```

```json
{
  "errors": 5,
  "messages": [
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
      "message": "Invalid value for property “color”.",
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
