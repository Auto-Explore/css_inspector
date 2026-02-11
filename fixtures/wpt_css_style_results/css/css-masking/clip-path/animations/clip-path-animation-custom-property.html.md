# css/css-masking/clip-path/animations/clip-path-animation-custom-property.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-custom-property.html"
}
```

## style[0]

```css

  * {
    --small: inset(10px 10px);
    --large: inset(30px 30px);
  }

  @property --large {
    syntax: "<basic-shape>";
    inherits: true;
    initial-value: inset(10px 10px);
  }

  @property --small {
    syntax: "<basic-shape>";
    inherits: true;
    initial-value: inset(30px 30px);
  }

  @keyframes clippath {
    0% {
      clip-path: var(--small);
    }

    100% {
      clip-path: var(--large);
    }
  }

  .background {
    width: 200px;
    height: 200px;
    background-color: blue;
    animation: clippath 20s steps(2, jump-both) -10s;
  }

  .background.circularize {
    --small: circle(10% at 50% 50%);
    --large: circle(40% at 50% 50%);
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
