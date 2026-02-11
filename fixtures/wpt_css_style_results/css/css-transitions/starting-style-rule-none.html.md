# css/css-transitions/starting-style-rule-none.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-rule-none.html"
}
```

## style[0]

```css

  #target {
    transition-property: background-color;
    transition-duration: 100s;
    transition-timing-function: steps(2, start);
    background-color: green;
  }
  @starting-style {
    #target {
      display: none;
      background-color: red;
    }
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
