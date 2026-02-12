# css/css-pseudo/highlight-cascade/highlight-pseudos-visited-computed-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-pseudos-visited-computed-001.html"
}
```

## style[0]

```css

  a::selection {
    color: lime;
  }
  a::target-text {
    color: lime;
  }
  a::search-text {
    color: lime;
  }
  a::search-text:current {
    color: red;
  }
  a::spelling-error {
    color: lime;
  }
  a::grammar-error {
    color: lime;
  }
  a::highlight(foo) {
    color: lime;
  }
  a:visited::selection {
    color: yellow;
  }
  a:visited::target-text {
    color: yellow;
  }
  a:visited::search-text {
    color: yellow;
  }
  a:visited::search-text:current {
    color: red;
  }
  a:visited::spelling-error {
    color: yellow;
  }
  a:visited::grammar-error {
    color: yellow;
  }
  a:visited::highlight(foo) {
    color: yellow;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
