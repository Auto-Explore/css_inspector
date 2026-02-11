# css/css-counter-styles/counter-style-at-rule/descriptor-prefix.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-prefix.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends upper-roman;
    prefix: "Appendix ";
  }
  section {
    counter-reset: p -3;
  }
  p {
    counter-increment: p;
  }
  p::before {
    content: counter(p, a);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
