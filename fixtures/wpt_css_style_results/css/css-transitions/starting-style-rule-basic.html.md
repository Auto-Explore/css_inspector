# css/css-transitions/starting-style-rule-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-rule-basic.html"
}
```

## style[0]

```css

  #target {
    transition-property: background-color, color;
    transition-duration: 100s;
    transition-timing-function: steps(2, start);
    color: green;
    background-color: white;
  }
  @starting-style {
    #target {
      background-color: black;
    }
  }
  #target.red {
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
