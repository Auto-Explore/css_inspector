# css/css-animations/animation-name-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-name-001-manual.html"
}
```

## style[0]

```css

  div {
    animation-name: ;
    animation-duration: 8s;

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
      "message": "Missing value for property “animation-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
