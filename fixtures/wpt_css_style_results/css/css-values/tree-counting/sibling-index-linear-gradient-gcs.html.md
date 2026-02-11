# css/css-values/tree-counting/sibling-index-linear-gradient-gcs.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-linear-gradient-gcs.html"
}
```

## style[0]

```css

  #t1 {
    background-image: linear-gradient(white, rgb(0 0 calc(sibling-index() * 200)));
  }
  #t2 {
    background-image: cross-fade(linear-gradient(white, rgb(0 0 calc(sibling-index() * 200))) 40%, rgb(0 0 calc(sibling-index() * 200)) 60%);
  }
  #t3 {
    background-image: image-set(linear-gradient(white, rgb(0 0 calc(sibling-index() * 200))) 1x);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
