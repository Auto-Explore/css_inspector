# css/css-flexbox/flex-one-sets-flex-basis-to-zero-px.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-one-sets-flex-basis-to-zero-px.html"
}
```

## style[0]

```css

.flexbox > div {
    font: 14px/1 Ahem;
    min-width: 0;
    min-height: 0;
}

.flex-zero {
    flex: 0;
}

.flex-zero-one-zero-percent {
    flex: 0 1 0%;
}

.flex-zero-one-zero-px {
    flex: 0 1 0px;
}

.flex-half {
    flex: 0.5;
}

.flex-half-one-zero-percent {
    flex: 0.5 1 0%;
}

.flex-half-one-zero-px {
    flex: 0.5 1 0px;
}

.flex-one-one-zero-percent {
    flex: 1 1 0%;
}

.flex-one-one-zero-px {
    flex: 1 1 0px;
}

.vertical {
    writing-mode: vertical-rl;
}
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
