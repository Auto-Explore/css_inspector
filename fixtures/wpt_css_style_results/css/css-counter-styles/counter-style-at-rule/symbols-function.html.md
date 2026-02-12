# css/css-counter-styles/counter-style-at-rule/symbols-function.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/symbols-function.html"
}
```

## style[0]

```css

  .default {
    list-style-type: symbols('*' '\2020' '\2021' '\A7');
  }
  .cyclic {
    list-style-type: symbols(cyclic '*' '\2020' '\2021' '\A7');
  }
  .numeric {
    list-style-type: symbols(numeric '0' '1' '2');
  }
  .alphabetic {
    list-style-type: symbols(alphabetic '\26AA' '\26AB');
  }
  .symbolic {
    list-style-type: symbols(symbolic '*' '\2020' '\2021' '\A7');
  }
  .fixed {
    list-style-type: symbols(fixed '\25F0' '\25F1' '\25F2' '\25F3');
  }
  .counter, .counters {
    list-style-type: none;
    counter-reset: a;
  }
  .counter li, .counters li {
    counter-increment: a;
    padding-right: .5em;
  }
  .counter li::after {
    content: counter(a, symbols('*'));
  }
  .counters .counters li::after {
    content: counters(a, '.', symbols(numeric '0' '1'));
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
