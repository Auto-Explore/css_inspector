# css/css-counter-styles/counter-style-at-rule/descriptor-pad.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-pad.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends upper-roman;
    range: infinite 5;
    pad: 3 '*';
  }
  @counter-style b {
    system: extends decimal;
    negative: '(' ')';
    pad: 3 '0';
  }
  @counter-style c {
    system: alphabetic;
    symbols: a\0304  a\0301  a\030c  a\0300;
    pad: 3 o;
    suffix: '';
  }
  @counter-style d {
    system: extends decimal;
    pad: '0' 3;
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
