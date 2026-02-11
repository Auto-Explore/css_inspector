# css/css-conditional/container-queries/pseudo-elements-005.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-005.html"
}
```

## style[0]

```css

  #c1 {
    --theme: green;
  }
  @container style(--theme: green) {
    #c1::before {
      content: "";
      color: green;
      display: block;
      height: 100px;
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

## style[1]

```css

  #c2 {
    --theme: red;
  }
  #c2::before { color: red }
  #c2.green {
    --theme: green;
  }
  @container style(--theme: green) {
    #c2::before {
      content: "";
      color: green;
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
