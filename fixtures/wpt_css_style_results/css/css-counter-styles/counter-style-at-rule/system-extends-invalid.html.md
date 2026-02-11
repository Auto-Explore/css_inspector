# css/css-counter-styles/counter-style-at-rule/system-extends-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-extends-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends b;
    prefix: a;
  }
  @counter-style b {
    system: extends c;
    suffix: b;
  }
  @counter-style c {
    system: extends b;
    pad: 2 c;
  }
  @counter-style d {
    system: extends d;
    prefix: d;
  }
  @counter-style e {
    system: extends unknown;
    prefix: e;
  }
  @counter-style f {
    system: extends decimal;
    symbols: a;
  }
  @counter-style g {
    system: extends decimal;
    additive-symbols: 1 a;
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
