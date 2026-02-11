# css/css-conditional/container-queries/pseudo-elements-013.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-013.html"
}
```

## style[0]

```css

  #target { container-type: inline-size; }
  #target::highlight(foo) {
    text-decoration-line: underline;
    text-decoration-thickness: 0px;
  }
  @container (width >= 300px) {
    #target::highlight(foo) {
      text-decoration-line: underline;
      text-decoration-thickness: 10cqw;
    }
    #target::highlight(bar) {
      text-decoration-line: underline;
      text-decoration-thickness: 10cqw;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
