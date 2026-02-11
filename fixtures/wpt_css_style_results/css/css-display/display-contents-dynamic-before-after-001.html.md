# css/css-display/display-contents-dynamic-before-after-001.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-dynamic-before-after-001.html"
}
```

## style[0]

```css

    div { display: contents; color: green; }
    .p::before { content: "P" }
    .a::before { content: "A" }
    .s::after { content: "S" }
    div::before { color: red; display: contents; border: 1px solid red; }
    .active div::before { color: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
