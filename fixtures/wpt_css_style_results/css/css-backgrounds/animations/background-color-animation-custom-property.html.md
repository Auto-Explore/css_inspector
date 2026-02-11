# css/css-backgrounds/animations/background-color-animation-custom-property.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-custom-property.html"
}
```

## style[0]

```css

  @property --dark {
    syntax: "<color>";
    inherits: true;
    initial-value: #000;
  }

  @property --light {
    syntax: "<color>";
    inherits: true;
    initial-value: #fff;
  }

  @keyframes bgcolor {
    0% {
      background-color: var(--dark);
    }

    100% {
      background-color: var(--light);
    }
  }

  .background {
    width: 200px;
    height: 200px;
    animation: bgcolor 20s steps(2, jump-both) -10s;
  }

  .background.colorize {
    --dark: blue;
    --light: green;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
