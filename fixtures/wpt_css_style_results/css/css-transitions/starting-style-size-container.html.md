# css/css-transitions/starting-style-size-container.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-size-container.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    width: 100px;
  }
  #target {
    transition-property: background-color;
    transition-duration: 100s;
    transition-timing-function: steps(2, start);
    background-color: lime;
    display: none;
  }
  @container (width > 300px) {
    @starting-style {
      #target { background-color: white; }
    }
  }
  @container ((width > 200px) and (width < 300px)) {
    #target {
      display: block;
    }
    @starting-style {
      #target { background-color: white; }
    }
  }
  @container (width < 200px) {
    @starting-style {
      #target { background-color: red; }
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
