# css/css-display/display-contents-before-after-001.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-before-after-001.html"
}
```

## style[0]

```css

    /* Disable kerning because kerning may differ for different node tree. */
    html { font-kerning: none; font-feature-settings: "kern" off; }
    div { display: contents }
    .p::before { content: "P" }
    .a::before { content: "A" }
    .s::after { content: "S" }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
