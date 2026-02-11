# css/css-conditional/container-queries/pseudo-elements-007.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-007.html"
}
```

## style[0]

```css

  #target { container-type: inline-size; }
  #target::before,
  #target::after,
  #target::marker,
  #target::first-line,
  #target::first-letter,
  #target::backdrop {
    font-size: 0px;
  }
  @container (width >= 300px) {
    #target::before,
    #target::after,
    #target::marker,
    #target::first-line,
    #target::first-letter,
    #target::backdrop {
      font-size: 10cqw;
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
