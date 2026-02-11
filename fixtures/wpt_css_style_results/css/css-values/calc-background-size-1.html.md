# css/css-values/calc-background-size-1.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-background-size-1.html"
}
```

## style[0]

```css


p {
    height: 50px; width: 200px;
    border: thin solid;
    background-image: url(support/blue-32x32.png);
    background-repeat: no-repeat;
}

#one { background-size: calc(50px + 50%) calc(100% - 30px) }
#two { background-size: calc(50px + 50%) calc(10px - 50%) }

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
