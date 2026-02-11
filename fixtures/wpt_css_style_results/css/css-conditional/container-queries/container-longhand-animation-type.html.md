# css/css-conditional/container-queries/container-longhand-animation-type.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-longhand-animation-type.html"
}
```

## style[0]

```css

  @keyframes anim {
    from {
      --ref:PASS;
      container-name: FAIL;
      container-type: size;
    }
    to {
      --ref:PASS;
      container-name: FAIL;
      container-type: size;
    }
  }
  #container {
    --ref:FAIL;
    container-name: PASS;
    container-type: inline-size;
    animation: anim 1s linear paused;
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
