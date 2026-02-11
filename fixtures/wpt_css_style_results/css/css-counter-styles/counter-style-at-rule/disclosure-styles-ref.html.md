# css/css-counter-styles/counter-style-at-rule/disclosure-styles-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/disclosure-styles-ref.html"
}
```

## style[0]

```css

  @counter-style triangle-right {
    system: cyclic;
    symbols: \25b8;
    suffix: ' ';
  }
  @counter-style triangle-left {
    system: cyclic;
    symbols: \25c2;
    suffix: ' ';
  }
  @counter-style triangle-down {
    system: cyclic;
    symbols: \25be;
    suffix: ' ';
  }
  @counter-style triangle-up {
    system: cyclic;
    symbols: \25b4;
    suffix: ' ';
  }

  ul {
    padding: 0;
    list-style-position: inside;
  }

  .t-u { list-style-type: triangle-up; }
  .t-d { list-style-type: triangle-down; }
  .t-r { list-style-type: triangle-right; }
  .t-l { list-style-type: triangle-left; }
  div { font: 16px system-ui; margin: 1em; border: 1px solid gray; }
  li::marker { font-family: inherit; }
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
