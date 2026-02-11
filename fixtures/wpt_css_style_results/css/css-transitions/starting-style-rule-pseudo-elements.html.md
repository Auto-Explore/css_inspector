# css/css-transitions/starting-style-rule-pseudo-elements.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-rule-pseudo-elements.html"
}
```

## style[0]

```css

  #target::before {
    transition-property: background-color, color;
    transition-duration: 100s;
    transition-timing-function: steps(2, start);
    color: green;
    background-color: white;
    content: "";
  }
  @starting-style {
    #target::before {
      background-color: black;
    }
  }
  #target.red::before {
    background-color: red;
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
