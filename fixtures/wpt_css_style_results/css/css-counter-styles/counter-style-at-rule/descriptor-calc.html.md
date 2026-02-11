# css/css-counter-styles/counter-style-at-rule/descriptor-calc.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-calc.html"
}
```

## style[0]

```css

    /* 100em - 1px should be positive with pretty much any initial font size. */
    @counter-style a {
      system: extends upper-roman;
      range: calc(2 - sign(100em - 1px)) calc(5 + sign(100em - 1px));
      pad: calc(3 + sign(100em - 1px)) '*';
    }
    @counter-style b {
      system: fixed calc(1 + sign(100em - 1px));
      symbols: g h;
    }
    @counter-style c {
      system: additive;
      additive-symbols: calc(2 + sign(100em - 1px)) c, 2 b, 1 a;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
