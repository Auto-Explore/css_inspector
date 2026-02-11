# css/css-pseudo/first-line-with-before-after.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-line-with-before-after.html"
}
```

## style[0]

```css

  #target::first-line {
    font-size: 150%;
  }
  #target::before, #child2::before {
    content: "red ";
    color: red;
  }
  #child1, #child2 {
    color: green;
  }
  #child1::after, #target::after {
    content: " blue";
    color: blue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
