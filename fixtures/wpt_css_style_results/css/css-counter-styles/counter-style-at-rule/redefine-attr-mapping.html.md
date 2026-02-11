# css/css-counter-styles/counter-style-at-rule/redefine-attr-mapping.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/redefine-attr-mapping.html"
}
```

## style[0]

```css

  @counter-style triangle {
    system: cyclic;
    symbols: \2023;
    suffix: "";
  }
  @counter-style circle {
    system: extends triangle;
  }
  @counter-style square {
    system: extends triangle;
  }
  @counter-style lower-roman {
    system: extends hiragana;
  }
  @counter-style upper-roman {
    system: extends katakana;
  }
  @counter-style lower-alpha {
    system: extends hiragana-iroha;
  }
  @counter-style upper-alpha {
    system: extends katakana-iroha;
  }
  ::marker { font-family: inherit; }
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
