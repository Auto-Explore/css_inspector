# css/css-pseudo/highlight-cascade/highlight-currentcolor-computed-visited.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-currentcolor-computed-visited.html"
}
```

## style[0]

```css

  a {
    color: lime;
  }
  a:visited {
    color: cyan;
  }
  a:visited::selection {
    color: currentcolor;
  }
  a:visited::target-text {
    color: currentcolor;
  }
  a:visited::search-text {
    color: currentcolor;
  }
  a:visited::search-text:current {
    color: red;
  }
  a:visited::spelling-error {
    color: currentcolor;
  }
  a:visited::grammar-error {
    color: currentcolor;
  }
  a:visited::highlight(foo) {
    color: currentcolor;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
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
