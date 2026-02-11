# css/css-conditional/container-queries/pseudo-elements-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-002.html"
}
```

## style[0]

```css

  .container { container-type: inline-size; }

  @container (max-width: 100px) { #c1::before { content: "PASS" } }
  @container (min-width: 150px) { #c1::before { content: "FAIL" } }

  @container (max-width: 100px) { #c2::before { content: "PASS" } }
  @container (min-width: 150px) { #c2::before { content: "FAIL" } }

  @container (max-width: 100px) { #c3::after { content: "PASS" } }
  @container (min-width: 150px) { #c3::after { content: "FAIL" } }

  @container (max-width: 100px) { #c4::after { content: "PASS" } }
  @container (min-width: 150px) { #c4::after { content: "FAIL" } }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
