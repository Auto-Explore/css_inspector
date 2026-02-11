# css/css-counter-styles/counter-style-at-rule/name-case-sensitivity-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/name-case-sensitivity-ref.html"
}
```

## style[0]

```css

  ol, div, p {
    padding: 0; margin: 0;
    line-height: 150%;
  }
  ol {
    list-style-position: inside;
  }
  li, div, p {
    float: left;
  }
  @counter-style decimal-leading-zero {
    system: extends decimal;
    pad: 3 '0';
  }
  @counter-style custom-style {
    system: cyclic;
    symbols: \2023;
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

  #counter { counter-reset: a 1; }
  #counter-a::before { content: counter(a, hiragana); }
  #counter-b::before { content: counter(a, decimal-leading-zero); }
  #counter-c::before { content: counter(a, custom-style); }
  #counter-d::before { content: counter(a, decimal); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

  #counters { counter-reset: a 1; }
  #counters-a::before { content: counters(a, '', hiragana); }
  #counters-b::before { content: counters(a, '', decimal-leading-zero); }
  #counters-c::before { content: counters(a, '', custom-style); }
  #counters-d::before { content: counters(a, '', decimal); }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

  @counter-style a { system: extends hiragana; }
  @counter-style b { system: extends decimal-leading-zero; }
  @counter-style c { system: extends custom-style; }
  @counter-style d { system: extends decimal; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
