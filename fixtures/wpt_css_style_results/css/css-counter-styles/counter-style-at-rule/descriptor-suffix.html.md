# css/css-counter-styles/counter-style-at-rule/descriptor-suffix.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-suffix.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends decimal;
    suffix: ",";
  }
  @counter-style b {
    system: extends decimal;
    suffix: \3001;
  }
  ol {
    list-style-position: inside;
  }
  ol, section, p {
    padding: 0; margin: 0;
    line-height: 150%;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
