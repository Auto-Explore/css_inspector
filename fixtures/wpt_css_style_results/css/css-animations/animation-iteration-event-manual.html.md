# css/css-animations/animation-iteration-event-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-iteration-event-manual.html"
}
```

## style[0]

```css

  div {
    animation-name: sample;
    animation-duration: 10s;
    animation-iteration-count: 3;

    color: yellow;
    font-weight: bolder;
    font-size: xx-large;
    text-align: center;

    background-color: blue;
    height: 100px;
    width: 100px;
    position: relative;
  }

  @keyframes sample {
    from {
      left: 150px;
    }
    to {
      left: 0px;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
