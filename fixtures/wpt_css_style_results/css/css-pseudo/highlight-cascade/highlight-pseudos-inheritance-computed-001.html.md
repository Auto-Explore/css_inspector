# css/css-pseudo/highlight-cascade/highlight-pseudos-inheritance-computed-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-pseudos-inheritance-computed-001.html"
}
```

## style[0]

```css

  .target::selection {
    background-color: green;
    color: lime;
  }
  .target::target-text {
    background-color: green;
    color: lime;
  }
  .target::search-text {
    background-color: green;
    color: lime;
  }
  .target::search-text:current {
    background-color: red;
    color: red;
  }
  .target::spelling-error {
    background-color: green;
    color: lime;
  }
  .target::grammar-error {
    background-color: green;
    color: lime;
  }
  .target::highlight(foo) {
    background-color: green;
    color: lime;
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
