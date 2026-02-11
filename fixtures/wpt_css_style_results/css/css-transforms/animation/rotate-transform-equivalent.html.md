# css/css-transforms/animation/rotate-transform-equivalent.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/rotate-transform-equivalent.html"
}
```

## style[0]

```css

  .block {
    border: 2px solid white; /* Avoid anti-aliasing artifacts */
    height: 100px;
    width: 100px;
    position: absolute;
    left: 100px;
    top: 100px;
  }

  .rotation {
    background: red;
  }

  .overlay {
    background: green;
  }

  #rotateAdd {
    rotate: 1 0 0 45deg;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “rotate”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
