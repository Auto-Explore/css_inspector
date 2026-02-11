# css/css-pseudo/svg-text-selection-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/svg-text-selection-shadow.html"
}
```

## style[0]

```css

  div {
    font: 16px Ahem;
  }
  text {
    text-shadow: 5px 5px 0px green;
  }
  ::selection {
    background-color: transparent;
    text-shadow: 3px 3px 0px blue;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
