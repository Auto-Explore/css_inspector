# css/css-values/calc-transform-origin-1.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-transform-origin-1.html"
}
```

## style[0]

```css


body { margin: 100px }

p {
    height: 50px; width: 200px;
    background: yellow;
    transform: rotate(15deg);
}

#one { transform-origin: calc(50px + 50%) calc(100% - 30px); }
#two { transform-origin: calc(-12.5% + 3px) calc(-10px - 50%); }

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
