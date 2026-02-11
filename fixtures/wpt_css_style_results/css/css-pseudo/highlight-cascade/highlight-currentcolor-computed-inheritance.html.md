# css/css-pseudo/highlight-cascade/highlight-currentcolor-computed-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-currentcolor-computed-inheritance.html"
}
```

## style[0]

```css

  #target {
    background-color: green;
    color: lime;
  }
  #target::selection {
    color: currentcolor;
    background-color: currentcolor;
  }
  #target::target-text {
    color: currentcolor;
    background-color: currentcolor;
  }
  #target::search-text {
    color: currentcolor;
    background-color: currentcolor;
  }
  #target::search-text:current {
    color: red;
    background-color: red;
  }
  #target::spelling-error {
    color: currentcolor;
    background-color: currentcolor;
  }
  #target::grammar-error {
    color: currentcolor;
    background-color: currentcolor;
  }
  #target::highlight(foo) {
    color: currentcolor;
    background-color: currentcolor;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
